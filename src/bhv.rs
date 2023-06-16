use std::{cmp::Ordering, rc::Rc};

use crate::{
    aabb::AxisAlignedBoundingBox,
    hitable::{HitRecord, Hitable},
    utils::random_int,
};

pub struct BHVNode {
    left: Rc<dyn Hitable>,
    right: Rc<dyn Hitable>,
    some_box: AxisAlignedBoundingBox,
}

impl BHVNode {
    // Build the BHV structure in the constructor
    pub fn new(
        mut src_objects: Vec<Rc<dyn Hitable>>,
        range: (usize, usize),
        time: (f64, f64),
    ) -> Self {
        let axis = random_int(0, 3);
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => {
                panic!("Axis can only be 0, 1 or 2")
            }
        };

        let object_span = range.1 - range.0;

        let mut left;
        let mut right;

        if object_span == 1 {
            left = Rc::clone(&src_objects[range.0]);
            right = Rc::clone(&src_objects[range.0]);
        } else if object_span == 2 {
            if comparator(
                &Rc::clone(&src_objects[range.0]),
                &Rc::clone(&src_objects[range.0 + 1]),
            ) == Ordering::Less
            {
                left = Rc::clone(&src_objects[range.0]);
                right = Rc::clone(&src_objects[range.0 + 1])
            } else {
                left = Rc::clone(&src_objects[range.0 + 1]);
                right = Rc::clone(&src_objects[range.0])
            }
        } else {
            src_objects[range.0..range.1].sort_by(comparator);
            let mid = range.0 + object_span / 2;
            left = Rc::new(BHVNode::new(src_objects.clone(), (range.0, mid), time));
            right = Rc::new(BHVNode::new(src_objects, (mid, range.1), time));
        }

        match left.bounding_box(time) {
            Some(box_left) => match right.bounding_box(time) {
                Some(box_right) => BHVNode {
                    left,
                    right,
                    some_box: AxisAlignedBoundingBox::surrounding_box(box_left, box_right),
                },
                None => panic!("No bounding box in bvh_node constructor"),
            },
            None => {
                panic!("No bounding box in bvh_node constructor")
            }
        }
    }

    fn box_compare(a: &Rc<dyn Hitable>, b: &Rc<dyn Hitable>, axis: usize) -> Ordering {
        match a.bounding_box((0.0, 0.0)) {
            None => panic!("No bounding box in bvh_node constructor"),
            Some(box_a) => match b.bounding_box((0.0, 0.0)) {
                None => panic!("No bounding box in bvh_node constructor"),
                Some(box_b) => {
                    if box_a.minimum.at(axis) < box_b.minimum.at(axis) {
                        Ordering::Less
                    } else if box_a.minimum.at(axis) > box_b.minimum.at(axis) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
            },
        }
    }

    fn box_x_compare(a: &Rc<dyn Hitable>, b: &Rc<dyn Hitable>) -> Ordering {
        return Self::box_compare(a, b, 0);
    }

    fn box_y_compare(a: &Rc<dyn Hitable>, b: &Rc<dyn Hitable>) -> Ordering {
        return Self::box_compare(a, b, 1);
    }

    fn box_z_compare(a: &Rc<dyn Hitable>, b: &Rc<dyn Hitable>) -> Ordering {
        return Self::box_compare(a, b, 2);
    }
}

impl Hitable for BHVNode {
    // Recusive hit implementation
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.some_box.hit(r, t_min, t_max) {
            Some(_hit_record) => {
                let hit_left = self.left.hit(r, t_min, t_max);

                let t_max_right = if let Some(left_rec) = hit_left.clone() {
                    left_rec.t
                } else {
                    t_max
                };

                let hit_right = self.right.hit(r, t_min, t_max_right);

                if let Some(left_rec) = hit_left {
                    return Some(left_rec);
                }

                if let Some(right_rec) = hit_right {
                    return Some(right_rec);
                }

                return None;
            }
            None => {
                return None;
            }
        }
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AxisAlignedBoundingBox> {
        Some(self.some_box.clone())
    }
}
