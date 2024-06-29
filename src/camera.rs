use rayon::prelude::*;
use std::time::Instant;

use crate::{
    ray::{Interval, Ray},
    util::sample_square,
    vec::{cross_product, random_in_unit_disk, unit_vector, Vector3},
    world::World,
};

pub struct Camera {
    width: u32,
    height: u32,
    center: Vector3,
    samples_per_pixel: u32,
    max_depth: u32,
    background: Vector3,

    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pixel00_loc: Vector3,
    pixel_samples_scale: f64,
    defocus: bool,
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
}

impl Camera {
    pub fn new(
        width: u32,
        look_from: Vector3,
        look_at: Vector3,
        vertical_fov: f64,
        focus_distance: f64,
        defocus_angle: f64,
        samples_per_pixel: u32,
        background: Vector3,
    ) -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let height = width as f64 / aspect_ratio;

        let camera_up = Vector3::new(0.0, 1.0, 0.0);
        let camera_center = look_from;

        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (width as f64 / height as f64);

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross_product(camera_up, w));
        let v = cross_product(w, u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height;

        let viewport_upper_left =
            camera_center - (w * focus_distance) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_distance * (defocus_angle.to_radians() / 2.0).tan();

        Self {
            width,
            height: height as u32,
            center: camera_center,
            samples_per_pixel,
            max_depth: 50,
            background,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            defocus_disk_u: u * defocus_radius,
            defocus_disk_v: v * defocus_radius,
            defocus: defocus_angle > 0.0,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn render(&self, world: &World, frame_buffer: &mut [u32]) {
        let start = Instant::now();

        // One chunk is one row of pixels in the image
        let chunks: Vec<(usize, &mut [u32])> = frame_buffer
            .chunks_mut(self.width as usize)
            .enumerate()
            .collect();

        let interval = Interval::new(0.0, 1.0);

        chunks.into_par_iter().for_each(|(height_index, chunk)| {
            for i in 0..self.width {
                let mut color = Vector3::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, height_index as u32);
                    color = color + self.ray_color(&world, &ray, self.max_depth);
                }

                let scaled_color = color * self.pixel_samples_scale;

                let (r, g, b) = (
                    (interval.clamp(scaled_color.x().sqrt()) * 255.0) as u32,
                    (interval.clamp(scaled_color.y().sqrt()) * 255.0) as u32,
                    (interval.clamp(scaled_color.z().sqrt()) * 255.0) as u32,
                );

                // Pack RGB into 1 u32
                chunk[i as usize] = (r << 16) | (g << 8) | b
            }
        });

        println!("Frame time: {}ms", start.elapsed().as_millis());
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();

        let pixel_sample_center = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));

        let origin = if self.defocus {
            self.defocus_disk_sample()
        } else {
            self.center
        };

        Ray::new(origin, pixel_sample_center - origin)
    }

    fn ray_color(&self, world: &World, ray: &Ray, depth: u32) -> Vector3 {
        if depth <= 0 {
            return Vector3::zero();
        }

        let interval = Interval::new(0.001, f64::INFINITY);
        let hit = world.hit(ray, &interval);

        match hit {
            Some(hit) => {
                let material = hit.material();

                let emitted = material.emitted(hit.point());

                let bounce = material.scatter(ray, &hit);
                match bounce {
                    Some((bounce_ray, attenunation)) => {
                        emitted + self.ray_color(world, &bounce_ray, depth - 1) * attenunation
                    }
                    None => emitted,
                }
            }
            None => self.background,
        }
    }

    fn defocus_disk_sample(&self) -> Vector3 {
        let point = random_in_unit_disk();
        self.center + (self.defocus_disk_u * point.x()) + (self.defocus_disk_v * point.y())
    }
}
