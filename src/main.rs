use crate::network::Network;

mod network;
mod layer;
mod neuron;

fn main() {
    //let input_dim: i32 = 2;
    //let layer_vec: Vec<i32> = vec![input_dim, 3, 2];
    //let mut n: Network = Network::new(layer_vec.len() as i32, layer_vec);            // Ho tolto mut davanti a n

    //n.init_weight_randomly();

    // let intra_weights: Vec<Vec<Vec<f64>>> =
    //     vec![vec![vec![0.0, -1.0], vec![-1.0, 0.0]],
    //         vec![vec![0.0, -0.5, -1.0], vec![-0.5, 0.0, -1.0], vec![-0.5, -1.0, 0.0]],
    //         vec![vec![0.0, -0.2], vec![-0.8, 0.0]]];
    //
    // let extra_weights: Vec<Vec<Vec<f64>>> =
    //     vec![vec![vec![]],
    //         vec![vec![1.0, 1.0], vec![0.5, 1.0], vec![1.0, 2.0]],
    //         vec![vec![1.0, 0.5, 1.0], vec![2.0, 1.0, 1.0]]];

    let input_dim: i32 = 2;
    let layer_vec: Vec<i32> = vec![input_dim, 1];
    let mut n: Network = Network::new(layer_vec.len() as i32, layer_vec);            // Ho tolto mut davanti a n

    let intra_weights: Vec<Vec<Vec<f64>>> =
        vec![vec![vec![0.0, -1.0], vec![-1.0, 0.0]],
            vec![vec![0.0]]];

    let extra_weights: Vec<Vec<Vec<f64>>> =
        vec![vec![vec![]],
            vec![vec![1.0, 1.0]]];

    n.init_weights_defined(extra_weights, intra_weights);

    let input: Vec<Vec<u8>> = vec![vec![1, 1],
                                   vec![1, 1],
                                   vec![1, 1],
                                   vec![1, 1],];
    let time: Vec<i32> = vec![1, 2, 3, 4];

    n.process(input, time);


    println!("{}", n);

    println!("Hello, world!");
}
