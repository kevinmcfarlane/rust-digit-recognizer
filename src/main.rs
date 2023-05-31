use rust_digit_recognizer::{Evaluator, Observation, Distance, BasicClassifier, ManhattanDistance, EuclideanDistance, Classifier, read_observations};
use simple_stopwatch::Stopwatch;

/// After being fed a training set of images representing digits, the program predicts the digits in a new set of images and reports how many were correctly predicted.
fn main() {
    digit_recognize(&ManhattanDistance {}, "Manhattan Distance");
    digit_recognize(&EuclideanDistance {}, "Euclidean Distance");
}

/// Computes the percentage of correctly predicted digits using the specified distance (cost) algorithm.
/// 
/// # Arguments
///
/// * `distance` - A distance (cost) algorithm.
/// * `description` - Algorithm description.
///
fn digit_recognize(distance: &dyn Distance, description: &str) {
    let sw = Stopwatch::start_new();
    
    let mut classifier = BasicClassifier { training_set: Vec::new(), distance };

    let training_path = "trainingsample.csv";
    let training_set: Vec<Observation> = read_observations(training_path);
    classifier.train(&training_set);

    let validation_path = "validationsample.csv";
    let validation_set: Vec<Observation> = read_observations(validation_path);

    let evaluator = Evaluator {};
    let percent_correct = 100.0 * evaluator.percent_correct(&validation_set, &classifier);
    let percent_correct = format!("{percent_correct:.2}%");

    println!("Correctly predicted: {percent_correct}");

    let elapsed_seconds = sw.s();
    println!("Time elapsed ({description}) = {elapsed_seconds:.2}s\n");
}
