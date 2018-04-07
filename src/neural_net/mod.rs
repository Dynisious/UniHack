
use std::collections::HashMap;

mod neuron;

use self::neuron::*;

pub const LAYER_SIZE: usize = 20;
pub const NEURAL_OUTPUT: usize = 3;

pub struct NeuralNet {
    layers: [HashMap<usize, Neuron>; LAYER_SIZE],
}

impl NeuralNet {
    pub fn new(layers: [HashMap<usize, Neuron>; LAYER_SIZE]) -> Self {
        Self { layers }
    }
    pub fn clean(mut self) -> Self {
        for layer in self.layers.iter_mut() {
            for (_, neuron) in layer.iter_mut() {
                *neuron = neuron.clone().clean();
            }
        }
        
        self
    }
    pub fn verify(&mut self) -> () {
        for layer in 0..self.layers.len() {
            for index in 0..LAYER_SIZE {
                if let Some(neuron) = self.layers[layer].get_mut(&index) {
                    
                }
            }
        }
    }
    pub fn run(&mut self) -> [usize; NEURAL_OUTPUT] {
        [0; NEURAL_OUTPUT]
    }
}
