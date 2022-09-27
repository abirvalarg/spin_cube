use std::{time::{Duration, Instant}, sync::Arc, sync::Mutex, f32::consts::{PI, TAU}};

use math::{Lin, Surface};
use ultraviolet::{Vec4, Mat4, Vec3};

mod term;
mod math;

const CHARS: [char; 8] = [' ', '.', ',', ':', 'i', 'l', 'w', 'W'];

fn main() {
    let running = Arc::new(Mutex::new(true));
    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            let mut running = running.lock().unwrap();
            if *running {
                *running = false;
            } else {
                std::process::exit(130);
            }
        }).unwrap();
    }

    let cube: [[Vec4; 3]; 12] = [
        [
            [-1., -1., 1., 1.].into(),
            [1., -1., 1., 1.].into(),
            [-1., 1., 1., 1.].into()
        ],
        [
            [1., -1., 1., 1.].into(),
            [1., 1., 1., 1.].into(),
            [-1., 1., 1., 1.].into()
        ],
        [
            [1., -1., 1., 1.].into(),
            [1., -1., -1., 1.].into(),
            [1., 1., 1., 1.].into()
        ],
        [
            [1., -1., -1., 1.].into(),
            [1., 1., -1., 1.].into(),
            [1., 1., 1., 1.].into()
        ],
        [
            [1., 1., -1., 1.].into(),
            [1., -1., -1., 1.].into(),
            [-1., -1., -1., 1.].into()
        ],
        [
            [-1., 1., -1., 1.].into(),
            [1., 1., -1., 1.].into(),
            [-1., -1., -1., 1.].into(),
        ],
        [
            [-1., -1., -1., 1.].into(),
            [-1., -1., 1., 1.].into(),
            [-1., 1., -1., 1.].into(),
        ],
        [
            [-1., 1., 1., 1.].into(),
            [-1., 1., -1., 1.].into(),
            [-1., -1., 1., 1.].into(),
        ],
        [
            [-1., 1., 1., 1.].into(),
            [1., 1., 1., 1.].into(),
            [-1., 1., -1., 1.].into(),
        ],
        [
            [-1., 1., -1., 1.].into(),
            [1., 1., 1., 1.].into(),
            [1., 1., -1., 1.].into(),
        ],
        [
            [-1., -1., -1., 1.].into(),
            [1., -1., -1., 1.].into(),
            [-1., -1., 1., 1.].into(),
        ],
        [
            [-1., -1., 1., 1.].into(),
            [1., -1., -1., 1.].into(),
            [1., -1., 1., 1.].into(),
        ]
    ];

    let delay = Duration::from_secs_f32(1. / 15.);
    let mut start_time;
    let mut rotation_y = 0.;
    let mut rotation_x = 0.;
    let cam = Mat4::from_translation(ultraviolet::Vec3::new(0., 0., -5.));
    let light = Vec3::new(-0.4, -0.4, -1.).normalized();

    while *running.lock().unwrap() {
        start_time = Instant::now();
        let size = term::TermSize::get();
        let mut screen = Vec::with_capacity(size.height as usize);
        screen.resize_with(size.height as usize, || {
            let mut res = Vec::with_capacity(size.width as usize);
            res.resize(size.width as usize, (0.0_f32, 1.0_f32));
            res
        });

        let proj = ultraviolet::projection::perspective_gl(PI / 3., size.width as f32 / size.height as f32, 0.1, 100.);

        let rot_mat = Mat4::from_rotation_y(rotation_y) * Mat4::from_rotation_x(rotation_x);

        let cube = cube.map(|triangle| triangle.map(|vert| proj * cam * rot_mat * vert));

        for triangle in cube {
            let v1: Vec3 = (triangle[1] - triangle[0]).into();
            let v2: Vec3 = (triangle[2] - triangle[0]).into();
            let n = v1.cross(v2);
            let n_len = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
            let cos = -light.dot(n) / n_len;

            raster(&mut screen, triangle, (cos + 1.) / 2.);
        }

        for (idx, line) in screen.into_iter().enumerate() {
            if idx > 0 {
                term::put('\n');
            }
            for (brightness, _depth) in line {
                let idx = (brightness * 7.).floor() as usize;
                term::put(CHARS[idx]);
            }
            term::flush();
        }
        rotation_y += PI / 30.;
        if rotation_y >= TAU {
            rotation_y -= TAU;
        }
        rotation_x += PI / 60.;
        if rotation_x >= TAU {
            rotation_x -= TAU;
        }
        let elapsed = start_time.elapsed();
        if elapsed < delay {
            let time_left = delay - elapsed;
            std::thread::sleep(time_left);
        }
    }
}

fn raster(screen: &mut Vec<Vec<(f32, f32)>>, triangle: [Vec4; 3], val: f32) {
    let mut triangle = triangle.map(|x| {
        let res = x / x[3];
        <Vec4 as Into<Vec3>>::into(res)
    });
    triangle.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
    let screen_coords = triangle.map(|coords|
        [(coords[0] + 1.) / 2. * screen[0].len() as f32, (1. - coords[1]) / 2. * screen.len() as f32]
    );
    let f1 = Lin::from((screen_coords[0], screen_coords[2]));
    let f2 = Lin::from((screen_coords[0], screen_coords[1]));
    let f3 = Lin::from((screen_coords[1], screen_coords[2]));
    let surf = Surface::from(triangle);
    for x in (screen_coords[0][0].round() as usize)..(screen_coords[2][0].round() as usize) {
        let y1 = f1.at(x as f32);
        let y2 = if (x as f32) < screen_coords[1][0] {
            f2.at(x as f32)
        } else {
            f3.at(x as f32)
        };
        for y in (y1.min(y2).round() as usize)..(y1.max(y2).round() as usize) {
            let real_x = x as f32 / screen[0].len() as f32 * 2. - 1.;
            let real_y = 1. - (y as f32 / screen.len() as f32 * 2.);
            let z = surf.at_x_y(real_x, real_y);
            if y >= screen.len() {
                break;
            }
            if z < 1. && z < screen[y][x].1 {
                screen[y][x] = (val, z);
            }
        }
    }
}
