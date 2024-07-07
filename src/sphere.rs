use crate::{
    aabb::AABB,
    material::Material,
    ray::{HitRecord, Hittable, Interval, Ray},
    vec::{dot_product, Vector3},
};

pub struct Sphere {
    center: Vector3,
    radius: f64,
    radius_squared: f64,
    material: Material,
    bounding_box: AABB,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Material) -> Self {
        let radius_vector = Vector3::new(radius, radius, radius);
        let bounding_box = AABB::from_points(center - radius_vector, center + radius_vector);

        Self {
            center,
            radius,
            radius_squared: radius * radius,
            material,
            bounding_box,
        }
    }

    pub fn center(&self) -> Vector3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> Material {
        self.material
    }
}

impl Hittable for Sphere {
    fn bounding_box(&self) -> crate::aabb::AABB {
        self.bounding_box
    }

    fn hit(&self, ray: &Ray, t: &Interval) -> Option<HitRecord> {
        let oc = self.center() - ray.origin();
        let a = ray.direction_length_squared(); // dot(dir, dir)
        let h = dot_product(ray.direction(), oc);
        let c = oc.length_squared() - self.radius_squared;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        for root in [(h - sqrtd) / a, (h + sqrtd) / a] {
            if t.surrounds(root) {
                let point = ray.at(root);
                let outward_normal = (point - self.center()) / self.radius();
                let front_face = dot_product(ray.direction(), outward_normal) < 0.0;
                let normal = if front_face {
                    outward_normal
                } else {
                    -outward_normal
                };
                return Some(HitRecord::new(point, normal, self.material, root, front_face));
            }
        }

        None
    }
}
