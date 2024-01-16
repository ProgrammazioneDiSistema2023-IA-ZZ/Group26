use crate::error::{Error_res, Componente, Tipo};

use rand::Rng;
use std::fmt;


#[derive(Clone)]
pub struct Neuron{
    pub index: usize,
    pub layer_index: usize,
    pub v_soglia: f64,
    pub v_riposo: f64,
    pub v_reset: f64,
    pub v_memorizzato: f64,
    pub tau: f64,
    pub t_prec: i32,
    pub layer_prec_dim: i32,
    pub layer_actual_dim: i32,
    pub intra_weights: Vec<f64>,
    pub extra_weights: Vec<f64>
}

impl Neuron{
    pub fn new(layer_index: usize, index: usize, layer_prec_dim: i32, layer_actual_dim:i32) -> Self{
        Neuron{
            index,
            layer_index,
            v_soglia: 0.2,
            v_riposo: 0.3,
            v_reset: 0.0,
            v_memorizzato:0.0,
            tau: 0.5,
            t_prec: 0,
            layer_prec_dim,
            layer_actual_dim,
            intra_weights: Vec::new(),
            extra_weights: Vec::new()}
    }

    pub fn init_weights_random(&mut self){
        let mut rng = rand::thread_rng();
        for _ in 0..self.layer_actual_dim{
            self.intra_weights.push(rng.gen_range(0.0..1.0));
        }
        for _ in 0..self.layer_prec_dim{
            self.extra_weights.push(rng.gen_range(0.0..1.0));
        }
    }

    pub fn init_weights_defined(&mut self, intra_weights: Vec<f64>, extra_weights: Vec<f64>){
        self.intra_weights = intra_weights;
        self.extra_weights = extra_weights;
    }

    pub fn process(&mut self, spikes_extra: Vec<u8>, spikes_intra:Vec<u8>, time: i32, error_res: Error_res) -> u8{
        let mut ret_val= 0;
        let mut summation: f64 = 0.0;
        if (error_res.neuron_id == self.index && error_res.layer_id == self.layer_index) {
            if self.layer_index != 0 {
                for (index, &e) in spikes_extra.iter().enumerate() {
                    if (error_res.componenti == Componente::Pesi_e && index == error_res.weight_id as usize) {
                        summation += (e as f64) * (error_res.apply_error(*self.extra_weights.get(index).unwrap()));
                    } else {
                        summation += (e as f64) * (*self.extra_weights.get(index).unwrap());
                    }
                }
            } else {
                self.v_memorizzato = *spikes_extra.get(self.index).unwrap() as f64;
            }
            for (index, &e) in spikes_intra.iter().enumerate() {
                if index != self.index {
                    if (error_res.componenti == Componente::Pesi_i && index == error_res.weight_id as usize) {
                        summation += (e as f64) * (error_res.apply_error(*self.intra_weights.get(index).unwrap()));
                    } else {
                        summation += (e as f64) * (*self.intra_weights.get(index).unwrap());
                    }
                }
            }

            println!("\t{} {} {}", self.index, self.layer_index, time);
            println!("\t\tsummation: {}", summation);
            println!("\t\tvmem: {}", self.v_memorizzato);

            print!("{} {} prima: {}", self.index, self.layer_index, self.v_soglia);

            let new_v_mem = if error_res.componenti == Componente::Memorizzato { error_res.apply_error(self.v_memorizzato) } else { self.v_memorizzato };
            let new_v_th = if error_res.componenti == Componente::Soglia { error_res.apply_error(self.v_soglia) } else { self.v_soglia };
            let new_v_reset = if error_res.componenti == Componente::Reset { error_res.apply_error(self.v_reset) } else { self.v_reset };
            let new_v_rest = if error_res.componenti == Componente::Riposo { error_res.apply_error(self.v_riposo) } else { self.v_riposo };

            println!("\tdopo: {}", new_v_th);

            self.v_memorizzato = new_v_rest + (new_v_mem - new_v_rest) * (-(time - self.t_prec) as f64 / (self.tau)).exp() + summation;
            self.t_prec = time;
            if self.v_memorizzato >= new_v_th {
                ret_val = 1;
                self.v_memorizzato = new_v_reset;
            }
            println!("\t\tafter compare: {}, out:{}", self.v_memorizzato, ret_val);
        } else {
            if self.layer_index != 0 {
                for (index, &e) in spikes_extra.iter().enumerate() {
                    //println!("{index}, {e} at neuron {} of {} layer", self.index, self.layer_index);
                    summation += (e as f64) * (*self.extra_weights.get(index).unwrap());
                }
            } else {
                self.v_memorizzato = *spikes_extra.get(self.index).unwrap() as f64;
            }
            for (index, &e) in spikes_intra.iter().enumerate() {
                if index != self.index {
                    summation += (e as f64) * (*self.intra_weights.get(index).unwrap());
                }
            }

            println!("\t{} {} {}", self.index, self.layer_index, time);
            println!("\t\tsummation: {}", summation);
            println!("\t\tvmem: {}", self.v_memorizzato);

            // v_mem(ts) = v_rest + [v_mem(ts-1) - v_rest] * e^-((ts-(ts-1))/tau)
            self.v_memorizzato = self.v_riposo + (self.v_memorizzato - self.v_riposo) * (-(time - self.t_prec) as f64 / (self.tau)).exp() + summation;
            self.t_prec = time;
            if self.v_memorizzato >= self.v_soglia {
                ret_val = 1;
                self.v_memorizzato = self.v_reset;
            }

            println!("\t\tafter compare: {}, out:{}", self.v_memorizzato, ret_val);
        }
        println!("\n\n");
        ret_val
    }
}

impl fmt::Display for Neuron{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Neuron n:{} at layer n:{}, values:\n\t\t\t[v_th:{}, v_rest:{}, v_reset:{}, v_mem:{}, tau:{}]\n\t\t\tIntra weights: {:?}\n\t\t\tExtra weights: {:?}\n", self.index, self.layer_index, self.v_soglia, self.v_riposo, self.v_reset, self.v_memorizzato, self.tau, self.intra_weights, self.extra_weights)
    }
}