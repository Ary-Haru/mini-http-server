mod server;
mod router;
mod thread_pool;
mod http;

use crate::server::HttpServer;
use crate::router::Router;

fn main() {
    // if logs dont show up, it aint the servers fault Its yours
    env_logger::init();

    let mut router = Router::new();

    // ye, "/" exists, revolutionary
    router.get("/", |_req| {
        http::response::Response::json(200, serde_json::json!({
            "message": "Rust HTTP server alive and kicking"
        }))
    });

    // health checks cuz kubernetes gets anxious
    router.get("/health", |_req| {
        http::response::Response::text(200, "OK")
    });

    // echo endpoint, what u send is what u get
    router.post("/echo", |req| {
        http::response::Response::json(
            200,
            req.body.unwrap_or_else(|| serde_json::json!({}))
        )
    });

    let server = HttpServer::new("127.0.0.1:8080", router);
    server.start();
}
