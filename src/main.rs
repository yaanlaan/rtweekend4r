use std::sync::Arc;
use trweekend4r::{
    Camera, Color, Dielectric, HittableList, Lambertian, Metal, Point3, Sphere, Vec3,
};

fn main() -> std::io::Result<()> {
    // 创建世界
    let mut world = HittableList::new();

    // 创建地面材质
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // 创建随机小球
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = trweekend4r::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * trweekend4r::random_double(),
                0.2,
                b as f64 + 0.9 * trweekend4r::random_double(),
            );

            // 避免在固定位置创建球体
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn trweekend4r::Material>;

                if choose_mat < 0.8 {
                    // 漫反射（80%）
                    let albedo = Vec3::random() * Vec3::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // 金属（15%）
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = trweekend4r::random_double_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    // 玻璃（5%）
                    sphere_material = Arc::new(Dielectric::new(1.5));
                }

                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // 三个大球体
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // 创建并配置摄像机
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    // 渲染
    camera.render(&world)
}
