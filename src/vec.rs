use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::util::{random_float, random_unit_float};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn random_unit() -> Vector3 {
        Self::new(
            random_unit_float(),
            random_unit_float(),
            random_unit_float(),
        )
    }

    pub fn random(min: f64, max: f64) -> Vector3 {
        Self::new(
            random_float(min, max),
            random_float(min, max),
            random_float(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let point = Self::random(-1.0, 1.0);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    pub fn random_unit_vector() -> Vector3 {
        unit_vector(Self::random_in_unit_sphere())
    }

    pub fn random_on_hemisphere(normal: Vector3) -> Vector3 {
        let on_unit_sphere = Self::random_unit_vector();
        if dot_product(on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn reflect(vector: Vector3, normal: Vector3) -> Vector3 {
        vector - (normal * 2.0 * dot_product(vector, normal))
    }

    pub fn refract(uv: Vector3, normal: Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = dot_product(-uv, normal).min(1.0);
        let r_out_perpendicular = (uv + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * (-(1.0 - r_out_perpendicular.length_squared()).abs().sqrt());
        r_out_perpendicular + r_out_parallel
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x * rhs.x(), self.y * rhs.y(), self.z * rhs.z())
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1f64 / rhs)
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

pub fn dot_product(u: Vector3, v: Vector3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross_product(u: Vector3, v: Vector3) -> Vector3 {
    Vector3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn unit_vector(v: Vector3) -> Vector3 {
    v / v.length()
}

pub fn random_in_unit_disk() -> Vector3 {
    loop {
        let point = Vector3::new(random_float(-1.0, 1.0), random_float(-1.0, 1.0), 0.0);
        if point.length_squared() < 1.0 {
            return point;
        }
    }
}
