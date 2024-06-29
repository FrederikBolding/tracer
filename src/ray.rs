use crate::{
    aabb::AABB,
    bvh::BVHNode,
    material::Material,
    mesh::Mesh,
    quad::Quad,
    sphere::Sphere,
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
    pub fn new(
        point: Vector3,
        normal: Vector3,
        material: Material,
        t: f64,
        front_face: bool,
    ) -> HitRecord {
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

// TODO: Rename
pub enum WorldObject {
    BVHNode(BVHNode),
    Sphere(Sphere),
    Quad(Quad),
    Mesh(Mesh),
}

impl WorldObject {
    pub fn hit(&self, ray: &Ray, t: &Interval) -> Option<HitRecord> {
        match self {
            WorldObject::BVHNode(node) => node.hit(ray, t),
            WorldObject::Sphere(sphere) => sphere.hit(ray, t),
            WorldObject::Quad(quad) => quad.hit(ray, t),
            WorldObject::Mesh(mesh) => mesh.hit(ray, t),
        }
    }

    pub fn bounding_box(&self) -> AABB {
        match self {
            WorldObject::BVHNode(node) => node.bounding_box(),
            WorldObject::Sphere(sphere) => sphere.bounding_box(),
            WorldObject::Quad(quad) => quad.bounding_box(),
            WorldObject::Mesh(mesh) => mesh.bounding_box(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
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

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: Interval, b: Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Self { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn surrounds(&self, value: f64) -> bool {
        value > self.min && value < self.max
    }

    pub fn clamp(&self, value: f64) -> f64 {
        match value {
            _ if value < self.min => self.min,
            _ if value > self.max => self.max,
            _ => value,
        }
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}
