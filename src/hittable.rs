use crate::interval::Interval;
use crate::ray::Ray;
use glam::Vec3;

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable + Sync>> {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut closest = interval.max;
        let mut rec: Option<HitRecord> = None;

        for obj in self.iter() {
            if let Some(next_rec) = obj.hit(ray, Interval::new(interval.min, closest)) {
                closest = next_rec.t;
                rec = Some(next_rec);
            }
        }

        rec
    }
}

pub struct HitRecord {
    pub pos: Vec3,
    pub normal: Vec3,
    t: f32,
}

impl HitRecord {
    pub fn new(pos: Vec3, normal: Vec3, t: f32) -> HitRecord {
        HitRecord { pos, normal, t }
    }
}
