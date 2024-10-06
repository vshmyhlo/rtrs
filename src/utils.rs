use glam::Vec3;
use rand;
use rand::{
    distributions::{
        uniform::{UniformFloat, UniformSampler},
        Standard, Uniform,
    },
    prelude::*,
};

pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let p: Vec3 = rng.sample(Standard);
    return p.normalize();
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        // In the same hemisphere as the normal
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}
