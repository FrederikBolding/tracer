use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::AABB,
    ray::{HitRecord, Hittable, Interval, Ray, WorldObject},
};

// Bounding volume hierarchy
pub struct BVHNode {
    // TODO: This turned out very complex, look into refactoring?
    left: Arc<WorldObject>,
    right: Arc<WorldObject>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new_leaf(left: Arc<WorldObject>, right: Arc<WorldObject>) -> Self {
        let bounding_box = AABB::from_bounding_boxes(left.bounding_box(), right.bounding_box());

        Self {
            left,
            right,
            bounding_box,
        }
    }

    pub fn new(objects: Vec<Arc<WorldObject>>) -> Self {
        if objects.len() == 1 {
            return BVHNode::new_leaf(objects[0].clone(), objects[0].clone());
        } else if objects.len() == 2 {
            return BVHNode::new_leaf(objects[0].clone(), objects[1].clone());
        }

        let mut bounding_box = AABB::empty();
        for object in &objects {
            bounding_box = AABB::from_bounding_boxes(bounding_box, object.bounding_box());
        }

        let sort_axis = bounding_box.longest_axis();

        let mut copied_objects = objects.clone();
        copied_objects.sort_by(|a, b| {
            let a_axis_interval = a.bounding_box().axis_interval(sort_axis);
            let b_axis_interval = b.bounding_box().axis_interval(sort_axis);

            let a_center = (a_axis_interval.min() + a_axis_interval.max()) / 2.0;
            let b_center = (b_axis_interval.min() + b_axis_interval.max()) / 2.0;

            a_center.partial_cmp(&b_center).unwrap()
        });

        let middle = copied_objects.len() / 2;
        let right = Arc::new(WorldObject::BVHNode(BVHNode::new(
            copied_objects.split_off(middle),
        )));
        let left = Arc::new(WorldObject::BVHNode(BVHNode::new(copied_objects)));

        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BVHNode {
    fn bounding_box(&self) -> crate::aabb::AABB {
        self.bounding_box
    }

    fn hit(&self, ray: &Ray, t: &Interval) -> Option<HitRecord> {
        if self.bounding_box.hit(ray, t).is_none() {
            return None;
        }

        let left_hit = self.left.hit(ray, t);

        let t_max = match left_hit {
            Some(ref left_hit) => left_hit.t(),
            None => t.max(),
        };
        let right_interval = Interval::new(t.min(), t_max);
        let right_hit = self.right.hit(ray, &right_interval);

        if right_hit.is_some() {
            return right_hit;
        }

        left_hit
    }
}
