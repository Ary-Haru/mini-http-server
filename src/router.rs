use std::collections::HashMap;
use std::sync::Arc;

use crate::http::{request::Request, response::Response};

// cuz function pointers aint fancy enough
type Handler = Arc<dyn Fn(Request) -> Response + Send + Sync>;

#[derive(Clone)]
pub struct Router {
    routes: HashMap<(String, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.add("GET", path, handler);
    }

    pub fn post<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.add("POST", path, handler);
    }

    fn add<F>(&mut self, method: &str, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        // no regex routing no magic
        // just raw honest matching
        self.routes.insert(
            (method.to_string(), path.to_string()),
            Arc::new(handler),
        );
    }

    pub fn handle(&self, request: Request) -> Response {
        self.routes
            .get(&(request.method.clone(), request.path.clone()))
            .map(|handler| handler(request))
            .unwrap_or_else(|| Response::text(404, "Not Found"))
    }
}

