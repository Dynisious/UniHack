
#[derive(Clone)]
pub struct Neuron {
    threshold: f32,
    outputs: Vec<(usize, usize)>,
    pub state: f32,
}

impl Neuron {
    pub fn new(threshold: f32, outputs: Vec<(usize, usize)>, state: f32) -> Self {
        Self { threshold, outputs, state }
    }
    pub fn clean(mut self) -> Self {
        self.state = 0.0; self
    }
    pub fn normalise(mut self) -> Self {
        self.state = f32::max(0.0, self.state - self.threshold); self
    }
}

impl Default for Neuron {
    fn default() -> Self {
        Self::new(0.0, Vec::with_capacity(0), 0.0)
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
