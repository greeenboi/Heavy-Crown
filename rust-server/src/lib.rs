use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
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
    let mut builder = Pool::builder().connection_customizer(Box::new(SqliteConnectionCustomizer));
    if database_path == ":memory:" {
        builder = builder.max_size(1);
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
             PRAGMA busy_timeout = 5000;",
        )
    }

    fn on_release(&self, _connection: rusqlite::Connection) {}
}

pub fn create_item(pool: &DbPool, item: NewItem) -> rusqlite::Result<Item> {
    let connection = pool
        .get()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    connection.execute(
        "INSERT INTO items (title, description) VALUES (?1, ?2)",
        params![item.title, item.description],
    )?;
    let id = connection.last_insert_rowid();

    Ok(Item {
        id,
        title: item.title,
        description: item.description,
    })
}

pub fn list_items(pool: &DbPool) -> rusqlite::Result<Vec<Item>> {
    let connection = pool
        .get()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let mut statement =
        connection.prepare("SELECT id, title, description FROM items ORDER BY id")?;
    let items = statement
        .query_map([], |row| {
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
        connection.prepare("SELECT id, title, description FROM items WHERE id = ?1")?;
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
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
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
        assert_eq!(list_items(&pool).unwrap(), vec![created]);
        assert_eq!(
            get_item(&pool, 1).unwrap(),
            Some(list_items(&pool).unwrap()[0].clone())
        );
        assert_eq!(get_item(&pool, 999).unwrap(), None);
    }
}
