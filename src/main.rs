use crate::error::{Componente, Error_res, Tipo};
use crate::network::Network;
use crate::neuron::Neuron;
use std::io;
use rand::Rng;
use crate::Componente::{Comparatore, PesiI};

mod network;
mod layer;
mod neuron;
mod error;

/*
 - Fare file di testo e caricare e salvare la rete
 - Chiedere all'utente che errore vuole usare e che componente vuole utilizzare
 - numero di iterazioni che vuole provare
 - definire il calcolo della resilienza  (rete senza errore vs rete con errore)
 - fare una media ad esempio su 50 ingressi abbiamo un errore pari a 2

 */
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
    let mut n: Network = Network::new(layer_vec.len() as i32, layer_vec);           // inizializza la rete con parametri standard
    //n.init_values_defined(0.4, 0.1, 0.3, 0.2);
    let intra_weights: Vec<Vec<Vec<f64>>> =
        vec![vec![vec![0.0, -1.0], vec![-1.0, 0.0]],
            vec![vec![0.0]]];

    let extra_weights: Vec<Vec<Vec<f64>>> =
        vec![vec![vec![]],
            vec![vec![1.0, 1.0]]];

    //n.init_weights_defined(extra_weights, intra_weights);
    n.init_weight_randomly((0.0, 2.0));

    let input: Vec<Vec<u8>> = vec![vec![1, 1],
                                   vec![1, 1],
                                   vec![1, 1],
                                   vec![1, 1],];
    let time: Vec<i32> = vec![1, 2, 3, 4];

    let components: Componente = Componente::None;
    let tipo: Tipo = Tipo::StuckAt1;
    let bit_number : i32 = 52; // 64 bit --> la mantissa è fino al 51 bit, dal 52 esimo al 63 esimo è l'esponente, il 64 esimo è il segno
    let error_res = Error_res::new(0, 0, components, tipo, -1, bit_number, 5);

    n.process(input, time, error_res);


    println!("{}", n);


    //----------------------------------------------------------


    // Inizializziamo il generatore di valori casuali
    let mut rng = rand::thread_rng();

    // Definizione della rete




    // Definizione delgi input in mod randomico
    println!("Inserire in numero di input");
    let mut input_number = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Lettura fallita");
    let input_number = input_number.trim().parse::<i32>().expect("Input non valido");
    let mut input: Vec<Vec<u8>> = Vec::new();
    let mut input_time: Vec<i32> = Vec::new();
    for i in 1..=input_number{
        let mut tmp: Vec<u8> = Vec::new();
        for _ in 0..n.layer_array.get(0).unwrap().neuron_number{
            tmp.push(rng.gen_range(0..=1));
        }
        input.push(tmp);
        input_time.push(i);
    }

    // Definizione del numero di iterazioni per valutare la resilienza della rete
    println!("Inserire in numero di iterazioni");
    let mut iteration_number = String::new();
    io::stdin()
        .read_line(&mut iteration_number)
        .expect("Lettura fallita");
    let iteration_number = iteration_number.trim().parse::<i32>().expect("Input non valido");

    // Definizione del tipo di errore da testare
    let tipo: Tipo = get_error_type();

    // Definizione dei componenti da testare
    let componenti: Vec<Componente> = get_error_component();

    // Calcolo output corretti
    let no_error = Error_res::new(0, 0, Componente::None, Tipo::None, 0, 0, 0);
    let result_correct = n.process(input.clone(), input_time.clone(), no_error);

    // Misurazione resilienza
    let mut resilienza = 0;
    for _ in 0..iteration_number{
        let componente: Componente = *componenti.get(rng.gen_range(0..componenti.len())).unwrap();
        let layer_id: usize = if componente == Componente::PesiE { rng.gen_range(1..=(n.number_of_layer as usize)) } else { rng.gen_range(0..=(n.number_of_layer as usize)) };
        let neuron_id: usize = rng.gen_range(0..=(n.layer_array.get(layer_id).unwrap().neuron_number as usize));
        let weight_id: i32 = if componente == Componente::PesiE {
            rng.gen_range(0..=(n.layer_array.get(layer_id).unwrap().dim_layer_prec as i32))
        } else if componente == Componente::PesiI{
            rng.gen_range(0..=(n.layer_array.get(layer_id).unwrap().neuron_number as i32))
        } else {
            0
        };
        let bit_position: i32 = rng.gen_range(0..64);           // Se non crea spike limitare il range solo all'esponente (bit 52, 62)
        let time: i32 = if tipo == Tipo::Flip { rng.gen_range(0..=input_number) } else  { 0 };
        let error_resilience = Error_res::new(neuron_id, layer_id, componente, tipo, weight_id, bit_number, time);

        let result = n.process(input.clone(), input_time.clone(), error_resilience);

        resilienza += error_computation(result_correct.clone(), result.clone());
    }

    let resilienza: f64 = (resilienza as f64) / (input_number * iteration_number);
}

fn get_error_component() -> Vec<Componente>{
    let mut components: Vec<Componente> = Vec::new();

    println!("Seleziona i componenti da testare:");
    println!("\t0: Soglia");
    println!("\t1: Riposo");
    println!("\t2: Reset");
    println!("\t3: Memorizzato");
    println!("\t4: Pesi intra");
    println!("\t5: Pesi extra");
    println!("\t6: Sommatore");
    println!("\t7: Moltiplicatore");
    println!("\t8: Comparatore");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Lettura Fallita");
        match input.trim() {
            "0" => {
                if components.iter().any(|e| e == &Componente::Soglia){
                    println!("Già presente");
                } else {
                    components.push(Componente::Soglia);
                }
            },
            "1" => {
                if components.iter().any(|e| e == &Componente::Riposo){
                    println!("Già presente");
                } else {
                    components.push(Componente::Riposo);
                }
            },
            "2" => {
                if components.iter().any(|e| e == &Componente::Reset){
                    println!("Già presente");
                } else {
                    components.push(Componente::Reset);
                }
            },
            "3" => {
                if components.iter().any(|e| e == &Componente::Memorizzato){
                    println!("Già presente");
                } else {
                    components.push(Componente::Memorizzato);
                }
            },
            "4" => {
                if components.iter().any(|e| e == &Componente::PesiI){
                    println!("Già presente");
                } else {
                    components.push(Componente::PesiI);
                }
            },
            "5" => {
                if components.iter().any(|e| e == &Componente::PesiE){
                    println!("Già presente");
                } else {
                    components.push(Componente::PesiE);
                }
            },
            "6" => {
                if components.iter().any(|e| e == &Componente::Sommatore){
                    println!("Già presente");
                } else {
                    components.push(Componente::Sommatore);
                }
            },
            "7" => {
                if components.iter().any(|e| e == &Componente::Moltiplicatore){
                    println!("Già presente");
                } else {
                    components.push(Componente::Moltiplicatore);
                }
            },
            "8" => {
                if components.iter().any(|e| e == &Componente::Comparatore){
                    println!("Già presente");
                } else {
                    components.push(Componente::Comparatore);
                }
            },
            "9" => return components,
            _ => println!("Input non valido"),
        }
    }
}

fn get_error_type() -> Tipo {
    println!("\n Seleziona il tipo di ERRORE:");
    println!("0. Stuckat0");
    println!("1. Stuckat1");
    println!("2. BitFlip");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Lettura Fallita");

        match input.trim() {
            "0" => return Tipo::StuckAt0,
            "1" => return Tipo::StuckAt1,
            "2" => return Tipo::Flip,
            _ => println!("Input sbagliato, sono possibili solo: 0-> Stuckat0, 1-> Stuckat1, 2-> Bitflip"),
        }
    }
}

fn error_computation(correct: Vec<Vec<u8>>, wrong: Vec<Vec<u8>>) -> i32{
    let mut v = 0;
    let mut tmp: bool = true;
    for (c, w) in correct.iter().zip(wrong.iter()){
        tmp = true;
        for (cc, ww) in c.iter().zip(w.iter()){
            if cc != ww { tmp = false };
        }
        if tmp { v += 1 }
    }
    v
}

