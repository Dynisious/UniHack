use rocket::response::NamedFile;
use std::fs::File;
use std::path::Path;

// COORDINATES
// a constant coordinates that will be returned by the neural network

pub const COORDINATES: [(f32, f32); 20] = [
    (-37.9136876, 145.124993),
    (-37.9004926, 145.1269537),
    (-37.8988587, 145.1248777),
    (-37.8914801, 145.1436102),
    (-37.885445, 145.1541325),
    (-37.8201476, 144.9679872),
    (-37.8175881, 144.9698728),
    (-37.8176178, 144.9678022),
    (-37.8175881, 144.9621749),
    (-37.8248957, 144.958173),
    (-37.8107587, 144.9627382),
    (-37.8111571, 144.946264),
    (-37.8139288, 144.9449229),
    (-37.8154226, 144.9391776),
    (-37.8221329, 144.94775),
    (-37.8028712, 144.9591225),
    (-37.8044733, 144.9505931),
    (-37.8030556, 144.95443),
    (-37.7990585, 144.9593961),
    (-37.7984926, 144.9678692),
];

// get_neural_results()
//
// retrieves a GET request consisting of 5 indices from the `quiz`
//
// returns a formatted string consisting of 3 sets of coordinates
#[get("/results/<a>/<b>/<c>/<d>/<e>")]
pub fn get_neural_results(
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
) -> Result<String, String> {
    let input = [a, b, c, d, e];
    let neural_model = ::UniHack::load_network(File::open("neural_networks/model_a").map_err(|_| "Neural file not found.")?)
        .map_err(|_| "Neural file could not be parsed.")?;
    let output = ::UniHack::neural_net(&input, neural_model).0;
    Ok(format!(
        "{},{};{},{};{},{}",
        COORDINATES[output[0]].0,
        COORDINATES[output[0]].1,
        COORDINATES[output[1]].0,
        COORDINATES[output[1]].1,
        COORDINATES[output[2]].0,
        COORDINATES[output[2]].1
    ))
}

// return_results()
//
// serves the itinerary page
#[get("/itinerary")]
pub fn return_results() -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/itinerary.html")).ok()
}
