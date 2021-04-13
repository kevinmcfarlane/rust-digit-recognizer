use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
/// A digit from 0 to 9 and its representation in pixels.
pub struct Observation<'a> {
    label: &'a str,
    pixels: Vec<i32> 
}

impl<'a> Observation<'a> {
    pub fn new(label: &str, pixels: Vec<i32>) -> Observation {
        let label = label;
        let pixels = pixels;

        Observation { label, pixels }
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

pub struct BasicClassifier<'a> {
    data: Vec<Observation<'a>>
}

impl<'a> BasicClassifier<'a> {
    pub fn new(data: Vec<Observation>) -> BasicClassifier {
        let data = data;

        BasicClassifier { data }
    }

    /// Predicts the digit that the image corresponds to.
    ///
    /// # Arguments
    ///
    /// `pixels` -  The pixels representing the image.
    ///
    pub fn predict(self, pixels: &'a Vec<i32>) -> &str {
        let mut shortest = f64::MAX;
        let mut current_best = Observation::new("", vec![0]);
        let data = self.data;

        for obs in data {
            let dist = ManhattanDistance::between(&obs.pixels, &pixels);
            if dist < shortest {
                shortest = dist;
                current_best = obs;
            }
        }

        current_best.label
    }
}

pub struct Evaluator {}

impl Evaluator {
    /// "Scores" the prediction by comparing what the classifier predicts with the true value. If they match,
    /// we record a 1, otherwise we record a 0. By using numbers like this rather than true/false values, we can
    /// average this out to get the percentage correct.
    pub fn score(obs: &Observation, classifier: &BasicClassifier) -> f64 {
        let label = obs.label;
        let prediction = &classifier.predict(&obs.pixels);
        
        if label == prediction {
            1.0
        } else {
            0.0
        }
    }
}

fn main()
{
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    // Skip header
    let rows = reader.lines().skip(1);

    let mut observations: Vec<Observation> = Vec::new();

    for row in rows 
    {
        let r = row.unwrap();

        let comma_separated: Vec<&str> = r.split(",").collect();
        let label = comma_separated[0];

        let pixel_strings = &comma_separated[1..];
        let mut pixels:  Vec<i32> = Vec::new();

        for pixel_string in pixel_strings {
            let pixel: i32 = pixel_string.parse().unwrap();
            pixels.push(pixel);
        }

        let observation = Observation::new(label, pixels);

        observations.push(observation);
    }

    println!("{:?}", observations);
}