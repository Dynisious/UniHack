use rocket::response::NamedFile;
use std::path::Path;
use std::fs::File;

static COORDINATES: &'static [(f32, f32)] = &[
    (37.9136876, 145.124993),
    (-37.9004926, 145.1269537),
    (-37.8988587, 145.1248777),
    (-37.8914801, 145.1436102),
    (-37.885445, 145.1541325),
];

#[get("/results/<a>/<b>/<c>/<d>/<e>")]
pub fn get_neural_results(a: usize, b: usize, c: usize, d: usize, e: usize) -> Result<String, String> {
    let input = [a, b, c, d, e];
    let neural_model = ::UniHack::load_network(File::open("neural_networks/neural_model").map_err(|_| "Neural file not found.")?).map_err(|_| "Neural file could not be parsed.")?;
    let output = ::UniHack::neural_net(&input,neural_model).0;
    Ok(format!(
        "{},{};{},{};{},{}",
        COORDINATES[output[0] % 5].0,
        COORDINATES[output[0] % 5].1,
        COORDINATES[output[1] % 5].0,
        COORDINATES[output[1] % 5].1,
        COORDINATES[output[2] % 5].0,
        COORDINATES[output[2] % 5].1
    ))
}

#[get("/itinerary")]
pub fn return_results() -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/itinerary.html")).ok()
}
