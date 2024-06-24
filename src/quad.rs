use crate::{
    aabb::AABB,
    material::Material,
    ray::{HitRecord, Hittable, Interval, Ray},
    vec::{cross_product, dot_product, unit_vector, Vector3},
};

pub enum QuadType {
    Quad,
    Triangle,
}

// Quadrilateral - technically a parallelogram
pub struct Quad {
    q: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    normal: Vector3,
    d: f64,
    quad_type: QuadType,
    material: Material,
    bounding_box: AABB,
}

impl Quad {
    pub fn new_quad(q: Vector3, u: Vector3, v: Vector3, material: Material) -> Self {
        Self::new(q, u, v, material, QuadType::Quad)
    }

    pub fn new_triangle(a: Vector3, b: Vector3, c: Vector3, material: Material) -> Self {
        Self::new(a, b - a, c - a, material, QuadType::Triangle)
    }

    fn new(q: Vector3, u: Vector3, v: Vector3, material: Material, quad_type: QuadType) -> Self {
        let n = cross_product(u, v);
        let normal = unit_vector(n);
        let d = dot_product(normal, q);
        let w = n / dot_product(n, n);

        let bounding_box_diagonal_1 = AABB::from_points(q, q + u + v);
        let bounding_box_diagonal_2 = AABB::from_points(q + u, q + v);

        let bounding_box =
            AABB::from_bounding_boxes(bounding_box_diagonal_1, bounding_box_diagonal_2);

        Self {
            q,
            u,
            v,
            w,
            normal,
            d,
            quad_type,
            material,
            bounding_box,
        }
    }

    pub fn material(&self) -> Material {
        self.material
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> crate::aabb::AABB {
        self.bounding_box
    }

    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denominator = dot_product(self.normal, ray.direction());

        if denominator.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - dot_product(self.normal, ray.origin())) / denominator;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);

        let planar_intersection = intersection - self.q;

        let alpha = dot_product(self.w, cross_product(planar_intersection, self.v));
        let beta = dot_product(self.w, cross_product(self.u, planar_intersection));

        match self.quad_type {
            QuadType::Quad => {
                let interval = Interval::new(0.0, 1.0);
                if !interval.contains(alpha) || !interval.contains(beta) {
                    return None;
                }
            }
            QuadType::Triangle => {
                if !(alpha > 0.0 && beta > 0.0 && alpha + beta < 1.0) {
                    return None;
                }
            }
        }

        let front_face = denominator < 0.0;

        let normal: Vector3 = if front_face {
            self.normal
        } else {
            -self.normal
        };

        Some(HitRecord::new(
            intersection,
            normal,
            self.material,
            t,
            front_face,
        ))
    }
}
