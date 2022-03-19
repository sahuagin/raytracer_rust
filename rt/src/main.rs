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
    path::Path,
};

use rayon::prelude::*;

use rtlib::camera::Camera;
#[allow(unused_imports)]
use rtlib::materials::{Dielectric, Lambertian, Metal};
#[allow(unused_imports)]
use rtlib::sphere::Sphere;
#[allow(unused_imports)]
use rtlib::util::{
    color,
    color_just_attenuation,
    earth_scene,
    Image,
    random_scene,
    simple_light_scene,
    two_perlin_spheres,
    two_spheres,
    write_color,
};
use rtlib::vec3::Color;
use rtlib::bvh::Bvh;
use rtlib::hitlist::HitList;


fn main() {
    #[derive(Debug, Clone)]
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
        texture: Option<Image>,
        interior_light: Color,
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
        texture: None,
        // use interior lighthing by default
        interior_light: Color::new(1.0, 1.0, 1.0),
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
            ).arg(
                clap::arg!(--globe_texture <FILE> "An image that should be used if a textured globe is to be displayed.")
                .required(false)
                .allow_invalid_utf8(true)
            ).arg(
                clap::arg!(--explicit_lighting <TRUEorFALSE> "If you scene explicitly uses lights, set this to true. If you want to see all objects without worrying about lighting, set to false.")
                .required(false)
                .default_value("false")
            )
        .subcommand_required(true)
        .subcommand(
            Command::new("random_scene")
            .about("Generates the cover of the RayTracing book. 3 orbs in center of picture, 100 spheres of random textures around. One large sphere for 'floor'.")
            .arg(clap::arg!(-x --checkerboard "Turns the base to a checkerboard pattern."))
            )
        .subcommand(
            Command::new("two_spheres")
            .about("Display 2 large checkerboard spheres.")
            ).
        subcommand(
            Command::new("two_perlin_spheres")
            .about("Display 2 spheres with Perlin noise.")
            ).
        subcommand(
            Command::new("earth_scene")
            .about("Display a picture of Earth projected on a sphere.")
            ).
        subcommand(
            Command::new("simple_light_scene")
            .about("Using the two_perlin_spheres scene,
                   add a sphere and a rectangle of light.")
            );

    let matches = cmd.get_matches();

    //eprintln!("What we got on the cmdline {:?}", matches);

    // NOTE: If there is a default value for something, it'll always
    // show as "present", so we know we have settings for all of the
    // ones we have as default.
    ri.samples = matches.value_of_t("num_samples").expect("Number of samples is required.");
    ri.depth = matches.value_of_t("max_depth").expect("Maximum depth is required.");
    ri.vfov = matches.value_of_t("vfov").expect("Vertical FOV is required.");
    ri.width = matches.value_of_t("image_width").expect("Image width required.");
    ri.aperture = matches.value_of_t("aperture").expect("Aperture required.");
    ri.start = matches.value_of_t("start_time").expect("Start time required.");
    ri.stop = matches.value_of_t("stop_time").expect("Stop time required.");
    if let Some(raw_texture_path) = matches.value_of_os("globe_texture") {
        let config_path = Path::new(raw_texture_path);
        ri.texture = Some(Image::new(&config_path.display()));
    }

    let el: bool = matches.value_of_t("explicit_lighting").expect("Lighting type required.");
    if el == true {
        // the scene will provide it's own light, so objects don't have to produce
        // their own light
        ri.interior_light = Color::new(0.0, 0.0, 0.0);
    }

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
            texture: ri.texture,
            interior_light: ri.interior_light,
       }; 
    }
    // make read only
    let ri = ri;

    //eprintln!("Render info after arg parsing. {:?}", ri);

    
    // For error handling
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let mut rng = rand::thread_rng();

    // we'll use randopm_scene as the default
    let mut world: Arc<HitList>;

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
    let mut look_from = vect!(25.0, 2.5, 5.0);
    let mut look_at = rtmacros::vect!(3.0, 0.75, 0.75);
    let vup = rtmacros::vect!(0.0, 1.0, 0.0);
    let mut dist_to_focus: f64 = (look_from - look_at).length();
    #[allow(non_snake_case)]
    let mut APERTURE: f64 = ri.aperture as f64;
    let start_time_in_sec:f64 = 0.0;
    let stop_time_in_sec:f64 = 0.0;
    let vfov: f64 = ri.vfov as f64;
    let mut interior_light = ri.interior_light;
    let mut camera = Camera::new(
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

    // now we handle which scene we want to render. That is really how we make the world.
    // may need matches later for other subcommands
    #[allow(unused_variables)]
    let matches = match matches.subcommand() {
        Some(("random_scene", matches)) => {
            // this is already the default, so we only have to modify if the
            // checkerboard option is selected
            if matches.is_present("checkerboard") {
                world = Arc::new(random_scene(&mut rng, true));
            } else
            {
                world = Arc::new(random_scene(&mut rng, false));
            }

            matches
        },
        Some(("two_spheres", matches)) => {
            // we need to change the camera as well as populate a different world
            look_from = vect!(13, 2, 3);
            look_at = vect!(0, 0, 0);
            dist_to_focus = 10.0;
            APERTURE = 0.0;
            camera = Camera::new(
                look_from,
                look_at,
                vect!(0,1,0),
                20.,
                ASPECT_RATIO,
                APERTURE,
                dist_to_focus,
                0.0,
                1.0);
            world = Arc::new(two_spheres());
            matches
        },
        Some(("two_perlin_spheres", matches)) => {
            // we need to change the camera as well as populate a different world
            look_from = vect!(13, 2, 3);
            look_at = vect!(0, 0, 0);
            dist_to_focus = 10.0;
            APERTURE = 0.0;
            camera = Camera::new(
                look_from,
                look_at,
                vect!(0,1,0),
                20.,
                ASPECT_RATIO,
                APERTURE,
                dist_to_focus,
                0.0,
                1.0);
            world = Arc::new(two_perlin_spheres());
            matches

        },
        Some(("earth_scene", matches)) => {
            look_from = vect!(13, 2, 3);
            look_at = vect!(0, 0, 0);
            dist_to_focus = 10.0;
            APERTURE = 0.0;
            camera = Camera::new(
                look_from,
                look_at,
                vect!(0,1,0),
                20.,
                ASPECT_RATIO,
                APERTURE,
                dist_to_focus,
                0.0,
                1.0);
            world = Arc::new(earth_scene());
            matches
        },
        Some(("simple_light_scene", matches)) => {
            // we need to change the camera as well as populate a different world
            look_from = vect!(25, 2, 2);
            look_at = vect!(0, 0, 0);
            dist_to_focus = 10.0;
            APERTURE = 0.0;
            interior_light = Color::new(0.0, 0.0, 0.0);
            camera = Camera::new(
                look_from,
                look_at,
                vect!(0,1,0),
                30.,
                ASPECT_RATIO,
                APERTURE,
                dist_to_focus,
                0.0,
                1.0);
            world = Arc::new(simple_light_scene());
            matches
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
    //eprintln!("first get_matches {:?}", matches);
    //eprintln!("the render data is {:?}", &ri);

    let camera = camera;
    let interior_light = interior_light;

    let mut bvh = Bvh::new();
    bvh.add_hitlist(& mut world, start_time_in_sec, stop_time_in_sec);
    let world = Arc::new(bvh.build());


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
                    pixel_color += color(&r, world.as_ref(), MAX_DEPTH, &interior_light);
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
