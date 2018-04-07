
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

mod neuron;

use self::neuron::*;

pub const LAYER_SIZE: usize = 20;
pub const NEURAL_OUTPUT: usize = 3;

#[derive(Clone, Serialize, Deserialize)]
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
    pub fn integrety(mut self) -> Self {
        for layer in 1..(LAYER_SIZE - 1) {
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
        
        self
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
    pub fn reproduce(mut res: Self, other: &Self) -> Self {
        use std::collections::hash_map::{DefaultHasher, Entry::*};
        
        let mut hasher = DefaultHasher::default();
        
        res.hash(&mut hasher);
        other.hash(&mut hasher);
        for layer in 1..(LAYER_SIZE - 1) {
            for index in 0..LAYER_SIZE {
                let other = other.layers[layer].get(&index);
                
                match res.layers[layer].entry(index) {
                    Occupied(mut occupied) => match other {
                        Some(other_neuron) => {
                            let res_neuron = occupied.get_mut();
                            
                            other_neuron.hash(&mut hasher);
                            res_neuron.hash(&mut hasher);
                            match hasher.finish() % 3 {
                                0 => *res_neuron = res_neuron.clone().reproduce(other_neuron),
                                1 => *res_neuron = other_neuron.clone(),
                                2 => (),
                                _ => panic!(),
                            }
                        },
                        None => {
                            occupied.get_mut().hash(&mut hasher);
                            match hasher.finish() % 3 {
                                0 => { occupied.remove(); },
                                1 => {
                                    let res_neuron = occupied.get_mut();
                                    
                                    *res_neuron = res_neuron.clone();
                                },
                                2 => (),
                                _ => panic!(),
                            }
                        },
                    },
                    Vacant(mut vacant) => match other {
                        Some(other_neuron) => {
                            other_neuron.hash(&mut hasher);
                            match hasher.finish() % 3 {
                                0 => { vacant.insert(other_neuron.clone()); },
                                1 => { vacant.insert(other_neuron.clone()); },
                                2 => (),
                                _ => panic!(),
                            }
                        },
                        None => {
                            match hasher.finish() % 2 {
                                0 => { vacant.insert(Neuron::default()); },
                                1 => (),
                                _ => panic!(),
                            }
                        },
                    },
                }
            }
        }
        
        res.integrety()
    }
}

impl Hash for NeuralNet {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for layer in self.layers.iter() {
            for (index, neuron) in layer.iter() {
                index.hash(hasher);
                neuron.hash(hasher);
            }
        }
    }
}
