use glam::Vec3;
use image::{ImageFormat, Rgb, Rgb32FImage, RgbImage};
use std::fs::File;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl  Ray {

} {
    
}

type RgbF32 = Rgb<f32>;

fn main() -> Result<(), image::ImageError> {
    let mut image = Rgb32FImage::new(1920, 1080);

    for x in 0..1920 {
        for y in 0..1080 {
            let ray = Ray::new(camera_center, ray_direction);
            let pixel = ray_color(&ray);
            image.put_pixel(x, y, pixel);
        }
    }

    let mut file = File::create("./image.png")?;
    image.write_to(&mut file, ImageFormat::Png)
}

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

fn hit_sphere(sphere: &Sphere, ray: &Ray) -> bool {
    let oc = sphere.center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = -2.0 * ray.direction.dot(oc);
    let c = oc.dot(oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(ray: &Ray) -> RgbF32 {
    let sphere = Sphere::new(Vec3::new(0., 0., -1.), 0.5);
    if hit_sphere(&sphere, ray) {
        return Rgb([1., 0., 0.]);
    }

    Rgb([0., 0., 0.])
}
