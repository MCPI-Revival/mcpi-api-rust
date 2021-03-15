use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;
use std::cmp::{min, max};

#[cfg(test)]
mod tests {
    use crate::{create, Vec3};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let mut mc = create("localhost:4711");
        mc.post_to_chat("Hello World!");
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
        self.stream.write(&format!("{}\n", msg).as_bytes()).expect("Failed to send! Is MCPI still running?");
    }

    pub fn receive(&mut self) -> String {
        let mut reader = BufReader::new(&self.stream);
        let mut line = String::new();
        reader.read_line(&mut line).expect("Failed to receive! Is MCPI still running?");
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

    pub fn get_blocks(&mut self, pos1:Vec3, pos2:Vec3) -> Vec<u8> {
        let mut results:Vec<u8> = vec![];
        for y in min(pos1.y, pos2.y)..max(pos1.y, pos2.y)+1 {
            for x in min(pos1.x, pos2.x)..max(pos1.x, pos2.x)+1 {
                for z in min(pos1.z, pos2.z)..max(pos1.z, pos2.z) + 1 {
                    results.push(self.conn.send_receive(&format!("world.getBlock({},{},{})", x,y,z)).parse::<u8>().unwrap());
                }
            }
        }
        results
    }

    pub fn get_blocks_with_data(&mut self, pos1:Vec3, pos2:Vec3) -> Vec<Vec<u8>> {
        let mut results:Vec<Vec<u8>> = vec![];
        for y in min(pos1.y, pos2.y)..max(pos1.y, pos2.y)+1 {
            for x in min(pos1.x, pos2.x)..max(pos1.x, pos2.x)+1 {
                for z in min(pos1.z, pos2.z)..max(pos1.z, pos2.z) + 1 {
                    results.push(self.conn.send_receive(&format!("world.getBlockWithData({},{},{})", x,y,z)).split(',').map(|s| s.parse()).collect::<Result<Vec<u8>, _>>().unwrap());
                }
            }
        }
        results
    }

    pub fn set_block(&mut self, pos:Vec3, blocktype:u8, blockdata:u8) {
        self.conn.send(&format!("world.setBlock({},{},{},{},{})", pos.x, pos.y, pos.z, blocktype, blockdata));
    }

    pub fn set_blocks(&mut self, pos1:Vec3, pos2:Vec3, blocktype:u8, blockdata:u8) {
        self.conn.send(&format!("world.setBlocks({},{},{},{},{},{},{},{})", pos1.x,pos1.y,pos1.z,pos2.x,pos2.y,pos2.z,blocktype,blockdata));
    }

    pub fn get_height(&mut self, pos:Vec3) -> i8 {
        self.conn.send_receive(&format!("world.getHeight({},{})", pos.x,pos.z)).parse::<i8>().unwrap()
    }

    pub fn save_checkpoint(&mut self) {
        self.conn.send("world.checkpoint.save()");
    }

    pub fn restore_checkpoint(&mut self) {
        self.conn.send("world.checkpoint.restore()");
    }

    pub fn setting(&mut self, setting:&str, status:bool) {
        self.conn.send(&format!("world.setting({},{})",setting,if status == true {1} else {0}));
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