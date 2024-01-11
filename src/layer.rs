use crate::neuron::Neuron;
use std::fmt;

#[derive(Clone)]
pub struct Layer{
    pub neuron_numer: i32,
    pub neurons: Vec<Neuron>
}

impl Layer{
    pub fn new(layer_index: usize, dim: i32, dim_layer_prec: i32) -> Self{
        let mut neuron_array: Vec<Neuron> = Vec::new();

        for i in 0..dim{
            let tmp: Neuron = Neuron::new(layer_index, i as usize, dim_layer_prec, dim);
            neuron_array.push(tmp);
        }

        Layer{neuron_numer:dim, neurons:neuron_array}
    }

    pub fn init_weights_randomly(&mut self){                                     // da &mut self a self
        for neuron in self.neurons.iter_mut(){                          // da iter_mut a iter e da &mut mut neuron a neuron
            neuron.init_weights_random();
        }
    }
}

impl fmt::Display for Layer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "\tLayer dim: {}. Neuron:\n\t\t", self.neuron_numer)?;
        for (i, neuron) in self.neurons.iter().enumerate(){
            write!(f, "{}", neuron)?;
            if i < ((self.neuron_numer - 1) as usize){
                write!(f, "\t\t")?;
            }
        }
        Ok(())
    }
}