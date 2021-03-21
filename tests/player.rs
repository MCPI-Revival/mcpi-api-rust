use mcpi::{Vec3, create};

#[test]
fn main() {
    let mut mc = create("localhost:4711");
    let posp = mc.player().get_player_pos();
    let posp2 = Vec3::from(posp.x, posp.y + 10.0, posp.z);
    mc.player().set_player_pos(&posp2);
}