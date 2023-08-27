use rand_distr::Distribution;
use std::time::Instant;
use statrs::distribution::Normal;
use rand::SeedableRng;


fn sim() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let normal = Normal::new(1.0, 1.0).unwrap();

    let num_paths: usize = 200000;
    let num_steps: usize = 700;

    // Initialize the vector
    let mut _v: Vec<f64> = Vec::with_capacity(num_paths);

    // This takes about 0.9 to 1.1 secs. which matches numpy with same parameters, and 'out' option.
    for _step in 1..num_steps {
        _v = normal.sample_iter(&mut rng).take(num_paths).collect();
    }
}


fn main() {
    let start = Instant::now();
    sim();

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
