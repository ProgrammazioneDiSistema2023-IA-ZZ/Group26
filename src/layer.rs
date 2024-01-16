use crate::neuron::Neuron;
use std::fmt;
use std::mem::needs_drop;
use std::sync::{mpsc, Mutex, Arc};
use crate::error::Error_res;

#[derive(Clone)]
pub struct Layer{
    pub neuron_numer: i32,
    pub neurons: Vec<Neuron>,
    pub index:  usize
    //pub receiver: Arc<Mutex<mpsc::Receiver<(Vec<u8>, i32)>>>,
    //pub sender: mpsc::Sender<(Vec<u8>, i32)>
}

impl Layer{
    pub fn new(layer_index: usize, dim: i32, dim_layer_prec: i32/*, receiver: Arc<Mutex<mpsc::Receiver<(Vec<u8>, i32)>>>, sender: mpsc::Sender<(Vec<u8>, i32)>*/) -> Self{
        let mut neuron_array: Vec<Neuron> = Vec::new();

        for i in 0..dim {
            let tmp: Neuron = Neuron::new(layer_index, i as usize, dim_layer_prec, dim);
            neuron_array.push(tmp);
        }

        Layer{index: layer_index, neuron_numer:dim, neurons:neuron_array/*, receiver, sender*/}
    }

    pub fn init_weights_randomly(&mut self){                                     // da &mut self a self
        for neuron in self.neurons.iter_mut(){                          // da iter_mut a iter e da &mut mut neuron a neuron
            neuron.init_weights_random();
        }
    }

    pub fn init_weights_defined(&mut self, extra_weights: Vec<Vec<f64>>, intra_weights: Vec<Vec<f64>>){
        let ref t: Vec<f64> = Vec::new();
        for (indice, neuron) in self.neurons.iter_mut().enumerate() {
            neuron.init_weights_defined(intra_weights.get(indice).unwrap().clone(), extra_weights.get(indice).unwrap_or(t).clone());
        }
    }

    pub fn process(&mut self, receiver: Arc<Mutex<mpsc::Receiver<(Vec<u8>, i32)>>>, sender: mpsc::Sender<(Vec<u8>, i32)>, error_res: Error_res){
        let mut output: Vec<u8> = Vec::new();
        let mut previous_spikes: Vec<u8> = Vec::new();

        while let Ok(data_in) = receiver.lock().unwrap().recv() {
            if data_in.0.iter().any(|&x| x == 1) || output.iter().any(|&x| x == 1) {
                for neuron in self.neurons.iter_mut() {
                    output.push(neuron.process(data_in.0.clone(), previous_spikes.clone(), data_in.1, error_res));
                }
            }
            println!("{}, {:?}", self.index, output);

            if output.iter().any(|&x| x == 1) {                         // controllo ridondante per ecvitari di impegnare il canale inutilmente
                sender.send((output.clone(), data_in.1));
            }
            previous_spikes = output.clone();
            //std::mem::swap(&previous_spikes, &output);
            output.clear();
            //println!("Layer number: non si sa, {:?}", previous_spikes);

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