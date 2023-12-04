//! # Digit Recognizer
//!
//! `rust-digit-recognizer` is a "from-the-ground-up" implementation of the Kaggle [Digit Recognizer](https://www.kaggle.com/c/digit-recognizer/overview) problem in machine learning.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::Sync;
use std::process;
use rayon::prelude::*;

#[derive(Debug, Clone)]
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
    pub fn new(label: &str, pixels: &[i32]) -> Observation {
        Observation { label: label.to_string(), pixels: pixels.to_vec() }
    }
}

// Could also use derive (PartialEq) in place of this
impl PartialEq for Observation {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.pixels == other.pixels
    }
}

pub trait Distance {
    fn between(&self, pixels1: &[i32], pixels2: &[i32]) -> f64;
}

/// Compares two images pixel by pixel, computing each difference, and adding up their absolute values.
/// Identical images will have a distance of zero, and the further apart two pixels are, the higher the distance between the two
/// images will be. 
pub struct ManhattanDistance {}

impl Distance for ManhattanDistance {
    /// Computes the distance between two images. 
    /// Identical images will have a distance of zero.
    /// Uses sum of absolute difference.
    ///
    /// # Arguments
    ///
    /// * `pixels1` - The pixels representing the first image.
    /// * `pixels2` - The pixels representing the second image.
    ///
    fn between(&self, pixels1: &[i32], pixels2: &[i32]) -> f64 {
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

/// Compares two images pixel by pixel, computing each difference, and adding up their squared values.
/// Identical images will have a distance of zero, and the further apart two pixels are, the higher the distance between the two
/// images will be. 
pub struct EuclideanDistance {}

impl Distance for EuclideanDistance {
    /// Computes the distance between two images. 
    /// Identical images will have a distance of zero.
    /// Uses sum of squared difference.
    ///
    /// # Arguments
    ///
    /// * `pixels1` - The pixels representing the first image.
    /// * `pixels2` - The pixels representing the second image.
    ///
    fn between(&self, pixels1: &[i32], pixels2: &[i32]) -> f64 {
        assert_eq!(pixels1.len(), pixels2.len(), "Inconsistent image sizes.");

        let length = pixels1.len();
        let mut distance: f64 = 0.0;

        for i in 0..length {
            let difference = (pixels1[i] - pixels2[i]).pow(2);
            distance += difference as f64;
        }

        distance
    }
}

/// Trains a set of known observations and predicts the label of an image.
pub trait Classifier: Sync {
    fn train(&mut self, training_set: &[Observation]);
    fn predict(&self, pixels: &[i32]) -> String;
}

/// Classifies/predicts an image from a collection of pixels using a matching distance (cost) algorithm.
pub struct BasicClassifier<'a> {
    pub training_set: Vec<Observation>,
    pub distance: &'a dyn Distance,
}

unsafe impl<'a> Sync for BasicClassifier<'a> {}

impl<'a> Classifier for BasicClassifier<'a> {
    
    /// Trains a set of known observations.
    /// # Arguments
    ///
    /// * `training_set` - The training set of observations.
    /// 
    fn train(&mut self, training_set: &[Observation]) {
        self.training_set = training_set.to_vec();
    }

    /// Predicts the digit that the image in pixels corresponds to.
    ///
    /// # Arguments
    ///
    /// * `pixels` - The pixels representing the image.
    /// 
    fn predict(&self, pixels: &[i32]) -> String {
        let mut best_prediction = &self.training_set[0];
        let mut shortest_distance = self.distance.between(&best_prediction.pixels, pixels);
    
        for observation in &self.training_set[1..] {
            let distance = self.distance.between(&observation.pixels, pixels);
            if distance < shortest_distance {
                shortest_distance = distance;
                best_prediction = observation;
            }
        }
    
        best_prediction.label.to_string()
    }
}

/// Evaluates a model by computing the proportion of classifications it gets right.
pub struct Evaluator {}

impl Evaluator {
    /// "Scores" the prediction by comparing what the classifier predicts with the true value. If they match,
    /// we record a 1, otherwise we record a 0. By using numbers like this rather than true/false values, we can
    /// average this out to get the percentage correct.
    /// 
    /// # Arguments
    ///
    /// * `observation` - A digit from 0 to 9 and its representation in pixels.
    /// * `classifier` - The classifier for predicting whether the observation matches its label.
    ///
    pub fn score(&self, observation: &Observation, classifier: &dyn Classifier) -> f64 {
        let prediction = classifier.predict(&observation.pixels);
    
        println!("Digit: {} - {}", observation.label, if observation.label == prediction { "Match" } else { "Mismatch" });
        
        (observation.label == prediction) as i32 as f64
    }
    
    /// Calculates the percentage (as a fraction < 1) of images that are correctly predicted.
    ///
    /// # Arguments
    ///
    /// * `validation_set` - The validation set of observations.
    /// * `classifier` - The classifier for predicting  whether an observation matches its label.
    ///
    pub fn percent_correct(&self, validation_set: &[Observation], classifier: &dyn Classifier) -> f64 {
        validation_set
            .par_iter()
            .map(|observation| self.score(observation, classifier))
            .sum::<f64>()
            / validation_set.len() as f64
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
        eprintln!("Problem opening file: {path} - {err}");
        process::exit(1);
    });
    let reader = BufReader::new(file);

    // Skip header
    let rows = reader.lines().skip(1);

    let mut observations: Vec<Observation> = Vec::new();

    for row in rows 
    {
        let r = row.unwrap_or_else(|err| {
            eprintln!("Problem extracting observation row from input: {err} in file: {path}");
            process::exit(1);
        });

        let comma_separated: Vec<&str> = r.split(',').collect();
        let label = comma_separated[0];

        let pixel_strings = &comma_separated[1..];
        let mut pixels:  Vec<i32> = Vec::new();

        for pixel_string in pixel_strings {
            let pixel: i32 = pixel_string.parse().unwrap_or_else(|err| {
                eprintln!("Problem converting pixel string into integer: {err} in file: {path}");
                process::exit(1);
            });
            pixels.push(pixel);
        }

        let observation = Observation::new(label, &pixels);
        observations.push(observation);
    }

    observations
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_observation_new() {
        let observation = Observation::new("5", &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(observation.label, "5");
        assert_eq!(observation.pixels, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_manhattan_distance_between() {
        let distance = ManhattanDistance {};
        let pixels1 = vec![0, 0, 0, 0, 0];
        let pixels2 = vec![1, 1, 1, 1, 1];
        assert_eq!(distance.between(&pixels1, &pixels2), 5.0);
    }

    #[test]
    fn test_euclidean_distance_between() {
        let distance = EuclideanDistance {};
        let pixels1 = vec![0, 0, 0, 0, 0];
        let pixels2 = vec![3, 4, 5, 6, 7];
        assert_eq!(distance.between(&pixels1, &pixels2), 135.0);
    }

    #[test]
    fn test_basic_classifier_train() {
        let mut classifier = BasicClassifier {
            training_set: vec![],
            distance: &ManhattanDistance {},
        };
        let training_set = vec![
            Observation::new("0", &[0, 0, 0, 0, 0]),
            Observation::new("1", &[1, 1, 1, 1, 1]),
        ];
        classifier.train(&training_set);

        assert_eq!(classifier.training_set, training_set);
    }

    #[test]
    fn test_basic_classifier_predict() {
        let classifier = BasicClassifier {
            training_set: vec![
                Observation::new("0", &[0, 0, 0, 0, 0]),
                Observation::new("1", &[1, 1, 1, 1, 1]),
            ],
            distance: &ManhattanDistance {},
        };
        let pixels = vec![0, 0, 0, 0, 0];
        assert_eq!(classifier.predict(&pixels), "0");
    }

    #[test]
    fn test_evaluator_percent_correct() {
        let evaluator = Evaluator {};
        let training_set = vec![
            Observation::new("0", &[0, 0, 0, 0, 0]),
            Observation::new("1", &[1, 1, 1, 1, 1]),
        ];
        let validation_set = vec![
            Observation::new("0", &[0, 0, 0, 0, 0]),
            Observation::new("1", &[1, 1, 1, 1, 1]),
        ];
        let mut classifier = BasicClassifier {
            training_set: vec![],
            distance: &ManhattanDistance {},
        };

        classifier.train(&training_set);

        assert_eq!(evaluator.percent_correct(&validation_set, &classifier), 1.0);
    }

    #[test]
    fn test_evaluator_score() {
        let evaluator = Evaluator {};
        let training_set = vec![
            Observation::new("0", &[0, 0, 0, 0, 0]),
            Observation::new("1", &[1, 1, 1, 1, 1]),
        ];
        let observation = Observation::new("0", &[0, 0, 0, 0, 0]);
        let mut classifier = BasicClassifier {
            training_set: vec![],
            distance: &ManhattanDistance {},
        };

        classifier.train(&training_set);

        assert_eq!(evaluator.score(&observation, &classifier), 1.0);
    }
}
