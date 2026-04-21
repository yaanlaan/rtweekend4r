use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::f64::INFINITY;
use std::io::{self, Write};

pub struct Camera {
    pub aspect_ratio: f64, // 图像宽高比
    pub image_width: i32,  // 图像宽度
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,          // 垂直视场角（度数）
    pub lookfrom: Point3,   // 摄像机位置
    pub lookat: Point3,     // 看向的点
    pub vup: Vec3,          // 摄像机向上方向
    pub defocus_angle: f64, // 散焦角度（度数）
    pub focus_dist: f64,    // 焦点距离

    // 私有计算变量
    pixel_samples_scale: f64,
    image_height: i32,    // 图像高度
    center: Point3,       // 相机中心
    pixel00_loc: Point3,  // 像素 (0,0) 的位置
    pixel_delta_u: Vec3,  // 相邻像素水平间距
    pixel_delta_v: Vec3,  // 相邻像素垂直间距
    u: Vec3,              // 摄像机坐标系右向量
    v: Vec3,              // 摄像机坐标系上向量
    w: Vec3,              // 摄像机坐标系后向量
    defocus_disk_u: Vec3, // 散焦圆盘水平半径
    defocus_disk_v: Vec3, // 散焦圆盘竖直半径
}

impl Camera {
    pub fn new() -> Self {
        let samples_per_pixel = 100;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(1.0, 0.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
            w: Vec3::new(0.0, 0.0, 1.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) -> std::io::Result<()> {
        self.initialize();

        // 打印 PPM 格式头
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:3} ", self.image_height - j);
            io::stderr().flush()?;

            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world, self.max_depth);
                }
                crate::color::write_color(
                    &mut io::stdout(),
                    self.pixel_samples_scale * pixel_color,
                )?;
            }
        }
        eprint!("\rDone.                 \n");
        io::stderr().flush()?;
        Ok(())
    }

    fn initialize(&mut self) {
        // 计算图像高度，确保至少为 1
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.center = self.lookfrom;

        // 视口 (Viewport) 设置
        // 根据垂直视场角计算视口高度
        let theta = crate::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // 计算摄像机坐标系基向量
        self.w = Vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = Vec3::unit_vector(Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

        // 计算视口边沿向量（使用摄像机坐标系）
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * (-self.v);

        // 计算像素间距向量
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // 计算左上角像素位置
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // 计算散焦圆盘基向量
        let defocus_radius =
            self.focus_dist * (crate::degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec: HitRecord = Default::default();

        // 使用 Interval 来定义光线的有效范围
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation = Color::new(0.0, 0.0, 0.0);

            if let Some(mat) = &rec.mat {
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * self.ray_color(&scattered, world, depth - 1);
                }
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0) //背景
    }

    pub fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(
            crate::random_double() - 0.5,
            crate::random_double() - 0.5,
            0.0,
        )
    }
}
