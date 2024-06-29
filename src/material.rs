use crate::{
    ray::{HitRecord, Ray},
    util::random_unit_float,
    vec::{dot_product, unit_vector, Vector3},
};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    Light(Light),
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(ray_in, hit_record),
            Material::Metal(metal) => metal.scatter(ray_in, hit_record),
            Material::Dielectric(dielectric) => dielectric.scatter(ray_in, hit_record),
            _ => None,
        }
    }

    pub fn emitted(&self, point: Vector3) -> Vector3 {
        match self {
            Material::Light(light) => light.emitted(point),
            _ => Vector3::zero(),
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let reflected = Vector3::reflect(ray_in.direction(), hit_record.normal());

        let direction = unit_vector(reflected) + (Vector3::random_unit_vector() * self.fuzz);

        if dot_product(direction, hit_record.normal()) > 0.0 {
            Some((Ray::new(hit_record.point(), direction), self.albedo))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlick Approximation of reflectance
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)> {
        let normal = hit_record.normal();
        let refraction_index = if hit_record.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(ray_in.direction());
        let cos_theta = dot_product(-unit_direction, normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Determine whether there is a solution using Snells law.
        // If not, cannot refract and must reflect.
        let direction = if refraction_index * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, refraction_index) > random_unit_float()
        {
            Vector3::reflect(unit_direction, normal)
        } else {
            Vector3::refract(unit_direction, normal, refraction_index)
        };

        Some((
            Ray::new(hit_record.point(), direction),
            Vector3::new(1.0, 1.0, 1.0),
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    color: Vector3,
}

impl Light {
    pub fn new(color: Vector3) -> Self {
        Self { color }
    }
}

impl Light {
    pub fn emitted(&self, _point: Vector3) -> Vector3 {
        self.color
    }
}
