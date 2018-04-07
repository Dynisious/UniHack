extern crate UniHack;

use UniHack::NeuralNet;
use std::fs::File;

fn main() {
    let mut model = UniHack::load_network(
        File::open(".\\neural_networks\\model_a").unwrap(),
    ).unwrap();

    for _ in 0..500 {
        model = model.clone().reproduce(&model);
    }

    UniHack::store_network(
        File::create(".\\neural_networks\\model_a").unwrap(),
        &model,
    ).unwrap()
}
