use crate::network::Network;

mod network;
mod layer;
mod neuron;

fn main() {
    let input_dim: i32 = 2;
    let layer_vec: Vec<i32> = vec![input_dim, 3, 1];
    let mut n: Network = Network::new(layer_vec.len() as i32, layer_vec);            // Ho tolto mut davanti a n

    n.init_weight_randomly();

    let input: Vec<u8> = vec![1, 1];
    let time: i32 = 1;

    n.process(input, time);


    println!("{}", n);

    println!("Hello, world!");
}
