
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

/// Reads images from a file and transforms them to a form suitable for analyis.
pub struct DataReader {

}

impl DataReader {

    /// Return an Observation instance.
    ///
    /// # Arguments
    ///
    /// * `data` - A line of comma-delimited input data.
    ///
    pub fn  observation_factory(data: String) -> Observation {
        let label = data;
        let pixels = Vec::new();
        
        // TODO

        Observation::new(label.to_string(), pixels)
    }
}

fn main() {
    let label = "Point 1";
    let mut pixels = Vec::new();
    pixels.push(1);
    pixels.push(2);
    pixels.push(3);
    pixels.push(4);

    let observation = Observation::new(label.to_string(), pixels);
    println!("{:#?}", observation);

    let data = "";
    let obs = DataReader::observation_factory(data.to_string());
    println!("{:#?}", obs);
}
