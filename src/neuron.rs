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
            v_soglia: 0.5,
            v_riposo: 0.3,
            v_reset: 0.0,
            v_memorizzato:0.0,
            tau: 0.5,
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