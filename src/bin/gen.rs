
extern crate UniHack;

use std::fs::File;

fn main() {
    UniHack::store_network(File::create(".\\neural_networks\\neural_model").unwrap(), &UniHack::NeuralNet::default()).unwrap()
}
