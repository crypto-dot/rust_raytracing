mod vec3;
mod color;
use crate::color::write_color;
use std::io::Write;
use ansi_term::{Style, Color::{Red, Green, Blue, Yellow, Purple, Cyan, White, Black}};
use std::fs::File;
use env_logger;
use chrono_tz::America::Chicago;
use log::info;
fn main() {
    initialize_logger();
    let image_width: u16 = 256;
    let image_height: u16 = 256;
    let mut file  = File::create("image.ppm").expect("error creating file");
    let str = format!("P3\n{} {}\n255\n", image_width,image_height);
    file.write(str.as_bytes()).expect("failed write");

    for x in 0..image_height {
        info!("Scan lines remaining: {image_height}", image_height = image_height - x);
        for y in 0..image_width {
            let r = (y as f64) / (image_width) as f64;
            let g = (x as f64) / (image_height) as f64;
            let b = 0 as f64;

            let pixel_color = vec3::Vec3 { x: r, y: g, z: b };
            write_color(&mut file, pixel_color);
        }


    }    
    info!("Done!");
}

fn initialize_logger () {
    env_logger::Builder::new().format(
        |buf, record| {
            let level = record.level();
            let styled_level = match level {
                log::Level::Error => Style::new().fg(Red).bold().paint(level.to_string()),
                log::Level::Warn => Style::new().fg(Yellow).bold().paint(level.to_string()),
                log::Level::Info => Style::new().fg(Green).bold().paint(level.to_string()),
                log::Level::Debug => Style::new().fg(Blue).bold().paint(level.to_string()),
                log::Level::Trace => Style::new().fg(Purple).bold().paint(level.to_string())
            };
            let datetime = chrono::Local::now().with_timezone(&Chicago);
            write!(buf, "{} [", datetime)?;
            buf.write_all(styled_level.to_string().as_bytes())?; 
            writeln!(buf, "] - {}", record.args())
        }
    )
    .filter(None, log::LevelFilter::Trace)
    .init();
}