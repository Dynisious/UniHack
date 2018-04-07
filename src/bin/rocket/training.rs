use UniHack::NeuralNet;
use rocket::request::Form;
use rocket::response::NamedFile;
use std::fs::File;
use std::path::Path;

use results::COORDINATES;

#[derive(FromForm)]
pub struct Scores {
    model_a: f32,
    model_b: f32,
}

#[get("/training")]
pub fn training_mode() -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/training.html")).ok()
}

#[get("/training/<neural_model>/<a>/<b>/<c>/<d>/<e>")]
pub fn neural_training(
    neural_model: String,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
) -> Result<String, String> {
    let input = [a, b, c, d, e];
    let path = format!("neural_networks/{}", neural_model);
    let neural_model = ::UniHack::load_network(File::open(path).map_err(|_| "Neural file not found.")?)
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

#[post("/training", data = "<scores>")]
pub fn neural_training_completion(scores: Form<Scores>) {
    let model1_value = scores.get().model_a;
    let model2_value = scores.get().model_b;
    let mut keep_path = "neural_networks/model_a";
    let mut kill_path = "neural_networks/model_b";
    if model1_value<model2_value {
        let temp = keep_path;
        keep_path = kill_path;
        kill_path = temp;
    }
    let keep_model = ::UniHack::load_network(File::open(keep_path).unwrap()).unwrap();
    let mut kill_model = ::UniHack::load_network(File::open(kill_path).unwrap()).unwrap();
    kill_model = keep_model.reproduce(&kill_model);
    ::UniHack::store_network(File::create(kill_path).unwrap(), &kill_model).unwrap();
}
