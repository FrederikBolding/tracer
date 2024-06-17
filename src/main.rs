// https://raytracing.github.io/books/RayTracingInOneWeekend.html

use minifb::{Key, Window, WindowOptions};
use tracer::{camera::Camera, sphere::Sphere, vec::Vector3, world::World};

fn main() {
    let width = 800;
    let mut camera = Camera::new(width);
    let height = camera.height();

    let mut window = Window::new(
        "tracer - ESC to exit",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut world = World::new();

    world.add(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));

    camera.render(&world);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&camera.frame_buffer, width as usize, height as usize)
            .unwrap();
    }
}
