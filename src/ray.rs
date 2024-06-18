use crate::{
    material::Material,
    vec::{dot_product, Vector3},
};

pub struct Ray {
    origin: Vector3,    // A
    direction: Vector3, // b
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }
}

pub struct HitRecord {
    point: Vector3,
    normal: Vector3,
    material: Material, // TODO: Should this be a borrowed value?
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vector3, normal: Vector3, material: Material, t: f64, front_face: bool) -> HitRecord {
        Self {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }

    pub fn point(&self) -> Vector3 {
        self.point
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub fn derive_face_normal(ray: &Ray, outward_normal: Vector3) -> (bool, Vector3) {
    let front_face = dot_product(ray.direction(), outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };

    (front_face, normal)
}
