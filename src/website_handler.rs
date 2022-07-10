use crate::{
    http::{Request, Response, StatusCode},
    server::Handler,
};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request {
            Request {
                method: _,
                path: "/",
                query_string: _,
            } => Response::new(StatusCode::Ok, Some("<h1>Home</h1>".to_string())),
            Request {
                method: _,
                path: "/about",
                query_string: _,
            } => Response::new(StatusCode::Ok, Some("<h1>About</h1>".to_string())),
            Request {
                method: _,
                path: "/contact",
                query_string: _,
            } => Response::new(StatusCode::Ok, Some("<h1>Contact</h1>".to_string())),
            Request {
                method: _,
                path: "/blog",
                query_string: _,
            } => Response::new(StatusCode::Ok, Some("<h1>Blog</h1>".to_string())),
            Request {
                method: _,
                path: "/blog/post-1",
                query_string: _,
            } => Response::new(StatusCode::Ok, Some("<h1>Post 1</h1>".to_string())),
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
