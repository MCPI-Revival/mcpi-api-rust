use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;

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
        println!("{}",mc.get_block(position));
        println!("{}",mc.get_block_with_data(position));
    }
}

pub struct Minecraft {
    conn:Connection
}

struct Connection {
    stream:TcpStream
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    x:i32,
    y:i32,
    z:i32
}

impl Connection {
    pub fn send(&mut self, msg:&str) {
        self.stream.write(&format!("{}\n", msg).as_bytes());
    }

    pub fn receive(&mut self) -> String {
        let mut reader = BufReader::new(&self.stream);
        let mut line = String::new();
        reader.read_line(&mut line);
        line
    }

    pub fn send_receive(&mut self, msg:&str) -> String {
        self.send(msg);
        self.receive()
    }
}

impl Minecraft {
    pub fn post_to_chat(&mut self, msg:&str) {
        self.conn.send(&format!("chat.post({})", msg));
    }

    pub fn get_block(&mut self, pos:Vec3) -> String{
        self.conn.send_receive(&format!("world.getBlock({},{},{})", pos.x, pos.y, pos.z))
    }

    pub fn get_block_with_data(&mut self, pos:Vec3) -> String {
        self.conn.send_receive(&format!("world.getBlockWithData({},{},{})", pos.x, pos.y, pos.z))
    }
}

pub fn create(adress:&str) -> Minecraft {
    let stream = TcpStream::connect(adress);
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