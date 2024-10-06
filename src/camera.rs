use std::sync::Mutex;

use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::{random_on_hemisphere, random_unit_vector};
use glam::Vec3;
use image::{buffer::ConvertBuffer, ImageFormat, Rgb, Rgb32FImage, RgbImage};
use rayon::prelude::*;

pub struct Camera {
    width: u32,
    height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        Camera {
            width,
            height,
            samples_per_pixel: 100,
            max_depth: 50,
        }
    }

    pub fn render<H>(&self, world: &H) -> Rgb32FImage
    where
        H: Hittable + Sync,
    {
        let image_width = self.width as f32;
        let image_height = self.height as f32;

        let camera_center = Vec3::new(0., 0., 0.);
        let focal_length = 1.0; // distance between camera center and viewport
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width / image_height);

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // TODO: add or sub focal?
        let viewport_upper_left =
            camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let xy = (0..self.width)
            .into_par_iter()
            .flat_map(|x| (0..self.height).into_par_iter().map(move |y| (x, y)));

        let pixels = xy.map(|(x, y)| {
            let pixel_center =
                pixel00_loc + (x as f32 * pixel_delta_u) + (y as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = self.ray_color(&ray, world, self.max_depth);
            (x, y, color)
        });

        let mutex = Mutex::new(Rgb32FImage::new(self.width, self.height));

        pixels.for_each(|(x, y, color)| {
            let mut image = mutex.lock().unwrap();
            image.put_pixel(x, y, Rgb([color.x, color.y, color.z]));
        });

        mutex.into_inner().unwrap()
    }

    fn ray_color<H>(&self, ray: &Ray, world: &H, depth: u32) -> Vec3
    where
        H: Hittable,
    {
        if depth == 0 {
            return Vec3::ZERO;
        }

        if let Some(rec) = world.hit(ray, Interval::new(1e-3, f32::INFINITY)) {
            let direction = random_on_hemisphere(rec.normal);
            return 0.5 * self.ray_color(&Ray::new(rec.pos, direction), world, depth - 1);
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        return (1.0 - a) * Vec3::new(1., 1., 1.) + a * Vec3::new(0.5, 0.7, 1.0);
    }
}
