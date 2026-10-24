use std::{
    env,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    time::Duration,
};

use rust_server::{DbPool, NewItem, ThreadPool, create_item, create_pool, get_item, list_items};
use serde::Serialize;

const MAX_BODY_BYTES: usize = 64 * 1024;
const MAX_HEADER_LINES: usize = 64;
const IO_TIMEOUT: Duration = Duration::from_secs(5);

fn env_or<T: std::str::FromStr>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn main() {
    let database_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "items.db".to_owned());
    // Inside a container 127.0.0.1 is unreachable from the published port; default to all interfaces.
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_owned());
    // Handlers block on SQLite I/O, so more workers than cores helps hide that wait,
    // but each one costs memory and context switches on 1 vCPU.
    let workers: usize = env_or("WORKERS", 32);
    let queue: usize = env_or("QUEUE_CAPACITY", 1024);
    let list_limit: usize = env_or("LIST_LIMIT", 1000);

    let pool = create_pool(&database_path).expect("failed to initialize SQLite database");
    let listener = TcpListener::bind(&bind_addr).expect("failed to bind server");
    let thread_pool = ThreadPool::new(workers, queue);

    println!("Listening on http://{bind_addr} (workers={workers}, queue={queue})");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let pool = pool.clone();
                let shed = stream.try_clone();
                if thread_pool
                    .try_execute(move || handle_connection(stream, &pool, list_limit))
                    .is_err()
                {
                    // Queue full: shed load immediately rather than stall every client.
                    if let Ok(mut shed) = shed {
                        let _ = shed.write_all(&json_response(
                            "503 Service Unavailable",
                            &"server busy",
                        ));
                    }
                }
            }
            Err(error) => eprintln!("failed to accept connection: {error}"),
        }
    }
}

fn handle_connection(stream: TcpStream, pool: &DbPool, list_limit: usize) {
    let mut stream = stream;
    // Timeouts stop slow or idle clients from pinning a worker forever.
    let _ = stream.set_read_timeout(Some(IO_TIMEOUT));
    let _ = stream.set_write_timeout(Some(IO_TIMEOUT));
    let _ = stream.set_nodelay(true);

    let response = match read_request(&mut stream).and_then(|request| route(request, pool, list_limit))
    {
        Ok(response) => response,
        Err(ResponseError { status, message }) => json_response(status, &message),
    };

    if let Err(error) = stream.write_all(&response) {
        eprintln!("failed to write response: {error}");
    }
}

struct Request {
    method: String,
    path: String,
    body: Vec<u8>,
}

struct ResponseError {
    status: &'static str,
    message: String,
}

fn read_request(stream: &mut TcpStream) -> Result<Request, ResponseError> {
    let mut reader = BufReader::new(stream);
    let mut headers = Vec::new();
    reader
        .read_until(b'\n', &mut headers)
        .map_err(internal_error)?;
    let request_line =
        String::from_utf8(headers).map_err(|_| bad_request("request line is not valid UTF-8"))?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts
        .next()
        .ok_or_else(|| bad_request("missing HTTP method"))?;
    let path = request_parts
        .next()
        .ok_or_else(|| bad_request("missing request path"))?
        .to_owned();

    let mut content_length = 0;
    let mut line = Vec::with_capacity(128);
    for _ in 0..MAX_HEADER_LINES {
        line.clear();
        let read = reader
            .read_until(b'\n', &mut line)
            .map_err(internal_error)?;
        if read == 0 || line == b"\r\n" || line == b"\n" {
            break;
        }
        // Header names are case-insensitive (k6 and browsers may send lowercase).
        let Some(colon) = line.iter().position(|&b| b == b':') else {
            continue;
        };
        if line[..colon].eq_ignore_ascii_case(b"content-length") {
            content_length = std::str::from_utf8(&line[colon + 1..])
                .ok()
                .and_then(|v| v.trim().parse::<usize>().ok())
                .ok_or_else(|| bad_request("invalid Content-Length"))?;
        }
    }
    if content_length > MAX_BODY_BYTES {
        return Err(ResponseError {
            status: "413 Payload Too Large",
            message: "request body too large".to_owned(),
        });
    }

    let mut body = vec![0; content_length];
    reader.read_exact(&mut body).map_err(internal_error)?;
    Ok(Request {
        method: method.to_owned(),
        path,
        body,
    })
}

fn route(
    request: Request,
    pool: &DbPool,
    list_limit: usize,
) -> Result<Vec<u8>, ResponseError> {
    // Ignore any query string when routing.
    let path = request.path.split('?').next().unwrap_or("");
    match (request.method.as_str(), path) {
        ("POST", "/items/") | ("POST", "/items") => {
            let item = serde_json::from_slice::<NewItem>(&request.body)
                .map_err(|error| bad_request(&format!("invalid JSON body: {error}")))?;
            let item = create_item(pool, item).map_err(database_error)?;
            Ok(json_response("201 Created", &item))
        }
        ("GET", "/items/") | ("GET", "/items") => {
            let items = list_items(pool, list_limit).map_err(database_error)?;
            Ok(json_response("200 OK", &items))
        }
        ("GET", path) if path.starts_with("/items/") => {
            let id = path["/items/".len()..]
                .parse::<i64>()
                .map_err(|_| bad_request("item ID must be an integer"))?;
            match get_item(pool, id).map_err(database_error)? {
                Some(item) => Ok(json_response("200 OK", &item)),
                None => Err(ResponseError {
                    status: "404 Not Found",
                    message: "item not found".to_owned(),
                }),
            }
        }
        _ => Err(ResponseError {
            status: "404 Not Found",
            message: "route not found".to_owned(),
        }),
    }
}

fn json_response<T: Serialize>(status: &'static str, value: &T) -> Vec<u8> {
    let body = serde_json::to_vec(value).expect("API response must be serializable");
    let mut out = Vec::with_capacity(body.len() + 128);
    // Writing to a Vec cannot fail.
    let _ = write!(
        out,
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    out.extend_from_slice(&body);
    out
}

fn bad_request(message: &str) -> ResponseError {
    ResponseError {
        status: "400 Bad Request",
        message: message.to_owned(),
    }
}

fn internal_error(error: std::io::Error) -> ResponseError {
    eprintln!("failed to read request: {error}");
    ResponseError {
        status: "400 Bad Request",
        message: "invalid HTTP request".to_owned(),
    }
}

fn database_error(error: rusqlite::Error) -> ResponseError {
    eprintln!("database error: {error}");
    ResponseError {
        status: "500 Internal Server Error",
        message: "database error".to_owned(),
    }
}
