use rand::RngExt;

fn main() {
    let target = std::f64::consts::PI;

    let mut rng = rand::rng();

    let mut inside = 0;
    let mut count = 0;

    for _ in 0..100000000 {
        let x: f64 = rng.random_range(0.0..1.0);
        let y: f64 = rng.random_range(0.0..1.0);

        if x * x + y * y <= 1.0 {
            inside += 1;
        }

        count += 1;

        if count % 100000 == 0 {
            let estimate = 4.0 * inside as f64 / count as f64;
            println!("Current estimate: {}", estimate);
            let error = (estimate - target).abs();
            println!("Current error: {}", error);
        }
    }
}
