use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::{Deserialize, Serialize};

pub type DbPool = Pool<SqliteConnectionManager>;

#[derive(Debug, Deserialize)]
pub struct NewItem {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
    pub id: i64,
    pub title: String,
    pub description: String,
}

pub fn create_pool(database_path: &str) -> Result<DbPool, Box<dyn std::error::Error>> {
    let manager = SqliteConnectionManager::file(database_path);
    let mut builder = Pool::builder()
        .connection_customizer(Box::new(SqliteConnectionCustomizer))
        // Fail fast instead of queueing for 30s when the pool is exhausted.
        .connection_timeout(Duration::from_secs(5));
    if database_path == ":memory:" {
        builder = builder.max_size(1);
    } else {
        // SQLite allows one writer at a time; a handful of connections is plenty on 1 vCPU
        // and keeps per-connection page caches within the 2GB budget.
        builder = builder.max_size(8).min_idle(Some(8));
    }
    let pool = builder.build(manager)?;

    let connection = pool.get()?;
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL
        );",
    )?;

    Ok(pool)
}

#[derive(Debug)]
struct SqliteConnectionCustomizer;

impl r2d2::CustomizeConnection<rusqlite::Connection, rusqlite::Error>
    for SqliteConnectionCustomizer
{
    fn on_acquire(&self, connection: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA busy_timeout = 5000;
             PRAGMA temp_store = MEMORY;
             PRAGMA cache_size = -16384;
             PRAGMA mmap_size = 134217728;",
        )
    }

    fn on_release(&self, _connection: rusqlite::Connection) {}
}

pub fn create_item(pool: &DbPool, item: NewItem) -> rusqlite::Result<Item> {
    let connection = pool
        .get()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    connection
        .prepare_cached("INSERT INTO items (title, description) VALUES (?1, ?2)")?
        .execute(params![item.title, item.description])?;
    let id = connection.last_insert_rowid();

    Ok(Item {
        id,
        title: item.title,
        description: item.description,
    })
}

/// Returns at most `limit` items in ascending id order. Unbounded listing grows with every
/// POST and eventually dominates CPU, memory and response time.
pub fn list_items(pool: &DbPool, limit: usize) -> rusqlite::Result<Vec<Item>> {
    let connection = pool
        .get()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let mut statement =
        connection
        .prepare_cached("SELECT id, title, description FROM items ORDER BY id LIMIT ?1")?;
    let items = statement
        .query_map(params![limit as i64], |row| {
            Ok(Item {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(items)
}

pub fn get_item(pool: &DbPool, item_id: i64) -> rusqlite::Result<Option<Item>> {
    let connection = pool
        .get()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let mut statement =
        connection.prepare_cached("SELECT id, title, description FROM items WHERE id = ?1")?;
    let mut rows = statement.query(params![item_id])?;

    rows.next()?.map_or(Ok(None), |row| {
        Ok(Some(Item {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
        }))
    })
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::SyncSender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Returned by [`ThreadPool::try_execute`] when the queue is full.
#[derive(Debug)]
pub struct QueueFull;

impl ThreadPool {
    /// Create a pool of `size` workers with room for `queue_capacity` waiting jobs.
    ///
    /// # Panics
    ///
    /// Panics if `size` is zero.
    pub fn new(size: usize, queue_capacity: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::sync_channel(queue_capacity);
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Queue a job without blocking. Lets the caller shed load (e.g. reply 503)
    /// instead of letting an unbounded backlog grow memory and latency.
    pub fn try_execute<F>(&self, f: F) -> Result<(), QueueFull>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .as_ref()
            .unwrap()
            .try_send(Box::new(f))
            .map_err(|_| QueueFull)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::Builder::new()
            .name(format!("worker-{id}"))
            // Handlers are shallow; small stacks keep memory low.
            .stack_size(256 * 1024)
            .spawn(move || {
                loop {
                    // Hold the lock only while waiting for a job, not while running it.
                    let message = receiver.lock().unwrap_or_else(|e| e.into_inner()).recv();
                    match message {
                        Ok(job) => {
                            // A panicking handler must not permanently kill a worker.
                            let _ = catch_unwind(AssertUnwindSafe(job));
                        }
                        Err(_) => break,
                    }
                }
            })
            .expect("failed to spawn worker thread");

        Worker {
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_lists_and_reads_items() {
        let pool = create_pool(":memory:").unwrap();
        let created = create_item(
            &pool,
            NewItem {
                title: "Title".to_owned(),
                description: "Description".to_owned(),
            },
        )
        .unwrap();

        assert_eq!(created.id, 1);
        assert_eq!(list_items(&pool, 1000).unwrap(), vec![created]);
        assert_eq!(
            get_item(&pool, 1).unwrap(),
            Some(list_items(&pool, 1000).unwrap()[0].clone())
        );
        assert_eq!(get_item(&pool, 999).unwrap(), None);
    }
}
