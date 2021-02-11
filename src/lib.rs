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
        let position1 = Vec3 {
            x: 25,
            y: 0,
            z: 5
        };
        let position2 = Vec3 {
            x: 30,
            y: 5,
            z: 10
        };
        mc.post_to_chat("Test");
        println!("{}",mc.get_block(position1));
        println!("{:?}",mc.get_block_with_data(position1));
        mc.set_block(position1,18,1);
        mc.set_blocks(position1,position2,18,1);
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
    pub x:i32,
    pub y:i32,
    pub z:i32
}

impl Connection {
    pub fn send(&mut self, msg:&str) {
        self.stream.write(&format!("{}\n", msg).as_bytes());
    }

    pub fn receive(&mut self) -> String {
        let mut reader = BufReader::new(&self.stream);
        let mut line = String::new();
        reader.read_line(&mut line);
        line.replace('\n',"")
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

    pub fn get_block(&mut self, pos:Vec3) -> u8 {
        self.conn.send_receive(&format!("world.getBlock({},{},{})", pos.x, pos.y, pos.z)).parse::<u8>().unwrap()
    }

    pub fn get_block_with_data(&mut self, pos:Vec3) -> Vec<u8> {
        self.conn.send_receive(&format!("world.getBlockWithData({},{},{})", pos.x, pos.y, pos.z)).split(',').map(|s| s.parse()).collect::<Result<Vec<u8>, _>>().unwrap()
    }

    pub fn set_block(&mut self, pos:Vec3, blocktype:u8, blockdata:u8) {
        self.conn.send(&format!("world.setBlock({},{},{},{},{})", pos.x, pos.y, pos.z, blocktype, blockdata));
    }

    pub fn set_blocks(&mut self, pos1:Vec3, pos2:Vec3, blocktype:u8, blockdata:u8) {
        self.conn.send(&format!("world.setBlocks({},{},{},{},{},{},{},{})", pos1.x,pos1.y,pos1.z,pos2.x,pos2.y,pos2.z,blocktype,blockdata));
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