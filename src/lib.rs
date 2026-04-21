use rand::Rng;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use camera::Camera;
pub use color::{Color, write_color};
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use interval::Interval;
pub use material::{Dielectric, Lambertian, Material, Metal};
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Point3, Vec3};

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
