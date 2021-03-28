use mcpi_api::{create, TileVec3};

#[test]
fn main() {
    let mut mc = create("localhost:4711");
    mc.post_to_chat("Hello world!");
    let pos1 = TileVec3::from(-31,6,-11);
    let pos2 = TileVec3::from(-32,6,-12);
    let pos3 = TileVec3::from(-31,7,-11);
    let pos4 = TileVec3::from(-30,8,-10);
    println!("{:?}",mc.get_player_entity_ids());
    println!("{:?}", mc.get_block(&pos1));
    println!("{:?}", mc.get_block_with_data(&pos1));
    println!("{:?}", mc.get_blocks(&pos1, &pos2));
    println!("{:?}", mc.get_blocks_with_data(&pos1, &pos2));
    mc.set_blocks(&pos3, &pos4, 2,0);
    mc.set_block(&pos3, 1,0);
    println!("{:?}", mc.get_height(&pos4));
}