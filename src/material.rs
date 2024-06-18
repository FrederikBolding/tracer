use crate::{
    ray::{HitRecord, Ray},
    vec::Vector3,
};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(ray_in, hit_record),
            Material::Metal(metal) => metal.scatter(ray_in, hit_record),
        }
    }
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)>;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let scatter_direction = hit_record.normal() + Vector3::random_unit_vector();

        let direction = if scatter_direction.near_zero() {
            hit_record.normal()
        } else {
            scatter_direction
        };

        let ray = Ray::new(hit_record.point(), direction);

        Some((ray, self.albedo))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vector3,
}

impl Metal {
    pub fn new(albedo: Vector3) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let direction = Vector3::reflect(ray_in.direction(), hit_record.normal());

        let ray = Ray::new(hit_record.point(), direction);

        Some((ray, self.albedo))
    }
}
