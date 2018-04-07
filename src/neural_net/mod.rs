
use std::collections::HashMap;

mod neuron;

use self::neuron::*;

pub const LAYER_SIZE: usize = 20;
pub const NEURAL_OUTPUT: usize = 3;

#[derive(Clone)]
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
                if let Some(ref mut neuron) = self.layers[layer].get_mut(&index)
                    .map(|neuron| neuron.clone()) {
                    neuron.outputs = neuron.outputs.iter()
                        .map(Clone::clone)
                        .filter(|(output, _)| self.layers[layer].iter().any(|(target, _)| target == output))
                        .collect();
                }
            }
        }
    }
    pub fn run(&mut self) -> [usize; NEURAL_OUTPUT] {
        for layer in 0..self.layers.len() {
            for index in 0..LAYER_SIZE {
                if let Some((value, outputs)) = self.layers[layer].get_mut(&index)
                    .map(|neuron| (neuron.clone().normalise().state, neuron.outputs.clone())) {
                    let layer = layer + 1;
                    
                    for (output, weight) in outputs.iter() {
                        if let Some(ref mut neuron) = self.layers[layer].get_mut(&output) {
                            neuron.state += value * weight;
                        }
                    }
                }
            }
        }
        
        let mut res = [0; NEURAL_OUTPUT];
        let mut output = self.layers[LAYER_SIZE - 1].iter()
            .map(|(index, neuron)| (index, neuron.clone().normalise().state))
            .collect::<Vec<_>>();
        output.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(::std::cmp::Ordering::Equal));
        
        for (res, output) in res.iter_mut()
            .zip(output.iter().map(|(&index, _)| index)) {
            *res = output;
        }
        
        return res;
    }
    pub fn reproduce(left: &Self, right: &Self) -> Self {
        left.clone()
    }
}
