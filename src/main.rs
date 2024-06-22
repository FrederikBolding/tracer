// https://raytracing.github.io/books/RayTracingInOneWeekend.html

use minifb::{Key, Window, WindowOptions};
use tracer::{
    camera::Camera,
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
    vec::Vector3,
    world::World,
};

fn main() {
    let width = 400;
    let from = Vector3::new(-2.0, 2.0, 1.0);
    let to = Vector3::new(0.0, 0.0, -1.0);
    let vertical_fov = 20.0;
    let focus_distance = 3.4;
    let defocus_angle = 10.0;

    let mut camera = Camera::new(width, from, to, vertical_fov, focus_distance, defocus_angle);
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

    let material_ground = Material::Lambertian(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let material_center = Material::Lambertian(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
    let material_left = Material::Dielectric(Dielectric::new(1.50));
    let material_bubble = Material::Dielectric(Dielectric::new(1.00 / 1.50));
    let material_right = Material::Metal(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    world.add(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    camera.render(&world);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&camera.frame_buffer, width as usize, height as usize)
            .unwrap();
    }
}
