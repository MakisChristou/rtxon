use crate::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
};

pub struct HitableList {
    objects: Vec<Box<dyn Hitable>>,
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
        self.objects.push(Box::new(hitable));
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
}
