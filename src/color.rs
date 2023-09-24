use crate::vec3::Vec3 as Color;
use std::io::Write;

pub fn write_color(file: &mut std::fs::File, pixel_color: Color) {
    let ir  =  (pixel_color.x * 255.999) as u8;
    let ig  = (pixel_color.y * 255.999) as u8;
    let ib = (pixel_color.z * 255.999) as u8;

    let str = format!("{} {} {}\n", ir,ig,ib);
    file.write(str.as_bytes()).expect("failed write");
}