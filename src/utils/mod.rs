pub mod tile_mesh;

pub struct Layers {
    pub ground: f32,
    pub interactive: f32,
    pub player: f32,
    // pub dialogue: f32,
    // pub overlay: f32,
}

// A central place to track how graphics are layered.
pub const Z: Layers = Layers {
    ground: 0.0,
    interactive: 1.0,
    player: 5.0,
    // dialogue: 10.0
    // overlay: 50.0,
};
