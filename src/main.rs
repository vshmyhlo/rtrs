use crate::sphere::Sphere;
mod camera;
use crate::camera::Camera;
mod utils;
use glam::{Vec2, Vec3};
mod interval;
use hittable::Hittable;
use image::{buffer::ConvertBuffer, ImageFormat, Rgb, Rgb32FImage, RgbImage};
mod hittable;
mod ray;
mod sphere;
use rand::{
    distributions::{
        uniform::{UniformFloat, UniformSampler},
        Standard, Uniform,
    },
    prelude::*,
};
use std::{fs::File, time};

type Color = Rgb<f32>;

fn main() -> Result<(), image::ImageError> {
    let mut world: Vec<Box<dyn Hittable + Sync>> = vec![];
    world.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    let camera = Camera::new(1920, 1080);

    let start = time::Instant::now();
    let image: RgbImage = camera.render(&world).convert();
    println!("render took {:?}", start.elapsed());

    let mut file = File::create("./image.png")?;
    image.write_to(&mut file, ImageFormat::Png)
}
