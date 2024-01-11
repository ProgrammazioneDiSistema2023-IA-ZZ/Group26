use crate::layer::Layer;
use std::fmt;

pub struct Network{
    pub number_of_layer: i32,
    pub layer_array: Vec<Layer>
}

impl Network{
    pub fn new(nl:i32, vec: Vec<i32>) -> Self{
        let mut layer_vec: Vec<Layer> = Vec::new();
        let mut dim_layer_prec: i32;
        for (i, &value) in vec.iter().enumerate() {             // se il valore implementa il tratto copy allora lo copia e non serve &
            if layer_vec.is_empty(){
                dim_layer_prec = 0;
            }
            else {
                dim_layer_prec = layer_vec.last().unwrap().neuron_numer;
            }
            let tmp: Layer = Layer::new(i, value, dim_layer_prec);
            layer_vec.push(tmp);
        }

        Network{number_of_layer:nl,layer_array:layer_vec}
    }

    pub fn init_weight_randomly(&mut self){                                     // Da &mut self a self
        for lasagna in self.layer_array.iter_mut(){                     //Da iter_mut a iter e da &mut mut lasagna a lasagna
            lasagna.init_weights_randomly();
        }
    }
}

impl fmt::Display for Network{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Network dim: {}. Layer:\n", self.number_of_layer)?;
        for layer in self.layer_array.iter(){
            write!(f, "{}", layer)?;
        }
        Ok(())
    }
}