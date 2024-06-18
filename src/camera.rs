use std::time::Instant;

use crate::{
    ray::Ray,
    util::sample_square,
    vec::{unit_vector, Vector3},
    world::World,
};

pub struct Camera {
    pub frame_buffer: Vec<u32>,
    width: u32,
    height: u32,
    center: Vector3,
    samples_per_pixel: u32,
    max_depth: u32,

    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pixel00_loc: Vector3,
    pixel_samples_scale: f64,
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

        let samples_per_pixel = 100;

        Self {
            width,
            height: height as u32,
            frame_buffer: vec![0; width as usize * height as usize],
            center: Vector3::zero(),
            samples_per_pixel,
            max_depth: 10,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();

        let pixel_sample_center = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));

        Ray::new(self.center, pixel_sample_center - self.center)
    }

    pub fn render(&mut self, world: &World) {
        let start = Instant::now();

        for j in 0..self.height {
            for i in 0..self.width {
                let mut color = Vector3::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    color = color + Self::ray_color(&world, &ray, self.max_depth);
                }

                // TODO: Simplify color handling
                let (r, g, b) = (
                    (clamp((color.x() * self.pixel_samples_scale).sqrt(), 0.0, 1.0) * 255.0) as u32,
                    (clamp((color.y() * self.pixel_samples_scale).sqrt(), 0.0, 1.0) * 255.0) as u32,
                    (clamp((color.z() * self.pixel_samples_scale).sqrt(), 0.0, 1.0) * 255.0) as u32,
                );
                self.frame_buffer[(i + (j * self.width)) as usize] = (r << 16) | (g << 8) | b
            }
        }

        println!("Frame time: {}ms", start.elapsed().as_millis());
    }

    fn ray_color(world: &World, ray: &Ray, depth: u32) -> Vector3 {
        if depth <= 0 {
            return Vector3::zero();
        }

        let hit = world.hit(ray, 0.001, f64::INFINITY);

        match hit {
            Some(hit) => {
                let material = hit.material();

                let bounce = material.scatter(ray, &hit);
                match bounce {
                    Some((bounce_ray, attenunation)) => {
                        Self::ray_color(world, &bounce_ray, depth - 1) * attenunation
                    }
                    None => Vector3::zero(),
                }
            }
            None => {
                let unit_direction = unit_vector(ray.direction());

                let a = 0.5 * (unit_direction.y() + 1.0);

                return Vector3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vector3::new(0.5, 0.7, 1.0) * a;
            }
        }
    }
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    return if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    };
}
