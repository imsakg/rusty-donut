#![allow(unused)]

use std::{
    thread,
    time::{self, SystemTime, UNIX_EPOCH},
};

mod error;
mod prelude;
mod utils;

struct Sdf {
    x: f64,
    y: f64,
    z: f64,
}

fn donut(x: f64, y: f64, z: f64) -> f64 {
    let radius = 0.4;
    let thickness = 0.3;
    (((x.powi(2) + y.powi(2)).sqrt() - radius).powi(2) + z.powi(2)).sqrt() - thickness / 2_f64
}

fn normal(x: f64, y: f64, z: f64) -> Sdf {
    let eps = 0.001;
    let n_x = donut(x + eps, y, z) - donut(x - eps, y, z);
    let n_y = donut(x, y + eps, z) - donut(x, y - eps, z);
    let n_z = donut(x, y, z + eps) - donut(x, y, z - eps);
    let norm = (n_x.powi(2) + n_y.powi(2) + n_z.powi(2)).sqrt();
    Sdf {
        x: n_x / norm,
        y: n_y / norm,
        z: n_z / norm,
    }
}

// fn sample(x:f64, f:32) -> String
fn sample(x: f64, y: f64) -> char {
    let mut z = -10_f64;
    for _step in 0..30 {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let tet = time as f64 / 1000.0 * 2_f64;

        let t_x = x * tet.cos() - z * tet.sin();
        let t_z = x * tet.sin() + z * tet.cos();
        let d = donut(t_x, y, t_z);
        if d <= 0.01 {
            let sdf = normal(t_x, y, t_z);
            let is_let = sdf.y < -0.15;
            let is_frosted = sdf.z < -0.5;
            if is_frosted {
                if is_let {
                    return '@';
                } else {
                    return '#';
                };
            } else if is_let {
                return '=';
            } else {
                return '.';
            }
        } else {
            z += d;
        }
    }
    ' '
}

fn main() {
    loop {
        let mut frame_chars = String::new();
        for y in 0..20 {
            for x in 0..80 {
                let remapped_x = x as f64 / 80.0 * 2.0 - 1.0;
                let remapped_y = (y as f64 / 20.0 * 2.0 - 1.0) * (2.0 * 20.0 / 80.0);
                let s = sample(remapped_x, remapped_y);
                frame_chars.push(s);
            }
            frame_chars.push('\n');
        }

        println!("\x1B[2J\x1B[1;1H{frame_chars}");

        thread::sleep(time::Duration::from_millis(1000 / 30));
    }
}
