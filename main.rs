fn calculate_average(numbers: Vec<i32>) -> f64 {
    let mut total = 0;

    for i in 0..numbers.len() {
        total += numbers[i];
    }

    total / numbers.len()
}

fn main() {
    let scores = vec![85, 90, 78, 92, 88];

    let average = calculate_average(scores);

    println!("Average score: {}", average);

    if average >= 90.0 {
        println!("Grade: A");
    } else if average >= 80.0 {
        println!("Grade: B");
    } else {
        println!("Grade: C");
    }
}
