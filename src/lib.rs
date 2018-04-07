
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod neural_net;

use std::fs::File;
use neural_net::*;
pub use neural_net::NeuralNet;

pub fn neural_net(inputs: &[usize], mut network: NeuralNet) -> ([usize; NEURAL_OUTPUT], NeuralNet) {
    network = network.clean().inputs(inputs);
    
    (network.run(), network)
}

pub fn store_network(file: File, network: &NeuralNet) -> serde_json::Result<()> {
    serde_json::to_writer(file, network)
}

pub fn load_network(file: File) -> serde_json::Result<NeuralNet>{
    serde_json::from_reader(file)
}
