use crate::network::Network;

mod network;
mod layer;
mod neuron;

fn main() {
    let input_dim: i32 = 3;
    let layer_vec: Vec<i32> = vec![input_dim, 2, 4, 1];
    let mut n: Network = Network::new(2, layer_vec);            // Ho tolto mut davanti a n

    n.init_weight_randomly();


    println!("{}", n);

    println!("Hello, world!");
}
