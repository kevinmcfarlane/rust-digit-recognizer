//! # Digit Recognizer
//!
//! `rust-digit-recognizer` is a "from-the-ground-up" implementation of the Kaggle [Digit Recognizer](https://www.kaggle.com/c/digit-recognizer/overview) problem in machine learning.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use simple_stopwatch::Stopwatch;

#[derive(Debug)]
/// A digit from 0 to 9 and its representation in pixels.
pub struct Observation {
    label: String,
    pixels: Vec<i32> 
}

impl Observation {
    /// Constructs a new instance from a label (a number representing an image) and a collection of pixels representing that image.
    /// # Arguments
    ///
    /// * `label` - The number representing the image.
    /// * `pixels` - The pixels representing the image.
    ///
    pub fn new(label: &str, pixels: &Vec<i32>) -> Observation {
        let label = label;
        let pixels = pixels;

        Observation { label: label.to_string(), pixels: pixels.to_vec() }
    }
}

/// Compares two images pixel by pixel, computing each difference, and adding up their absolute values.
/// Identical images will have a distance of zero, and the further apart two pixels are, the higher the distance between the two
/// images will be. 
pub struct ManhattanDistance {}

impl ManhattanDistance {
    /// Computes the distance between two images. 
    /// Identical images will have a distance of zero.
    ///
    /// # Arguments
    ///
    /// * `pixels1` - The pixels representing the first image.
    /// * `pixels2` - The pixels representing the second image.
    ///
    pub fn between(pixels1: &Vec<i32>, pixels2: &Vec<i32>) -> f64 {
        assert_eq!(pixels1.len(), pixels2.len(), "Inconsistent image sizes.");

        let length = pixels1.len();
        let mut distance: f64 = 0.0;

        for i in 0..length {
            let difference = f64::from(pixels1[i] - pixels2[i]).abs();
            distance += difference;
        }

        distance
    }
}

/// Classifies/predicts an image from a collection of pixels using a matching algorithm.
pub struct BasicClassifier {}

impl BasicClassifier {
    /// Predicts the digit that the image in pixels corresponds to.
    ///
    /// # Arguments
    ///
    /// * `training_set` -  The training set of observations.
    /// * `pixels` -  The pixels representing the image.
    ///
    pub fn predict(training_set: &Vec<Observation>, pixels: &Vec<i32>) -> String {
        let mut shortest = f64::MAX;
        
        let default_label = "";
        let default_pixels: Vec<i32> = Vec::new();
        let mut current_best = Observation::new(default_label, &default_pixels);

        for obs in training_set {
            let dist = ManhattanDistance::between(&obs.pixels, &pixels);
            if dist < shortest {
                shortest = dist;
                current_best = Observation::new(&obs.label, &obs.pixels);
            }
        }

        current_best.label
    }
}

/// Evaluates a model by computing the proportion of classifications it gets right.
pub struct Evaluator {
    training_set: Vec<Observation>,
}

impl Evaluator {
    /// Constructs a new instance from a training set of observations.
    ///
    /// # Arguments
    ///
    /// * `training_set` -  The training set of observations.
    ///
    pub fn new(training_set: Vec<Observation>) -> Evaluator {
        let training_set = training_set;

        Evaluator { training_set }
    }

    /// "Scores" the prediction by comparing what the classifier predicts with the true value. If they match,
    /// we record a 1, otherwise we record a 0. By using numbers like this rather than true/false values, we can
    /// average this out to get the percentage correct.
    pub fn score(&self, obs: Observation) -> f64 {
        let label = obs.label;
        let prediction = BasicClassifier::predict(&self.training_set, &obs.pixels);
        
        print!("Digit: {} - ", label);
        
        if label == prediction {
            println!("Match");
            1.0
        } else {
            println!("Mismatch");
            0.0
        }
    }

    /// Calculates the percentage (as a fraction < 1) of images that are correctly predicted.
    ///
    /// # Arguments
    ///
    /// * `validation_set` -  The validation set of observations.
    ///
    pub fn percent_correct(&self, validation_set: Vec<Observation>) -> f64 {
        let mut scores: Vec<f64> = Vec::new();
        let number_of_scores = validation_set.len();

        for obs in validation_set {
            let score = *(&self.score(obs));
            scores.push(score);
        }

        let sum: f64 = Iterator::sum(scores.iter());
        let average = sum / (number_of_scores as f64);

        average
    }
}

/// Reads observations as comma separated labels (the numbers the pixels represent) and pixels 
/// from the specified path and returns a collection of Observation instances.
///
/// # Arguments
///
/// * `path` -  The input path.
///
pub fn read_observations(path: &str) -> Vec<Observation>{
    let file = File::open(path).unwrap_or_else(|err| {
        println!("Problem opening file: {}", err);
        process::exit(1);
    });
    let reader = BufReader::new(file);

    // Skip header
    let rows = reader.lines().skip(1);

    let mut observations: Vec<Observation> = Vec::new();

    for row in rows 
    {
        let r = row.unwrap_or_else(|err| {
            println!("Problem extracting observation row from input: {}", err);
            process::exit(1);
        });

        let comma_separated: Vec<&str> = r.split(",").collect();
        let label = comma_separated[0];

        let pixel_strings = &comma_separated[1..];
        let mut pixels:  Vec<i32> = Vec::new();

        for pixel_string in pixel_strings {
            let pixel: i32 = pixel_string.parse().unwrap_or_else(|err| {
                println!("Problem converting pixel string into integer: {}", err);
                process::exit(1);
            });
            pixels.push(pixel);
        }

        let observation = Observation::new(label, &pixels);
        observations.push(observation);
    }

    observations
}

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