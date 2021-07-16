use rust_digit_recognizer::{Evaluator, Observation, read_observations};
use simple_stopwatch::Stopwatch;

/// After being fed a training set of images representing digits, the program predicts the digits in a new set of images and reports how many were correctly predicted.
fn main()
{
    let sw = Stopwatch::start_new();

    let training_path = "trainingsample.csv";
    let training_set: Vec<Observation> = read_observations(training_path);
    
    let validation_path = "validationsample.csv";
    let validation_set: Vec<Observation> = read_observations(validation_path);

    let evaluator = Evaluator::new(training_set);
    let percent_correct = evaluator.percent_correct(validation_set);
    let percent_correct = format!("{:.2}%", 100.0 * percent_correct);

    println!("Correctly predicted: {}", percent_correct);

    let elapsed_seconds = sw.s();
    println!("Time elapsed = {:.2}s", elapsed_seconds);
}