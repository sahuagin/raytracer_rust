fn main() {
    let nx = 200; // image width
    let ny = 100; // image height

    println!("P3\n{} {}\n255", nx, ny);

    // we use the riter because origin is at the lower left 
    // to maintain a right handed coordinate system
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
