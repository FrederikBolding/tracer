use std::sync::Arc;

use crate::{
    bvh::BVHNode,
    ray::{HitRecord, Hittable, Interval, Ray, WorldObject},
};

pub struct World {
    node: BVHNode,
}

impl World {
    pub fn new(objects: Vec<Arc<WorldObject>>) -> Self {
        let node = BVHNode::new(objects);
        Self { node }
    }

    pub fn hit(&self, ray: &Ray, t: &Interval) -> Option<HitRecord> {
        self.node.hit(ray, t)
    }
}
