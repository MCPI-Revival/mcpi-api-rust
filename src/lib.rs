//! # MCPI API
//! `mcpi_api` is a wrapper for the Minecraft Pi Edition API handling parsing and other aspects for you.
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;
use std::cmp::{min, max};

#[cfg(test)]
mod tests {

    #[test]
    fn development_test() {
    }
}

///Struct containing functions and a Connection struct.
pub struct Minecraft {
    conn:Connection
}

struct Connection {
    stream:TcpStream
}

///Struct containing functions and a Connection struct.
pub struct Player<'a> {
    conn:&'a mut Connection
}

pub struct Entity<'a> {
    conn:&'a mut Connection
}

pub struct Event<'a> {
    conn:&'a mut Connection
}

///Struct used to specify tile positions.
#[derive(Debug)]
pub struct TileVec3 {
    pub x:i32,
    pub y:i32,
    pub z:i32
}

///Struct used to specify entity positions.
#[derive(Debug)]
pub struct Vec3 {
    pub x:f32,
    pub y:f32,
    pub z:f32
}

pub struct BlockEvent {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub hit: bool
}

impl TileVec3 {
    /// Function to generate a TileVec3 from 3 i32's
    pub fn from(x:i32, y:i32, z:i32) -> TileVec3 {
        TileVec3 {
            x,
            y,
            z
        }
    }
    ///Function to generate a TileVec3 from a Vec<i32>
    /// # Panics
    /// This function panics if the vector contains less then 3 elements
    pub fn from_vector(vec:&Vec<i32>) -> TileVec3 {
        TileVec3 {
            x: vec[0],
            y: vec[1],
            z: vec[2]
        }
    }
}

impl BlockEvent {
    pub fn new(x:i32, y:i32, z:i32, hit:bool) -> BlockEvent {
        BlockEvent {
            x,
            y,
            z,
            hit
        }
    }
}

impl Vec3 {
    ///Function to generate a Vec3 from 3 f32's
    pub fn from(x:f32, y:f32, z:f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }
    ///Function to generate a Vec3 from a Vec<f32>
    /// # Panics
    /// This function panics if the vector contains less then 3 elements
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

impl Event<'_> {
    pub fn clear_all(&mut self) {
        self.conn.send("events.clear");
    }

    pub fn poll_block_hits(&mut self) -> BlockEvent {
        let s = self.conn.send_receive("events.block.hits()");

        let mut pos = Vec::new();

        if s.len() > 0 {
            for i in s.split(",").take(3) {
                let i = i.parse::<i32>().unwrap();
                pos.push(i)
            }
            return BlockEvent::new(pos[0], pos[1], pos[2], true);
        }

        let pos = BlockEvent::new(0, 0, 0, false);
        return pos
    }
}

///# Panics
/// All functions implemented on the Minecraft struct might panic if the API is not running anymore or packages fail to send.
/// This might change in a 0.2.0 version of this crate
impl Minecraft {
    ///Post a message to the chat
    pub fn post_to_chat(&mut self, msg:&str) {
        self.conn.send(&format!("chat.post({})", msg));
    }
    ///Get the block at a specific position
    pub fn get_block(&mut self, pos:&TileVec3) -> u8 {
        self.conn.send_receive(&format!("world.getBlock({},{},{})", pos.x, pos.y, pos.z)).parse::<u8>().unwrap()
    }
    ///Get the block with data at a specific position
    pub fn get_block_with_data(&mut self, pos:&TileVec3) -> Vec<u8> {
        self.conn.send_receive(&format!("world.getBlockWithData({},{},{})", pos.x, pos.y, pos.z)).split(',').map(|s| s.parse()).collect::<Result<Vec<u8>, _>>().unwrap()
    }
    ///Get a array of blocks contained in the specified area
    pub fn get_blocks(&mut self, pos1:&TileVec3, pos2:&TileVec3) -> Vec<u8> {
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
    ///Get a array of blocks with their data contained in the specified area
    pub fn get_blocks_with_data(&mut self, pos1:&TileVec3, pos2:&TileVec3) -> Vec<Vec<u8>> {
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
    ///Set a block at a specific position
    pub fn set_block(&mut self, pos:&TileVec3, blocktype:u8, blockdata:u8) {
        self.conn.send(&format!("world.setBlock({},{},{},{},{})", pos.x, pos.y, pos.z, blocktype, blockdata));
    }
    ///Fill the specified area with the a block
    pub fn set_blocks(&mut self, pos1:&TileVec3, pos2:&TileVec3, blocktype:u8, blockdata:u8) {
        self.conn.send(&format!("world.setBlocks({},{},{},{},{},{},{},{})", pos1.x,pos1.y,pos1.z,pos2.x,pos2.y,pos2.z,blocktype,blockdata));
    }
    ///Get the highest point at the specified position
    pub fn get_height(&mut self, pos:&TileVec3) -> i8 {
        self.conn.send_receive(&format!("world.getHeight({},{})", pos.x,pos.z)).parse::<i8>().unwrap()
    }
    ///Save the current world state as a checkpoint
    pub fn save_checkpoint(&mut self) {
        self.conn.send("world.checkpoint.save()");
    }
    ///Restore a previously saved world state
    pub fn restore_checkpoint(&mut self) {
        self.conn.send("world.checkpoint.restore()");
    }
    ///Set a world setting to true or false.
    /// Available settings: "world_immutable", "nametags_visible"
    pub fn setting(&mut self, setting:&str, status:bool) {
        self.conn.send(&format!("world.setting({},{})",setting,if status == true {1} else {0}));
    }
    ///Get a list of entity ids for all online players
    pub fn get_player_entity_ids(&mut self) -> Vec<u16> {
        self.conn.send_receive(&format!("world.getPlayerIds()")).split("|").map(|s| s.parse()).collect::<Result<Vec<u16>, _>>().unwrap()
    }
    ///Get a instance of the Player struct containing player related functions
    pub fn player(&mut self) -> Player {
        Player {
            conn: &mut self.conn
        }
    }

    pub fn entity(&mut self) -> Entity {
        Entity {
            conn: &mut self.conn
        }
    }
    
    pub fn event(&mut self) -> Event {
        Event {
            conn: &mut self.conn
        }
    }
}
///# Panics
/// All functions implemented on the Player struct might panic if the API is not running anymore or packages fail to send.
/// This might change in a 0.2.0 version of this crate
impl Player<'_> {
    ///Get the position of the main player
     pub fn get_pos(&mut self) -> Vec3 {
        Vec3::from_vector(&self.conn.send_receive(&format!("player.getPos()")).split(',').map(|s| s.parse()).collect::<Result<Vec<f32>, _>>().unwrap())
    }
    ///Set the position of the main player
    pub fn set_pos(&mut self, pos:&Vec3) {
        self.conn.send(&format!("player.setPos({},{},{})", pos.x, pos.y, pos.z));
    }
    ///Get the tile position of the main player
    pub fn get_tile_pos(&mut self) -> TileVec3 {
        let vec:Vec<i32> = self.conn.send_receive(&format!("player.getTile()")).split(',').map(|s| s.parse()).collect::<Result<Vec<i32>, _>>().unwrap();
        TileVec3::from_vector(&vec)
    }
    ///Set the tile position of the main player
    pub fn set_tile_pos(&mut self, pos:&TileVec3) {
        self.conn.send(&format!("player.setTile({},{},{})", pos.x, pos.y, pos.z))
    }
    ///Set a setting for the main player
    /// Available settings: "autojump"
    pub fn setting(&mut self, setting:&str, status:bool) {
        self.conn.send(&format!("player.setting({},{})",setting,if status {1} else {0}));
    }
}

impl Entity<'_> {
    ///Get the position of a player entity
    pub fn get_pos(&mut self, id:u16) -> Vec3 {
        Vec3::from_vector(&self.conn.send_receive(&format!("entity.getPos({})", id)).split(',').map(|s| s.parse()).collect::<Result<Vec<f32>, _>>().unwrap())
    }
    ///Set the position of a player entity
    pub fn set_pos(&mut self, id:u16, pos:&Vec3) {
        self.conn.send(&format!("entity.setPos({},{},{},{})", id, pos.x, pos.y, pos.z));
    }
    ///Get the tile position of a player entity
    pub fn get_tile_pos(&mut self, id:u16) -> TileVec3 {
        let vec:Vec<i32> = self.conn.send_receive(&format!("entity.getTile({})", id)).split(',').map(|s| s.parse()).collect::<Result<Vec<i32>, _>>().unwrap();
        TileVec3::from_vector(&vec)
    }
    ///Set the tile position of a player entity
    pub fn set_tile_pos(&mut self, id:u16, pos:&TileVec3) {
        self.conn.send(&format!("entity.setTile({},{},{},{})",id, pos.x, pos.y, pos.z))
    }
}

///Function to create a Minecraft struct.
/// Takes a IP adress and a port as arguments.
/// # Examples
/// ```
/// use mcpi_api::create;
/// let mut  mc = create("localhost:4711");
/// mc.post_to_chat("Hello World!")
/// ```
/// # Panics
/// This function panics if binding to the adress fails.
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
