use mcpi::{Vec3, create, TileVec3};

#[test]
fn main() {
    let mut mc = create("localhost:4711");
    let posp = mc.player().get_pos();
    let posp2 = Vec3::from(posp.x, posp.y + 10.0, posp.z);
    mc.player().set_pos(&posp2);
    let post = mc.player().get_tile_pos();
    let post2 = TileVec3::from(post.x, post.y + 10, post.z);
    mc.player().set_tile_pos(&post2);
    mc.player().setting("autojump", true);
}