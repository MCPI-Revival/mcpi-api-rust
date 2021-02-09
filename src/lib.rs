use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{Result as IoResult, Error};

#[cfg(test)]
mod tests {
    use crate::create;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let mut mc = create("localhost:4711");
        mc.post_to_chat("Test");
    }
}

pub struct Minecraft {
    conn:Connection
}

struct Connection {
    stream:TcpStream
}

struct Vec3 {
    x:i32,
    y:i32,
    z:i32
}

impl Connection {
    pub fn send(&mut self, msg:&str) {
        self.stream.write(&format!("{}\n",msg).as_bytes());
    }
}

impl Minecraft {
    pub fn post_to_chat(&mut self, msg:&str) {
        self.conn.send(&format!("chat.post({})", msg));
    }
}

pub fn create(adress:&str) -> Minecraft {
    let mut stream = TcpStream::connect(adress);
    match stream {
        Ok(_) => {}
        Err(e) => {
            panic!(e)
        }
    }
    Minecraft {
        conn: Connection {
            stream: stream.unwrap()
        }
    }
}