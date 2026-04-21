use crate::{interval, vec3::Vec3};
use std::io::{Write};
pub type Color = Vec3;

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> std::io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // gamma 校正
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // 将 [0,1] 映射到字节范围 [0,255]
    let interval_ = interval::Interval::new(0.0, 0.999);
    
    let rbyte = (255.999 * interval_.clamp(r)) as i32;
    let gbyte = (255.999 * interval_.clamp(g)) as i32;
    let bbyte = (255.999 * interval_.clamp(b)) as i32;

    // 写入像素颜色分量
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}

