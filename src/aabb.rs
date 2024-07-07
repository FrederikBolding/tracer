use crate::{
    ray::{Interval, Ray},
    vec::Vector3,
};

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

// Axis-aligned bounding box
impl AABB {
    pub fn empty() -> Self {
        let empty = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
        Self {
            x: empty,
            y: empty,
            z: empty,
        }
    }

    pub fn from_bounding_boxes(a: AABB, b: AABB) -> Self {
        let x = Interval::from_intervals(a.x, b.x);
        let y = Interval::from_intervals(a.y, b.y);
        let z = Interval::from_intervals(a.z, b.z);
        Self::new(x, y, z)
    }

    pub fn from_points(a: Vector3, b: Vector3) -> Self {
        let x = if a.x() <= b.x() {
            Interval::new(a.x(), b.x())
        } else {
            Interval::new(b.x(), a.x())
        };
        let y = if a.y() <= b.y() {
            Interval::new(a.y(), b.y())
        } else {
            Interval::new(b.y(), a.y())
        };
        let z = if a.z() <= b.z() {
            Interval::new(a.z(), b.z())
        } else {
            Interval::new(b.z(), a.z())
        };

        Self::new(x, y, z)
    }

    fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let delta = 0.0001;
        Self {
            x: if x.size() < delta { x.expand(delta) } else { x },
            y: if y.size() < delta { y.expand(delta) } else { y },
            z: if z.size() < delta { z.expand(delta) } else { z },
        }
    }

    pub fn axis_interval(&self, axis: i32) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!()
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<Interval> {
        let origin = ray.origin();
        let direction_inverse = ray.direction_inverse();

        let mut t_min = ray_t.min();
        let mut t_max = ray_t.max();

        for axis in 0..=2 {
            let interval = self.axis_interval(axis);

            let ad_inverse = direction_inverse.axis(axis);

            let axis_value = origin.axis(axis);

            let t0 = (interval.min() - axis_value) * ad_inverse;
            let t1 = (interval.max() - axis_value) * ad_inverse;

            t_min = t0.max(t_min).min(t1.max(t_min));
            t_max = t0.min(t_max).max(t1.min(t_max));

            if t_min >= t_max {
                return None;
            }
        }

        Some(Interval::new(t_min, t_max))
    }

    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            return if self.x.size() > self.z.size() { 0 } else { 2 };
        }

        return if self.y.size() > self.z.size() { 1 } else { 2 };
    }
}
