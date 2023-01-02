use std::io;

use log::{debug, error, trace, warn};
use tokio::net::TcpStream;

pub struct Connection {}

impl Connection {
    pub async fn init(socket: TcpStream) {
        let mut buf = vec![0; 4096];

        loop {
            match socket.readable().await {
                Ok(_) => {}
                Err(e) => {
                    error!("error waiting for socket to be readable {:?}", e);
                    return;
                }
            }

            let n = match socket.try_read(&mut buf) {
                Ok(n) if n == 0 => {
                    trace!("socket closed");
                    return;
                }
                Ok(n) => {
                    buf.truncate(n);
                    n
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => {
                    error!("failed to read from socket {:?}", e);
                    return;
                }
            };

            trace!("got {} bytes", n);
            trace!("read {:?}", String::from_utf8_lossy(&buf[..n]).to_string());
        }
    }
}
