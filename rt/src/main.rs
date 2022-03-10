//use clap::{arg, App, Command, AppSettings, SubCommand};
use clap::Command;
#[allow(unused_imports)]
use rand::Rng;
use rtlib;
// how it's done in scopeguard
//#[macro_use(vect)] extern crate rtmacros;
// also works?
use rtmacros::vect;
use std::{
    io::{stderr, Write},
    sync::Arc,
};

use rayon::prelude::*;

use rtlib::camera::Camera;
#[allow(unused_imports)]
use rtlib::materials::{Dielectric, Lambertian, Metal};
#[allow(unused_imports)]
use rtlib::sphere::Sphere;
#[allow(unused_imports)]
use rtlib::util::{color, random_scene, write_color};
use rtlib::vec3::Color;
use rtlib::bvh::Bvh;


fn main() {
    #[derive(Debug, Copy, Clone)]
    struct RenderInfo {
        depth: i32,
        #[allow(dead_code)]
        fast: bool,
        samples: i32,
        vfov: f32,
        width: i32,
        aperture: f32,
        start: f32,
        stop: f32,
    }

    let mut ri = RenderInfo {
        depth: 500,
        fast: false,
        samples: 500,
        vfov: 8.0,
        width: 2000,
        aperture: 0.2,
        start: 0.0,
        stop: 0.0,
    };

    let cmd = clap::Command::new("rt")
        .bin_name("rt").arg(
                clap::arg!(-f --fast "Set default values to help with adjusting scene.")
                .required(false)
            ).arg(
                clap::arg!(-d --max_depth <DEPTH> "Maximum depth to follow ray bounces. Default: 500")
                .required(false)
                .default_value("500")
                .validator(|s| s.parse::<i32>())
            ).arg(
                clap::arg!(-s --num_samples <SAMPLES> "Number of anti-aliasing samples per pixel. Default: 500")
                .required(false)
                .default_value("500")
                .validator(|s| s.parse::<i32>())
            ).arg(
                clap::arg!(-v --vfov <FOV> "Vertical FOV. Defaults: 8.0")
                .required(false)
                .default_value("8.0")
                .validator(|s| s.parse::<f32>())
            ).arg(
                clap::arg!(-w --image_width <WIDTH> "Image width. Height is calculated from this using a ratio. Default: 2000")
                .required(false)
                .default_value("2000")
                .validator(|s| s.parse::<i32>())
            ).arg(
                clap::arg!(-a --aperture <APERTURE> "Set the aperture to create impression of blur in foreground and background. Default: 0.2")
                .required(false)
                .default_value("0.2")
                .validator(|s| s.parse::<f32>())
            ).arg(
                clap::arg!(--start_time <START> "Time in seconds to start the render. If set, need to set stop_time as well. Default: 0.0")
                .required(false)
                .default_value("0.0")
                .validator(|s| s.parse::<f32>())
            ).arg(
                clap::arg!(--stop_time <STOP> "Time in seconds to stop the render. You should set start_time as well, but if not, will use start_time default. Default: 0.0")
                .required(false)
                .default_value("0.0")
                .validator(|s| s.parse::<f32>())
            )
        .subcommand_required(true)
        .subcommand(
            Command::new("random_scene")
            .about("Generates the cover of the RayTracing book. 3 orbs in center of picture, 100 spheres of random textures around. One large sphere for 'floor'.")
            );

    let matches = cmd.get_matches();

    println!("What we got on the cmdline {:?}", matches);

    // NOTE: If there is a default value for something, it'll always
    // show as "present", so we know we have settings for all of the
    // ones we have as default.
    ri.samples = matches.value_of_t("num_samples").expect("Number of samples is required.");
    println!("We got a number of samples of: {:?}", ri.samples);
    ri.depth = matches.value_of_t("max_depth").expect("Maximum depth is required.");
    println!("We got a max depth of: {:?}", ri.depth);
    ri.vfov = matches.value_of_t("vfov").expect("Vertical FOV is required.");
    println!("We got a vfov of: {:?}", ri.vfov);
    ri.width = matches.value_of_t("image_width").expect("Image width required.");
    println!("We got a width of: {:?}", ri.width);
    ri.aperture = matches.value_of_t("aperture").expect("Aperture required.");
    println!("We got a aperture of: {:?}", ri.aperture);
    ri.start = matches.value_of_t("start_time").expect("Start time required.");
    println!("We got a start time of: {:?} seconds", ri.start);
    ri.stop = matches.value_of_t("stop_time").expect("Stop time required.");
    println!("We got a stop time of: {:?} seconds", ri.stop);

    if matches.is_present("fast") {
       ri = RenderInfo{
            fast: true,
            depth: 5,
            samples: 5,
            vfov: ri.vfov,
            width: 200,
            aperture: ri.aperture,
            start: ri.start,
            stop: ri.stop,
       }; 
    }
    // make read only
    let ri = ri;

    println!("Render info after arg parsing. {:?}", ri);

    
    // For error handling
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let mut rng = rand::thread_rng();

    // we'll use randopm_scene as the default
    let mut world = Arc::new(random_scene(&mut rng));

    // now we handle which scene we want to render. That is really how we make the world.
    let matches = match matches.subcommand() {
        Some(("random_scene", matches)) => matches, // nothing to do, as this is the default
        _ => unreachable!("clap should ensure we don't get here"),
    };
    println!("first get_matches {:?}", matches);



    // if you make it 2000x1000 that's 100x, and then 100 more samples of each,
    // and then test against all the objects again for differaction. And then
    // do a depth of 50.
    // NOTE: 2000x1000 takes about 40 minutes at this point
    // only 92s as of this commit. Run in release instead of debug.
    #[allow(non_snake_case)]
    let SAMPLES_PER_PIXEL: i32 = ri.samples; // number of anti-aliasing samples
    #[allow(non_snake_case)]
    let MAX_DEPTH: i32 = ri.depth;
    //let radian: f64 = (std::f64::consts::PI/4.0).cos();
    let vfov: f64 = ri.vfov as f64;
    //let aspect: f64 = nx as f64 / ny as f64;
    const ASPECT_RATIO: f64 = 2.0 / 1.0; // eg 2000x1000, 800x400, 200x100
                                         // instead of setting the values and generating the aspect ratio,
                                         // we'll say how wide we want the overall image, and let the
                                         // ratio determine the height instead.
    #[allow(non_snake_case)]
    let IMAGE_WIDTH: i32 = ri.width; // image width
    #[allow(non_snake_case)]
    let IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    #[allow(non_snake_case)]
    let NUM_PIXELS: i32 = IMAGE_WIDTH * IMAGE_HEIGHT;

    let look_from = vect!(25.0, 2.5, 5.0);
    let look_at = rtmacros::vect!(3.0, 0.75, 0.75);
    let vup = rtmacros::vect!(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = (look_from - look_at).length();
    #[allow(non_snake_case)]
    let APERTURE: f64 = ri.aperture as f64;

    let start_time_in_sec:f64 = 0.0;
    let stop_time_in_sec:f64 = 0.0;
    let mut bvh = Bvh::new();
    bvh.add_hitlist(& mut world, start_time_in_sec, stop_time_in_sec);
    let world = Arc::new(bvh.build());

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
                    pixel_color += color(&r, world.as_ref(), MAX_DEPTH);
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
