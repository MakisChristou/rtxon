use crate::{hitable::{Hitable, HitRecord}, ray::Ray};

struct HitableList {
    objects: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(objects: Vec<Box<dyn Hitable>>) -> Self{
        HitableList{objects}
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
        let mut temp_rec: HitRecord = todo!();
        let mut hit_anything = false;
        let closest_so_far = t_max;

        for object in self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}