use std::rc::Rc;

use crate::{
    aabb::AxisAlignedBoundingBox,
    hitable::{HitRecord, Hitable},
    ray::Ray,
    vec3::Vec3,
};

pub struct HitableList {
    objects: Vec<Rc<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> Self {
        HitableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T: Hitable + 'static>(&mut self, hitable: T) {
        self.objects.push(Rc::new(hitable));
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::default();
        let mut rec = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec.clone());
            }
        }
        rec
    }

    fn bounding_box(&self, time: (f64, f64)) -> Option<AxisAlignedBoundingBox> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box =
            AxisAlignedBoundingBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut first_box = true;

        for object in &self.objects {
            if let Some(temp_box) = object.bounding_box(time) {
                output_box = if first_box {
                    temp_box
                } else {
                    AxisAlignedBoundingBox::surrounding_box(output_box, temp_box)
                };
                first_box = false
            }
        }
        Some(output_box)
    }
}
