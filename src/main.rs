// https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::sync::Arc;

use minifb::{Key, Window, WindowOptions};
use tracer::{
    camera::Camera,
    material::{Dielectric, Lambertian, Material, Metal},
    ray::WorldObject,
    sphere::Sphere,
    util::{random_color, random_color_range, random_float, random_unit_float},
    vec::Vector3,
    world::World,
};

fn main() {
    let width = 800;
    let from = Vector3::new(13.0, 2.0, 3.0);
    let to = Vector3::new(0.0, 0.0, 0.0);
    let vertical_fov = 20.0;
    let focus_distance = 10.0;
    let background = Vector3::new(0.7, 0.8, 1.0);

    let camera = Camera::new(
        width,
        from,
        to,
        vertical_fov,
        focus_distance,
        defocus_angle,
        samples_per_pixel,
        background,
    );
    let height = camera.height();

    let mut frame_buffer = vec![0; width as usize * height as usize];

    let mut window = Window::new(
        "tracer - ESC to exit",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut objects: Vec<Arc<WorldObject>> = vec![];

    let material_ground = Material::Lambertian(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    objects.push(WorldObject::Sphere(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )).into());

    let material_glass = Material::Dielectric(Dielectric::new(1.50));
    objects.push(WorldObject::Sphere(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material_glass,
    )).into());

    let material_diffuse = Material::Lambertian(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    objects.push(WorldObject::Sphere(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        material_diffuse,
    )).into());

    let material_metal = Material::Metal(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    objects.push(WorldObject::Sphere(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        material_metal,
    )).into());

    for x in -11..11 {
        for z in -11..11 {
            let random_material = random_unit_float();
            let center = Vector3::new(
                x as f64 + 0.9 * random_unit_float(),
                0.2,
                z as f64 + 0.9 * random_unit_float(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = match random_material {
                    _ if (0.0..=0.8).contains(&random_material) => {
                        Material::Lambertian(Lambertian::new(random_color() * random_color()))
                    }
                    _ if (0.8..=0.95).contains(&random_material) => Material::Metal(Metal::new(
                        random_color_range(0.5, 1.0),
                        random_float(0.0, 0.5),
                    )),
                    _ => Material::Dielectric(Dielectric::new(1.50)),
                };

                objects.push(WorldObject::Sphere(Sphere::new(center, 0.2, material)).into());
            }
        }
    }

    let world = World::new(objects);

    camera.render(&world, &mut frame_buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&frame_buffer, width as usize, height as usize)
            .unwrap();
    }
}
