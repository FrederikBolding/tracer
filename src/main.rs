// https://raytracing.github.io/books/RayTracingInOneWeekend.html

use minifb::{Key, Window, WindowOptions};
use tracer::{
    camera::Camera,
    material::{Dielectric, Lambertian, Material, Metal},
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
    let defocus_angle = 0.6;
    let samples_per_pixel = 500;

    let mut camera = Camera::new(
        width,
        from,
        to,
        vertical_fov,
        focus_distance,
        defocus_angle,
        samples_per_pixel,
    );
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

    let material_ground = Material::Lambertian(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));

    let material_glass = Material::Dielectric(Dielectric::new(1.50));
    world.add(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material_glass,
    ));

    let material_diffuse = Material::Lambertian(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        material_diffuse,
    ));

    let material_metal = Material::Metal(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        material_metal,
    ));

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

                world.add(Sphere::new(center, 0.2, material));
            }
        }
    }

    camera.render(&world);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&camera.frame_buffer, width as usize, height as usize)
            .unwrap();
    }
}
