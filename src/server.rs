use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use log::{error, info};

use crate::http::{request::Request, response::Response};
use crate::router::Router;
use crate::thread_pool::ThreadPool;

pub struct HttpServer {
    addr: String,
    router: Router,
}

impl HttpServer {
    pub fn new(addr: &str, router: Router) -> Self {
        Self {
            addr: addr.to_string(),
            router,
        }
    }

    pub fn start(self) {
        // binding can fail, usually cuz something else is already squatting on the port
        let listener = TcpListener::bind(&self.addr)
            .expect("Failed to bind address. Skill issue or port already in use.");

        let pool = ThreadPool::new(4);

        info!("Server running at http://{}", self.addr);

        // Infinite loop like production bugs.
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router = self.router.clone();
                    pool.execute(|| handle_connection(stream, router));
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, router: Router) {
    let mut buffer = [0u8; 4096];

    // reading raw bytes like its 1999
    match stream.read(&mut buffer) {
        Ok(_) => {
            if let Some(request) = Request::parse(&buffer) {
                let response = router.handle(request);
                let _ = stream.write_all(response.to_string().as_bytes());
            } else {
                // bad request
                let response = Response::text(400, "Bad Request");
                let _ = stream.write_all(response.to_string().as_bytes());
            }
        }
        Err(e) => error!("Failed to read from stream: {}", e),
    }
}
