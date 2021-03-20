use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;
use std::cmp::{min, max};

#[cfg(test)]
mod tests {
    use crate::{create, Vec3};

    #[test]
    fn development_tests() {
        assert_eq!(2 + 2, 4);
        let pos1 = Vec3::from(-31.0, 7.0, -11.0);
        let mut mc = create("localhost:4711");
        mc.post_to_chat("Hello World!");
        println!("{:?}",mc.get_player_pos());
        mc.set_player_pos(pos1)
    }
}

pub struct Minecraft {
    conn:Connection
}

struct Connection {
    stream:TcpStream
}

#[derive(Clone, Copy, Debug)]
pub struct TileVec3 {
    pub x:i32,
    pub y:i32,
    pub z:i32
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x:f32,
    pub y:f32,
    pub z:f32
}

impl TileVec3 {
    pub fn from(x:i32, y:i32, z:i32) -> TileVec3 {
        TileVec3 {
            x,
            y,
            z
        }
    }

    pub fn from_vector(vec:&Vec<i32>) -> TileVec3 {
        TileVec3 {
            x: vec[0],
            y: vec[1],
            z: vec[2]
        }
    }
}

impl Vec3 {
    pub fn from(x:f32, y:f32, z:f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }

    pub fn from_vector(vec:&Vec<f32>) -> Vec3 {
        Vec3{
            x: vec[0],
            y: vec[1],
            z: vec[2]
        }
    }
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

    pub fn get_block(&mut self, pos:TileVec3) -> u8 {
        self.conn.send_receive(&format!("world.getBlock({},{},{})", pos.x, pos.y, pos.z)).parse::<u8>().unwrap()
    }

    pub fn get_block_with_data(&mut self, pos:TileVec3) -> Vec<u8> {
        self.conn.send_receive(&format!("world.getBlockWithData({},{},{})", pos.x, pos.y, pos.z)).split(',').map(|s| s.parse()).collect::<Result<Vec<u8>, _>>().unwrap()
    }

    pub fn get_blocks(&mut self, pos1:TileVec3, pos2:TileVec3) -> Vec<u8> {
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

    pub fn get_blocks_with_data(&mut self, pos1:TileVec3, pos2:TileVec3) -> Vec<Vec<u8>> {
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

    pub fn set_block(&mut self, pos:TileVec3, blocktype:u8, blockdata:u8) {
        self.conn.send(&format!("world.setBlock({},{},{},{},{})", pos.x, pos.y, pos.z, blocktype, blockdata));
    }

    pub fn set_blocks(&mut self, pos1:TileVec3, pos2:TileVec3, blocktype:u8, blockdata:u8) {
        self.conn.send(&format!("world.setBlocks({},{},{},{},{},{},{},{})", pos1.x,pos1.y,pos1.z,pos2.x,pos2.y,pos2.z,blocktype,blockdata));
    }

    pub fn get_height(&mut self, pos:TileVec3) -> i8 {
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

    pub fn get_player_entity_ids(&mut self) -> Vec<u16> {
        self.conn.send_receive(&format!("world.getPlayerIds()")).split("|").map(|s| s.parse()).collect::<Result<Vec<u16>, _>>().unwrap()
    }

    pub fn get_player_pos(&mut self) -> Vec3 {
        let vec:Vec<f32> = self.conn.send_receive(&format!("player.getPos()")).split(',').map(|s| s.parse()).collect::<Result<Vec<f32>, _>>().unwrap();
        Vec3::from_vector(&vec)
    }

    pub fn set_player_pos(&mut self, pos:Vec3) {
        self.conn.send(&format!("player.setPos({},{},{})", pos.x, pos.y, pos.z));
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