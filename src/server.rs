use crate::http::Request;
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
                            stream.write(&buf).unwrap();
                            match Request::try_from(&buf[..n]) {
                                Ok(request) => {
                                    println!("{:?}", request);
                                }
                                Err(e) => {
                                    println!("{}", e);
                                }
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
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
