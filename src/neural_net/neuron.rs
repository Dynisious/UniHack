
use std::hash::{Hash, Hasher};

#[derive(Clone, Serialize, Deserialize)]
pub struct Neuron {
    pub threshold: f32,
    pub outputs: Vec<(usize, f32)>,
    pub state: f32,
}

impl Neuron {
    pub fn new(threshold: f32, outputs: Vec<(usize, f32)>, state: f32) -> Self {
        Self { threshold, outputs, state }
    }
    pub fn clean(mut self) -> Self {
        self.state = 0.0; self
    }
    pub fn normalise(mut self) -> Self {
        self.state = f32::max(0.0, self.state - self.threshold); self
    }
    pub fn reproduce(mut self, other: &Self) -> Self {
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::default();
        
        self.hash(&mut hasher);
        
        match hasher.finish() % 3 {
            0 => self.threshold = (self.threshold + other.threshold) / 2.0,
            1 => self.threshold = other.threshold,
            2 => (),
            _ => panic!(),
        }
        
        self.outputs = self.outputs.iter()
            .filter_map(|(res_index, mut res_weight)| {
                res_index.hash(&mut hasher);
                res_weight.to_bits().hash(&mut hasher);
                
                if let Some((other_index, other_weight)) = other.outputs.iter()
                    .find(|(other_index, _)| other_index == res_index) {
                    other_index.hash(&mut hasher);
                    other_weight.to_bits().hash(&mut hasher);
                    
                    match hasher.finish() % 3 {
                        0 => res_weight = (res_weight + other_weight) / 2.0,
                        1 => res_weight = *other_weight,
                        2 => (),
                        _ => panic!(),
                    }
                } else {
                    match hasher.finish() % 3 {
                        0 => res_weight = res_weight + (((hasher.finish() as f32 / 1000.0) % 3.0) - 1.0),
                        1 => (),
                        2 => return None,
                        _ => panic!(),
                    }
                }
                Some((*res_index, res_weight))
            }).collect();
        
        other.hash(&mut hasher);
        if hasher.finish() % 2 == 0 {
            self.outputs.push((
                hasher.finish() as usize % super::LAYER_SIZE,
                ((hasher.finish() as f32 / 1000.0) % 3.0) - 1.0
            ));
        }
        self.outputs.sort_by(|(index_a, _), (index_b, _)| index_a.cmp(&index_b));
        self.outputs.dedup_by(|(index_a, _), (index_b, _)| index_a == index_b);
        
        self.state = 0.0;
        self
    }
}

impl Default for Neuron {
    fn default() -> Self {
        Self::new(0.0, Vec::with_capacity(0), 0.0)
    }
}

impl Hash for Neuron {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.threshold.to_bits().hash(hasher);
        for (index, weight) in self.outputs.iter() {
            index.hash(hasher);
            weight.to_bits().hash(hasher);
        }
        self.state.to_bits().hash(hasher);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_neuron() {
        let mut neuron = Neuron::default();
        
        neuron.state = 1.0;
        assert_eq!(0.0, neuron.clone().clean().state, "`Neuron::clean` 1 failed.");
        
        neuron.state = 0.0;
        assert_eq!(0.0, neuron.clone().clean().state, "`Neuron::clean` 2 failed.");
        
        neuron.state = -1.0;
        assert_eq!(0.0, neuron.clone().clean().state, "`Neuron::clean` 3 failed.");
        
        neuron.state = 1.0;
        assert_eq!(1.0, neuron.clone().normalise().state, "`Neuron::normalise` 1 failed.");
        
        neuron.state = 0.0;
        assert_eq!(0.0, neuron.clone().normalise().state, "`Neuron::normalise` 2 failed.");
        
        neuron.state = -1.0;
        assert_eq!(0.0, neuron.clone().normalise().state, "`Neuron::normalise` 3 failed.");
    }
}
