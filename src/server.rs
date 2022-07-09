use crate::http::{Request, Response, StatusCode};
use std::{
    convert::TryFrom,
    io::{Read, Write},
    net::TcpListener,
};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Accepted connection from {}", addr);
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(n) => {
                            println!("Read {} bytes", n);
                            println!("{}", String::from_utf8_lossy(&buf));
                            let response = match Request::try_from(&buf[..n]) {
                                Ok(request) => {
                                    println!("{:?}", request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some(
                                            "<h1>Hello From Rust! You Animals!!!</h1>".to_string(),
                                        ),
                                    )
                                }
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                    stream.flush().unwrap();
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            };
        }
    }
}
