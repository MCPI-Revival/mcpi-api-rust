use mcpi_api::{Vec3, create, TileVec3};

#[test]
fn main() {
    let mut mc = create("localhost:4711");
    let id = mc.get_player_entity_ids()[0];
    let posp = mc.entity().get_pos(id);
    let posp2 = Vec3::from(posp.x, posp.y + 10.0, posp.z);
    mc.entity().set_pos(id, &posp2);
    let post = mc.entity().get_tile_pos(id);
    let post2 = TileVec3::from(post.x, post.y + 10, post.z);
    mc.entity().set_tile_pos(id, &post2);
}