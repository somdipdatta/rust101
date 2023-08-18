use rand::thread_rng;
use rand_distr::{Normal, Distribution};
use std::time::Instant;


fn sim() {
    let mut rng = thread_rng();
    let normal = Normal::new(1.0, 1.0).unwrap();

    let num_paths: usize = 100000;
    let num_steps: usize = 1000;

    // This takes about 20 secs. Next, how to collect into pre-allocated array?
    for _step in 1..num_steps {
        let _v: Vec<f32> = normal.sample_iter(&mut rng).take(num_paths).collect();
    }
}


fn main() {
    let start = Instant::now();
    sim();

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
