use std::{
    io::Read,
    net::{TcpListener, TcpStream},
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
                    stream.read(&mut buf).unwrap();
                    println!("{}", String::from_utf8_lossy(&buf));
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            };
        }
    }
}
