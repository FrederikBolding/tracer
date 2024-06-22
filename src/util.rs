use rand::Rng;

use crate::vec::Vector3;

pub fn random_unit_float() -> f64 {
    random_float(0.0, 1.0)
}

pub fn random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..max)
}

pub fn sample_square() -> Vector3 {
    Vector3::new(random_unit_float() - 0.5, random_unit_float() - 0.5, 0.0)
}

pub fn random_color() -> Vector3 {
    random_color_range(0.0, 1.0)
}

pub fn random_color_range(min: f64, max: f64) -> Vector3 {
    Vector3::new(
        random_float(min, max),
        random_float(min, max),
        random_float(min, max),
    )
}
