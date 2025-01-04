use the_next_week::scenes::{
    bouncing_spheres::bouncing_spheres,
    checkered_spheres::checkered_spheres,
    earth::earth
};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    const SCENE_ID: i32 = 3;

    match SCENE_ID {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        _ => panic!("Invalid scene id")
    }
}
