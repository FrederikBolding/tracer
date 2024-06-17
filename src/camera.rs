use crate::{
    ray::Ray,
    vec::{unit_vector, Vector3},
    world::World,
};

pub struct Camera {
    pub frame_buffer: Vec<u32>,
    width: u32,
    height: u32,
    center: Vector3,

    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pixel00_loc: Vector3,
}

impl Camera {
    pub fn new(width: u32) -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let height = width as f64 / aspect_ratio;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (width as f64 / height as f64);

        let camera_center = Vector3::zero();
        let focal_length = 1.0;

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height;

        let viewport_upper_left = camera_center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            width,
            height: height as u32,
            frame_buffer: vec![0; width as usize * height as usize],
            center: Vector3::zero(),
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn render(&mut self, world: &World) {
        for j in 0..self.height {
            for i in 0..self.width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * (i as f64))
                    + (self.pixel_delta_v * (j as f64));
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = Camera::ray_color(&world, &ray);

                // TODO: Simplify color handling
                let (r, g, b) = (color.x() as u32, color.y() as u32, color.z() as u32);
                self.frame_buffer[(i + (j * self.width)) as usize] = (r << 16) | (g << 8) | b
            }
        }
    }

    fn ray_color(world: &World, ray: &Ray) -> Vector3 {
        let hit = world.hit(ray, 0.0, f64::INFINITY);
        if hit.is_some() {
            let normal = hit.unwrap().normal();
            return Vector3::new(
                (normal.x() + 1.0) * 255.0,
                (normal.y() + 1.0) * 255.0,
                (normal.z() + 1.0) * 255.0,
            ) * 0.5;
        }

        let unit_direction = unit_vector(ray.direction());

        let a = 0.5 * (unit_direction.y() + 1.0);

        return Vector3::new(255.0, 255.0, 255.0) * (1.0 - a)
            + Vector3::new(127.5, 178.5, 255.0) * a;
    }
}
