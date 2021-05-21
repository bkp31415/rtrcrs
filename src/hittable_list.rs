use std::sync::Arc;

use crate::{HitRecord, Hittable, Ray,Aabb};

/// Defines a data-structure to store all the Hittable objects.
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    /// Creates an empty object list.
    pub fn default() -> Self {
        Self { objects: vec![] }
    }

    /// Used to add a new instance of an Hittable object to the list.  
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    /// Used to clear all the Hittable objects of the corresponding Ray.
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    /// Provides a direct interface to run hit() on a list of [Hittable](Hittable) objects.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_anything = Some(temp_rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, output_box: &Aabb) -> Option<Aabb>{
        match self.objects.first() {
            Some(first) =>
                match first.bounding_box(output_box) {
                    Some(bbox) =>
                        self.list.iter().skip(1).try_fold(bbox, |acc, hitable|
                            match hitable.bounding_box(t0, t1) {
                                Some(bbox) => Some(aabb::surrounding_box(&acc, &bbox)),
                                _ => None
                            }
                        ),
                    _ => None
                },
            _ => None
        }
    }
}
