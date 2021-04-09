use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::io::Read;

#[derive(Debug)]
/// A digit from 0 to 9 and its representation in pixels.
pub struct Observation {
    label: String,
    pixels: Vec<i32> 
}

impl Observation {
    pub fn new(label: String, pixels: Vec<i32>) -> Observation {
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

pub struct BasicClassifier {
    data: Vec<Observation>
}

impl BasicClassifier {
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
    pub fn predict(self, pixels: Vec<i32>) -> String {
        let mut shortest = f64::MAX;
        let mut current_best = Observation::new("".to_string(), vec![0]);
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

        let observation = Observation::new(label.to_string(), pixels);

        observations.push(observation);
    }

    println!("{:?}", observations);
}



// /// Reads images from a file and transforms them to a form suitable for analyis.
// pub struct DataReader {

// }

// impl DataReader {

//     /// Return an Observation instance.
//     ///
//     /// # Arguments
//     ///
//     /// * `data` - A line of comma-delimited input data.
//     ///
//     pub fn  observation_factory(data: String) -> Observation {
//         let label = data;
//         let pixels = Vec::new();
        
//         // TODO

//         Observation::new(label.to_string(), pixels)
//     }
// }

// fn read_file() {
//     let mut file = std::fs::File::open("..\\..\\data.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     print!("{}", contents);
// }

// fn main() {
//     let label = "Point 1";
//     let mut pixels = Vec::new();
//     pixels.push(1);
//     pixels.push(2);
//     pixels.push(3);
//     pixels.push(4);

//     let observation = Observation::new(label.to_string(), pixels);
//     println!("{:#?}", observation);

//     let data = "";
//     let obs = DataReader::observation_factory(data.to_string());
//     println!("{:#?}", obs);

    // let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
    // println!("{:?}", v);

    // let data1 = "label,pixel0,pixel1,pixel2,pixel3,pixel4,pixel5";
    // let pixels1: Vec<&str> = data1
    //     .split(",")
    //     .skip(1)
    //     //.map(|s| s.parse().unwrap())
    //     .collect();
    // println!("{:?}", pixels1);

//     read_file();    
// }
