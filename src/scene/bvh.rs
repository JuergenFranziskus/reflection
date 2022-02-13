use std::cmp::Ordering;
use crate::aabb::AABB;
use crate::Float;
use crate::randomness::Randomness;
use crate::ray::Ray;
use crate::scene::primitive::PrimitiveRef;


pub struct BVH {
    nodes: Vec<BVHNode>,
}
impl BVH {
    pub fn new<R: Randomness>(primitives: &mut [(PrimitiveRef, AABB)], rng: &mut R) -> Self {
        if primitives.len() == 0 {
            return Self {
                nodes: Vec::new()
            }
        }

        let mut nodes = Vec::new();
        BVHNode::build_stupid(primitives, &mut nodes, rng);

        let mut ret = Self {
            nodes,
        };
        ret.reverse();
        ret
    }

    pub fn top(&self) -> AABB {
        self.nodes[0].aabb()
    }
    pub fn reverse(&mut self) {
        let len = self.nodes.len();

        for node in &mut self.nodes {
            node.reverse(len)
        }

        self.nodes.reverse();
    }

    pub fn find_intersection<F, C, O>(&self, ray: &Ray, find: F, comp: C, t_min: Float, t_max: Float) -> Option<O>
        where F: Fn(&Ray, PrimitiveRef) -> Option<O>,
              C: Fn(O, O) -> O {
        if self.nodes.len() == 0 {
            return None;
        }

        let start = 0;
        self.find_intersection_rec(ray, &find, &comp, start, t_min, t_max)
    }
    fn find_intersection_rec<F, C, O>(&self, ray: &Ray, find: &F, comp: &C, node: usize, t_min: Float, t_max: Float) -> Option<O>
        where F: Fn(&Ray, PrimitiveRef) -> Option<O>,
              C: Fn(O, O) -> O {
        match &self.nodes[node] {
            BVHNode::Leaf { primitive, .. } => {
                find(ray, *primitive)
            }
            BVHNode::Binary { aabb, left, right } => {
                if aabb.intersects_ray(ray, t_min, t_max) {
                    let l_int = self.find_intersection_rec(ray, find, comp, *left, t_min, t_max);
                    let r_int = self.find_intersection_rec(ray, find, comp, *right, t_min, t_max);

                    match (l_int, r_int) {
                        (None, None) => None,
                        (Some(i), None) => Some(i),
                        (None, Some(i)) => Some(i),
                        (Some(a), Some(b)) => Some(comp(a, b))
                    }
                } else {
                    None
                }
            }
        }
    }
}

enum BVHNode {
    Leaf {
        aabb: AABB,
        primitive: PrimitiveRef,
    },
    Binary {
        aabb: AABB,
        left: usize,
        right: usize,
    },
}
impl BVHNode {
    fn aabb(&self) -> AABB {
        match self {
            Self::Leaf { aabb, .. } => *aabb,
            Self::Binary { aabb, .. } => *aabb,
        }
    }

    fn reverse(&mut self, len: usize) {
        match self {
            Self::Binary { left, right, .. } => {
                *left = len - *left - 1;
                *right = len - *right - 1;
            }
            _ => (),
        }
    }

    fn build_stupid<R: Randomness>(primitives: &mut [(PrimitiveRef, AABB)], nodes: &mut Vec<Self>, rng: &mut R) -> usize {
        assert_ne!(primitives.len(), 0);


        if primitives.len() == 1 {
            let i = nodes.len();

            nodes.push(Self::Leaf {
                aabb: primitives[0].1,
                primitive: primitives[0].0,
            });
            i
        } else {
            let axis = rng.usize_range_exclusive(0, 3);
            primitives.sort_by(|a, b| a.1.min[axis].partial_cmp(&b.1.min[axis]).unwrap_or(Ordering::Equal));
            let mid = primitives.len() / 2;

            let (left, right) = primitives.split_at_mut(mid);

            let left = Self::build_stupid(left, nodes, rng);
            let right = Self::build_stupid(right, nodes, rng);

            let l_aabb = nodes[left].aabb();
            let r_aabb = nodes[right].aabb();

            let aabb = AABB::merged(l_aabb, r_aabb);
            let i = nodes.len();
            nodes.push(Self::Binary {
                aabb,
                left,
                right,
            });
            i
        }
    }
}