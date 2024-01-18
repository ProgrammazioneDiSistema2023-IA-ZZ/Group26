use crate::neuron::Neuron;
use std::fmt;
//use std::mem::needs_drop;
use std::sync::{mpsc, Mutex, Arc};
use crate::error::ErrorRes;

#[derive(Clone)]
pub struct Layer{
    pub neuron_number: i32,
    pub neurons: Vec<Neuron>,
    pub index:  usize,
    pub dim_layer_prec: i32,
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

        Layer{index: layer_index, neuron_number:dim, neurons:neuron_array, dim_layer_prec/*, receiver, sender*/}
    }

    pub fn init_weights_randomly(&mut self, range: (f64, f64)){                                     // da &mut self a self
        for neuron in self.neurons.iter_mut(){                          // da iter_mut a iter e da &mut mut neuron a neuron
            neuron.init_weights_random(range);
        }
    }

    pub fn init_weights_defined(&mut self, extra_weights: Vec<Vec<f64>>, intra_weights: Vec<Vec<f64>>){
        let ref t: Vec<f64> = Vec::new();
        for (indice, neuron) in self.neurons.iter_mut().enumerate() {
            neuron.init_weights_defined(intra_weights.get(indice).unwrap().clone(), extra_weights.get(indice).unwrap_or(t).clone());
        }
    }

    pub fn init_values_defined(&mut self, soglia: f64, reset: f64, riposo: f64, tau: f64){
        for neuron in self.neurons.iter_mut(){
            neuron.set_v_soglia(soglia);
            neuron.set_v_riposo(riposo);
            neuron.set_v_reset(reset);
            neuron.set_tau(tau);
        }
    }

    pub fn process(&mut self, receiver: Arc<Mutex<mpsc::Receiver<(Vec<u8>, i32)>>>, sender: mpsc::Sender<(Vec<u8>, i32)>, error_res: ErrorRes){
        let mut output: Vec<u8> = Vec::new();
        let mut previous_spikes: Vec<u8> = Vec::new();

        while let Ok(data_in) = receiver.lock().unwrap().recv() {
            if data_in.0.iter().any(|&x| x == 1) || output.iter().any(|&x| x == 1) {
                for neuron in self.neurons.iter_mut() {
                    output.push(neuron.process(data_in.0.clone(), previous_spikes.clone(), data_in.1, error_res));
                }
            }

            if output.iter().any(|&x| x == 1) {                         // controllo ridondante per ecvitari di impegnare il canale inutilmente
                sender.send((output.clone(), data_in.1));
            }
            previous_spikes = output.clone();
            output.clear();

        }
    }
}

impl fmt::Display for Layer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Layer number {} has {} neuron(s):\n\t", self.index, self.neuron_number)?;
        for (i, neuron) in self.neurons.iter().enumerate(){
            write!(f, "{}", neuron)?;
            if i < ((self.neuron_number - 1) as usize){
                write!(f, "\t")?;
            }
        }
        Ok(())
    }
}