use std::{fs::File, io::BufReader, sync::Arc};

use obj::{load_obj, Obj, Position};

use crate::{
    aabb::AABB,
    bvh::BVHNode,
    material::Material,
    quad::Quad,
    ray::{HitRecord, Hittable, Interval, Ray, WorldObject},
    vec::Vector3,
};

pub struct Mesh {
    triangles: Vec<Arc<WorldObject>>,
    node: BVHNode,
    bounding_box: AABB,
}

impl Mesh {
    pub fn from_file(path: String, material: Material) -> Self {
        let file = File::open(path).unwrap();

        let input = BufReader::new(file);
        let model: Obj<Position> = load_obj(input).unwrap();

        let vertices = model.vertices;

        let chunked_indices = model.indices.chunks_exact(3);

        let triangles: Vec<Arc<WorldObject>> = chunked_indices
            .map(|chunk| {
                let a = vertices[chunk[0] as usize];
                let b = vertices[chunk[1] as usize];
                let c = vertices[chunk[2] as usize];
                let a_vector = position_to_vector(a);
                let b_vector = position_to_vector(b);
                let c_vector = position_to_vector(c);
                Arc::new(WorldObject::Quad(Quad::new_triangle(
                    a_vector, b_vector, c_vector, material,
                )))
            })
            .collect();

        let node = BVHNode::new(triangles.clone());

        let bounding_box = node.bounding_box();

        Self {
            triangles,
            node,
            bounding_box,
        }
    }
}

impl Hittable for Mesh {
    fn bounding_box(&self) -> crate::aabb::AABB {
        self.bounding_box
    }

    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        self.node.hit(ray, ray_t)
    }
}

fn position_to_vector(position: Position) -> Vector3 {
    let array = position.position;
    Vector3::new(array[0].into(), array[1].into(), array[2].into())
}
