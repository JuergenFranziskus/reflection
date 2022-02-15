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
    pub fn new<R: Randomness>(primitives: &mut [(PrimitiveRef, AABB)], _rng: &mut R) -> Self {
        if primitives.len() == 0 {
            return Self {
                nodes: Vec::new()
            }
        }

        let mut nodes = Vec::new();
        BVHNode::build_sah(primitives, &mut nodes);

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

    fn choose_split_axis(primitives: &[(PrimitiveRef, AABB)]) -> (Axis, Float, Float) {
        let mut min_x = Float::INFINITY;
        let mut max_x = -Float::INFINITY;
        let mut min_y = Float::INFINITY;
        let mut max_y = -Float::INFINITY;
        let mut min_z = Float::INFINITY;
        let mut max_z = -Float::INFINITY;

        for (_, aabb) in primitives {
            let centroid = aabb.centroid();

            min_x = min_x.min(centroid[0]);
            max_x = max_x.max(centroid[0]);
            min_y = min_y.min(centroid[1]);
            max_y = max_y.max(centroid[1]);
            min_z = min_z.min(centroid[2]);
            max_z = max_z.max(centroid[2]);
        }

        let extent_x = max_x - min_x;
        let extent_y = max_y - min_y;
        let extent_z = max_z - min_z;


        if extent_x.abs() > extent_y.abs() && extent_x.abs() > extent_z.abs() {
            (Axis::X, min_x, extent_x)
        }
        else if extent_y.abs() > extent_z.abs() {
            (Axis::Y, min_y, extent_y)
        }
        else {
            (Axis::Z, min_z, extent_z)
        }
    }

    #[allow(dead_code)]
    fn build_stupid(primitives: &mut [(PrimitiveRef, AABB)], nodes: &mut Vec<Self>) -> usize {
        assert_ne!(primitives.len(), 0);


        if primitives.len() == 1 {
            let i = nodes.len();

            nodes.push(Self::Leaf {
                aabb: primitives[0].1,
                primitive: primitives[0].0,
            });
            i
        } else {
            let axis = Self::choose_split_axis(primitives).0.to_index();
            primitives.sort_by(|a, b| a.1.min[axis].partial_cmp(&b.1.min[axis]).unwrap_or(Ordering::Equal));
            let mid = primitives.len() / 2;

            let (left, right) = primitives.split_at_mut(mid);

            let left = Self::build_stupid(left, nodes);
            let right = Self::build_stupid(right, nodes);

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
    #[allow(dead_code)]
    fn build_sah(primitives: &mut [(PrimitiveRef, AABB)], nodes: &mut Vec<Self>) -> usize {
        assert_ne!(primitives.len(), 0);

        if primitives.len() == 1 {
            let i = nodes.len();
            nodes.push(BVHNode::Leaf { primitive: primitives[0].0, aabb: primitives[0].1 });
            i
        }
        else {
            let (axis, min, extent) = Self::choose_split_axis(primitives);
            let bucket_amount = (primitives.len() as Float).log2().ceil() as usize * 2;

            let buckets = Self::build_buckets(primitives, bucket_amount, min, extent, axis);

            let total_bounds = Self::compute_total_bounds(&buckets);
            let costs = Self::compute_costs(&buckets, &total_bounds);
            let bucket_split_i = Self::compute_bucket_split_i(&costs);
            let dividing_line = buckets[bucket_split_i].dividing_line;

            let axis = axis.to_index();
            primitives.sort_by(|(_, a), (_, b)| a.centroid()[axis].partial_cmp(&b.centroid()[axis]).unwrap());
            let split_i = Self::compute_primitive_split_i(primitives, dividing_line, axis);


            let (left, right) = primitives.split_at_mut(split_i);

            let left = Self::build_sah(left, nodes);
            let right = Self::build_sah(right, nodes);

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

    fn build_buckets(primitives: &[(PrimitiveRef, AABB)], bucket_amount: usize, min: Float, extent: Float, axis: Axis) -> Vec<Bucket> {
        let mut buckets = Vec::with_capacity(bucket_amount);

        let step = extent / bucket_amount as Float;

        let dividing_lines: Vec<Float> = (0..bucket_amount)
            .map(|i| min + step * i as Float)
            .collect();


        for i in 0..bucket_amount {
            let my_line = dividing_lines[i];

            let in_bucket: Vec<usize> = (0..primitives.len())
                .filter(|&p_i| {
                    let centroid = primitives[p_i].1.centroid();
                    let pos = centroid[axis.to_index()];

                    if i == bucket_amount - 1 {
                        pos >= my_line
                    }
                    else {
                        let next_line = dividing_lines[i + 1];
                        pos >= my_line && pos < next_line
                    }
                })
                .collect();

            let count = in_bucket.len();
            if count != 0 {
                let mut aabbs = in_bucket.into_iter()
                    .map(|i| primitives[i].1);

                let init = aabbs.next().unwrap();
                let aabb = aabbs.fold(init, |a, b| AABB::merged(a, b));


                buckets.push(Bucket {
                    amount: count,
                    dividing_line: my_line,
                    aabb,
                });
            }
        }

        buckets
    }
    fn compute_total_bounds(buckets: &[Bucket]) -> AABB {
        let total_bounds_init: AABB = buckets[0].aabb;
        let total_bounds: AABB = buckets.iter().skip(1)
            .map(|b| b.aabb)
            .fold(total_bounds_init, |a, b| AABB::merged(a, b));

        total_bounds
    }
    fn compute_costs(buckets: &[Bucket], total_bounds: &AABB) -> Vec<Float> {
        let mut costs = Vec::with_capacity(buckets.len() - 1);
        for i in 1..buckets.len() {
            let mut b0 = buckets[0].aabb;
            let mut b1 = buckets.last().unwrap().aabb;
            let mut count0 = 0;
            let mut count1 = 1;

            for j in 0..i {
                b0 = AABB::merged(b0, buckets[j].aabb);
                count0 += buckets[j].amount;
            }
            for j in i..buckets.len() {
                b1 = AABB::merged(b1, buckets[j].aabb);
                count1 += buckets[j].amount;
            }

            let cost = 0.125 + (count0 as Float * b0.surface_area() + count1 as Float * b1.surface_area()) / total_bounds.surface_area();
            costs.push(cost);
        }

        costs
    }
    fn compute_bucket_split_i(costs: &[Float]) -> usize {
        let mut min_cost = costs[0];
        let mut split_i = 0;

        for i in 1..costs.len() {
            if costs[i] < min_cost {
                min_cost = costs[i];
                split_i = i;
            }
        }


        split_i + 1
    }
    fn compute_primitive_split_i(primitives: &[(PrimitiveRef, AABB)], dividing_line: Float, axis: usize) -> usize {
        for i in 1..primitives.len() {
            let my_pos = primitives[i].1.centroid()[axis];
            let last_pos = primitives[i - 1].1.centroid()[axis];

            if my_pos >= dividing_line && last_pos < dividing_line {
                return i;
            }
        }

        unreachable!()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Bucket {
    amount: usize,
    dividing_line: Float,
    aabb: AABB,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Axis {
    X,
    Y,
    Z
}
impl Axis {
    fn to_index(&self) -> usize {
        match self {
            Self::X => 0,
            Self::Y => 1,
            Self::Z => 2,
        }
    }
}
