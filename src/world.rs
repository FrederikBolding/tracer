use crate::{
    ray::{HitRecord, Hittable, Interval, Ray},
    sphere::Sphere,
};

pub struct World {
    objects: Vec<Box<Sphere>>,
}

impl World {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    // TODO: Should accept a Hittable
    pub fn add(&mut self, object: Sphere) {
        self.objects.push(Box::new(object));
    }

    pub fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        for object in &self.objects {
            let closest_so_far = match hit {
                Some(ref hit_record) => hit_record.t(),
                None => t.max(),
            };

            let interval = Interval::new(t.min(), closest_so_far);
            if let Some(potential_hit) = object.hit(ray, interval) {
                hit = Some(potential_hit);
            }
        }

        hit
    }
}
