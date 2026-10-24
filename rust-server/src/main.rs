use std::{
    env,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use rust_server::{DbPool, NewItem, ThreadPool, create_item, create_pool, get_item, list_items};
use serde::Serialize;

fn main() {
    let database_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "items.db".to_owned());
    let pool = create_pool(&database_path).expect("failed to initialize SQLite database");
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to bind server");
    let thread_pool = ThreadPool::new(4);

    println!("Listening on http://localhost:8000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let pool = pool.clone();
                thread_pool.execute(move || handle_connection(stream, &pool));
            }
            Err(error) => eprintln!("failed to accept connection: {error}"),
        }
    }
}

fn handle_connection(stream: TcpStream, pool: &DbPool) {
    let mut stream = stream;
    let response = match read_request(&mut stream).and_then(|request| route(request, pool)) {
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
    loop {
        let mut line = Vec::new();
        reader
            .read_until(b'\n', &mut line)
            .map_err(internal_error)?;
        if line == b"\r\n" || line == b"\n" {
            break;
        }
        let line = String::from_utf8(line)
            .map_err(|_| bad_request("request headers are not valid UTF-8"))?;
        if let Some(value) = line.strip_prefix("Content-Length:") {
            content_length = value
                .trim()
                .parse::<usize>()
                .map_err(|_| bad_request("invalid Content-Length"))?;
        }
    }

    let mut body = vec![0; content_length];
    reader.read_exact(&mut body).map_err(internal_error)?;
    Ok(Request {
        method: method.to_owned(),
        path,
        body,
    })
}

fn route(request: Request, pool: &DbPool) -> Result<Vec<u8>, ResponseError> {
    match (request.method.as_str(), request.path.as_str()) {
        ("POST", "/items/") | ("POST", "/items") => {
            let item = serde_json::from_slice::<NewItem>(&request.body)
                .map_err(|error| bad_request(&format!("invalid JSON body: {error}")))?;
            let item = create_item(pool, item).map_err(database_error)?;
            Ok(json_response("201 Created", &item))
        }
        ("GET", "/items/") | ("GET", "/items") => {
            let items = list_items(pool).map_err(database_error)?;
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
    format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes()
    .into_iter()
    .chain(body)
    .collect()
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
