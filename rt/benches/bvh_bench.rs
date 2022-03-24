#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
//use std::time::Instant;
#[allow(unused_imports)]
use rand::Rng;
use rtlib;
use rtlib::hitlist::HitList;
#[allow(unused_imports)]
use rtlib::hittable::Hittable;
#[allow(unused_imports)]
use rtlib::prelude::Vec3;
#[allow(unused_imports)]
use rtlib::ray::Ray;
// how it's done in scopeguard
//#[macro_use(vect)] extern crate rtmacros;
// also works?
use rtmacros::vect;
use std::{
    io::{stderr, Write},
    sync::Arc,
};

use rayon::prelude::*;

use rtlib::bvh::Bvh;
use rtlib::camera::Camera;
#[allow(unused_imports)]
use rtlib::materials::{Dielectric, Lambertian, Metal};
#[allow(unused_imports)]
use rtlib::sphere::Sphere;
#[allow(unused_imports)]
use rtlib::util::{color, random_scene, write_color};
use rtlib::vec3::Color;

fn bench_random_scene_bvh() {
    // For error handling
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let mut rng = rand::thread_rng();

    // if you make it 2000x1000 that's 100x, and then 100 more samples of each,
    // and then test against all the objects again for differaction. And then
    // do a depth of 50.
    // NOTE: 2000x1000 takes about 40 minutes at this point
    // only 92s as of this commit. Run in release instead of debug.
    const SAMPLES_PER_PIXEL: i32 = 5; // number of anti-aliasing samples
    const MAX_DEPTH: i32 = 50;
    //let radian: f64 = (std::f64::consts::PI/4.0).cos();
    let vfov: f64 = 8.0;
    //let aspect: f64 = nx as f64 / ny as f64;
    const ASPECT_RATIO: f64 = 2.0 / 1.0; // eg 2000x1000, 800x400, 200x100
                                         // instead of setting the values and generating the aspect ratio,
                                         // we'll say how wide we want the overall image, and let the
                                         // ratio determine the height instead.
    const IMAGE_WIDTH: i32 = 200; // image width
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const NUM_PIXELS: i32 = IMAGE_WIDTH * IMAGE_HEIGHT;

    let look_from = vect!(25.0, 2.5, 5.0);
    let look_at = rtmacros::vect!(3.0, 0.75, 0.75);
    let vup = rtmacros::vect!(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = (look_from - look_at).length();
    const APERTURE: f64 = 0.2;

    //let mut world = HitList::new();
    // Chapter 12, what next?
    let start_time_in_sec: f64 = 0.0;
    let stop_time_in_sec: f64 = 0.0;
    let mut world = Arc::new(random_scene(&mut rng, false, false));
    let mut bvh = Bvh::new();
    bvh.add_hitlist(&mut world, start_time_in_sec, stop_time_in_sec);
    let world = Arc::new(bvh.build());

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        APERTURE,
        dist_to_focus,
        0.0,
        0.0,
    );

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let n_finished = Arc::new(std::sync::atomic::AtomicI32::new(0));

    // we use the riter because origin is at the lower left
    // to maintain a right handed coordinate system
    let pixels = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .map(move |j| {
            let world = world.clone();
            let n_finished = n_finished.clone();
            (0..IMAGE_WIDTH).into_par_iter().map(move |i| {
                let mut rng = rand::thread_rng();
                let mut pixel_color = Color::default();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += color(&r, world.as_ref(), MAX_DEPTH, &vect!(1, 1, 1));
                }

                let n = n_finished.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                if n % (NUM_PIXELS / 1000) == 0 || n == NUM_PIXELS - 1 {
                    eprint!(
                        "\rCalculated {}/{} pixels ({:.1?}%)",
                        n + 1,
                        NUM_PIXELS,
                        (n + 1) as f64 / NUM_PIXELS as f64 * 100.0,
                    );
                    stderr().flush().unwrap();
                }

                pixel_color
            })
        })
        .flatten();

    let mut pixel_vec = Vec::with_capacity(NUM_PIXELS as usize);
    pixel_vec.par_extend(pixels);

    eprintln!();

    for (i, pixel_color) in pixel_vec.into_iter().enumerate() {
        if i as i32 % (NUM_PIXELS / 1000) == 0 || i as i32 == NUM_PIXELS - 1 {
            let n = i * 1;
            eprint!(
                "\rWriting pixel {}/{} ({:.1?}%)",
                n,
                NUM_PIXELS,
                n as f64 / NUM_PIXELS as f64 * 100.0,
            );
            stderr().flush().unwrap();
        }

        write_color(&mut handle, pixel_color, SAMPLES_PER_PIXEL).unwrap_or_else(|err| {
            panic!(
                "Oops, error {} saving color {} for pixel {}/{}",
                err,
                pixel_color,
                i + 1,
                NUM_PIXELS
            )
        })
    }

    eprintln!();
    eprintln!("Done");
}

fn bench_random_scene_no_bvh() {
    // For error handling
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let mut rng = rand::thread_rng();

    // if you make it 2000x1000 that's 100x, and then 100 more samples of each,
    // and then test against all the objects again for differaction. And then
    // do a depth of 50.
    // NOTE: 2000x1000 takes about 40 minutes at this point
    // only 92s as of this commit. Run in release instead of debug.
    const SAMPLES_PER_PIXEL: i32 = 5; // number of anti-aliasing samples
    const MAX_DEPTH: i32 = 50;
    //let radian: f64 = (std::f64::consts::PI/4.0).cos();
    let vfov: f64 = 8.0;
    //let aspect: f64 = nx as f64 / ny as f64;
    const ASPECT_RATIO: f64 = 2.0 / 1.0; // eg 2000x1000, 800x400, 200x100
                                         // instead of setting the values and generating the aspect ratio,
                                         // we'll say how wide we want the overall image, and let the
                                         // ratio determine the height instead.
    const IMAGE_WIDTH: i32 = 200; // image width
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const NUM_PIXELS: i32 = IMAGE_WIDTH * IMAGE_HEIGHT;

    let look_from = vect!(25.0, 2.5, 5.0);
    let look_at = rtmacros::vect!(3.0, 0.75, 0.75);
    let vup = rtmacros::vect!(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = (look_from - look_at).length();
    const APERTURE: f64 = 0.2;

    //let mut world = HitList::new();
    // Chapter 12, what next?
    let start_time_in_sec: f64 = 0.0;
    let stop_time_in_sec: f64 = 0.0;
    let world: Arc<HitList>;
    world = Arc::new(random_scene(&mut rng, false, false));

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        APERTURE,
        dist_to_focus,
        start_time_in_sec,
        stop_time_in_sec,
    );

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let n_finished = Arc::new(std::sync::atomic::AtomicI32::new(0));

    // we use the riter because origin is at the lower left
    // to maintain a right handed coordinate system
    let pixels = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .map(move |j| {
            let world = world.clone();
            let n_finished = n_finished.clone();
            (0..IMAGE_WIDTH).into_par_iter().map(move |i| {
                let mut rng = rand::thread_rng();
                let mut pixel_color = Color::default();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += color(&r, world.as_ref(), MAX_DEPTH, &vect!(1, 1, 1));
                }

                let n = n_finished.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                if n % (NUM_PIXELS / 1000) == 0 || n == NUM_PIXELS - 1 {
                    eprint!(
                        "\rCalculated {}/{} pixels ({:.1?}%)",
                        n + 1,
                        NUM_PIXELS,
                        (n + 1) as f64 / NUM_PIXELS as f64 * 100.0,
                    );
                    stderr().flush().unwrap();
                }

                pixel_color
            })
        })
        .flatten();

    let mut pixel_vec = Vec::with_capacity(NUM_PIXELS as usize);
    pixel_vec.par_extend(pixels);

    eprintln!();

    for (i, pixel_color) in pixel_vec.into_iter().enumerate() {
        if i as i32 % (NUM_PIXELS / 1000) == 0 || i as i32 == NUM_PIXELS - 1 {
            let n = i * 1;
            eprint!(
                "\rWriting pixel {}/{} ({:.1?}%)",
                n,
                NUM_PIXELS,
                n as f64 / NUM_PIXELS as f64 * 100.0,
            );
            stderr().flush().unwrap();
        }

        write_color(&mut handle, pixel_color, SAMPLES_PER_PIXEL).unwrap_or_else(|err| {
            panic!(
                "Oops, error {} saving color {} for pixel {}/{}",
                err,
                pixel_color,
                i + 1,
                NUM_PIXELS
            )
        })
    }

    eprintln!();
    eprintln!("Done");
}

fn bench_bvh_non_bvh(c: &mut Criterion) {
    let mut group = c.benchmark_group("bvh");
    //group.sampling_mode(SamplingMode::Flat);
    // 7.x minutes for the non-bvh, 2 minutes for the bvh version. ~= 10minutes per run, 10
    //   samples. == 100 minutes or shy of 2 hours to run
    //reduced the samples per pixel by 100x and depth by 100x, and size by 10x, and
    //now it's telling me it runs in picoseconds. So upped depth from 5 to 50 and trying again.
    //group.measurement_time(std::time::Duration::new(7200, 0)).sample_size(10);
    group.sample_size(10);

    for i in 0..10 {
        group.bench_function(BenchmarkId::new("Random Scene non-BVH", i), |b| {
            //ok, the iters is "What" iteration you're on, not how many to do
            b.iter(|| {
                //let start = Instant::now();
                black_box(bench_random_scene_no_bvh());
                //start.elapsed()
            })
        });

        group.bench_function(BenchmarkId::new("Random Scene BVH", i), |b| {
            b.iter(|| {
                //let start = Instant::now();
                black_box(bench_random_scene_bvh());
                //start.elapsed()
            })
        });
    }
}

criterion_group!(benches, bench_bvh_non_bvh);
criterion_main!(benches);
