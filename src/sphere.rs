use crate::{
    aabb::AABB,
    material::Material,
    ray::{derive_face_normal, HitRecord, Hittable, Interval, Ray},
    vec::{dot_product, Vector3},
};

pub struct Sphere {
    center: Vector3,
    radius: f64,
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
        let a = ray.direction().length_squared(); // dot(dir, dir)
        let h = dot_product(ray.direction(), oc);
        let c = oc.length_squared() - self.radius() * self.radius();
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        // TODO: Clean up
        if root <= t.min() || root >= t.max() {
            root = (h + sqrtd) / a;
            if root <= t.min() || root >= t.max() {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center()) / self.radius();
        let (front_face, normal) = derive_face_normal(ray, outward_normal);
        return Some(HitRecord::new(point, normal, self.material, t, front_face));
    }
}
