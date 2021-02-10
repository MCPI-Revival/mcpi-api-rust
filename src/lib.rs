use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{Result as IoResult, Error, BufReader};

#[cfg(test)]
mod tests {
    use crate::{create, Vec3};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let mut mc = create("localhost:4711");
        let position = Vec3 {
            x: -2,
            y: 8,
            z: -2
        };
        mc.post_to_chat("Test");
        println!(mc.get_block(position));
    }
}

pub struct Minecraft {
    conn:Connection
}

struct Connection {
    stream:TcpStream
}

pub struct Vec3 {
    x:i32,
    y:i32,
    z:i32
}

impl Connection {
    pub fn send(&mut self, msg:&str) {
        self.stream.write(&format!("{}\n", msg).as_bytes());
    }

    pub fn receive(&mut self) -> &str {
        let mut reader = BufReader::new(&self.stream);
        let mut line = String::new();
        reader.read_line(&mut line);
        line.as_str()
    }

    pub fn send_receive(&mut self, msg:&str) -> &str {
        self.send(msg);
        self.receive()
    }
}

impl Minecraft {
    pub fn post_to_chat(&mut self, msg:&str) {
        self.conn.send(&format!("chat.post({})", msg));
    }

    pub fn get_block(&mut self, pos:Vec3) -> &str{
        self.conn.send_receive(&format!("world.getBlock({},{},{})", pos.x, pos.y, pos.z))
    }
}

pub fn create(adress:&str) -> Minecraft {
    let mut stream = TcpStream::connect(adress);
    match stream {
        Ok(_) => {}
        Err(_) => {
            panic!("Failed to connect to the API! Is Minecraft running?")
        }
    }
    Minecraft {
        conn: Connection {
            stream: stream.unwrap()
        }
    }
}