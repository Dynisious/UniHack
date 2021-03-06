use UniHack::NeuralNet;
use rocket::response::NamedFile;
use std::fs::File;
use std::path::Path;

// same coordinates as the results.rs
use results::COORDINATES;

// training_mode()
//
// serves the training page
#[get("/training")]
pub fn training_mode() -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/train-neural-net.html")).ok()
}

// neural_training()
//
// retrieves a GET request consisting of the specified neural model
// and 5 indices from the `quiz`
//
// returns a formatted string consisting of 3 sets of coordinates
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

// neural_training_completion()
//
// retrieved a GET requect consisting boolean
// whether the second model is scoring better than first model
//
// then, overwrites the losing model
// with the child product of the winning model and losing model

#[get("/training/<kill_win>")]
pub fn neural_training_completion(kill_win: bool) {
    let mut keep_path = "neural_networks/model_a";
    let mut kill_path = "neural_networks/model_b";
    if kill_win {
        let temp = keep_path;
        keep_path = kill_path;
        kill_path = temp;
    }
    let keep_model = ::UniHack::load_network(File::open(keep_path).unwrap()).unwrap();
    let mut kill_model = ::UniHack::load_network(File::open(kill_path).unwrap()).unwrap();
    kill_model = keep_model.reproduce(&kill_model);
    ::UniHack::store_network(File::create(kill_path).unwrap(), &kill_model).unwrap();
}
