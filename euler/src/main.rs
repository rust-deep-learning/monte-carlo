use rand::RngExt;

fn main() {
    let target = 2.718281828459;

    let mut rng = rand::rng();

    let mut sum = 0.0;
    let mut count = 0;

    for _ in 0..100000000 {
        let mut current_sum = 0.0;
        let mut current_count = 0.0;
        while current_sum < 1.0 {
            let random_number: f64 = rng.random_range(0.0..1.0);
            current_sum += random_number;
            current_count += 1.0;
        }

        sum += current_count;
        count += 1;

        if count % 100000 == 0 {
            println!("Current average: {}", sum / count as f64);
            let error = (sum / count as f64 - target).abs();
            println!("Current error: {}", error);
        }
    }
}
