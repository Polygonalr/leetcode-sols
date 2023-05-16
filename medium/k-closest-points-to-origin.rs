use std::collections::BinaryHeap;
use std::cmp::{Reverse, Ordering};

struct PointDistPair {
    coords: (i32, i32),
    dist: f64,
}

impl PartialEq for PointDistPair {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}

impl Eq for PointDistPair {}

impl PartialOrd for PointDistPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointDistPair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dist > other.dist {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Reverse<PointDistPair>> = BinaryHeap::new();
        for pt in points {
            let d: f64 = ((pt[0].pow(2) + pt[1].pow(2)) as f64).sqrt();
            heap.push(Reverse(PointDistPair {
                coords: (pt[0], pt[1]),
                dist: d
            }));
        }

        let mut ans: Vec<Vec<i32>> = vec![];
        for _ in 0..k {
            let pt = heap.pop().unwrap().0;
            ans.push(vec![pt.coords.0, pt.coords.1]);
        }
        ans
    }
}
