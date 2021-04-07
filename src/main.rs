use std::fs::File;
use std::io::{BufRead, BufReader};

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
