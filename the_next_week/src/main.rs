use the_next_week::scenes::{
    bouncing_spheres::bouncing_spheres,
    checkered_spheres::checkered_spheres
};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    const SCENE_ID: i32 = 2;

    match SCENE_ID {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        _ => panic!("Invalid scene id")
    }
}
