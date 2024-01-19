use crate::error::{Componente, ErrorRes, Tipo};
use crate::network::Network;
use std::io;
use rand::Rng;

mod network;
mod layer;
mod neuron;
mod error;


fn main() {

    /*
        Piccola implementazione di testing per varificare la corretteza delle funzioni di supporto per l'inserimento manuale di pesi e avlori
        all'interno del Neurone.
    */
    // let input_dim: i32 = 2;
    // let layer_vec: Vec<i32> = vec![input_dim, 3, 2];
    // let mut n: Network = Network::new(layer_vec.len() as i32, layer_vec);
    //
    // let intra_weights: Vec<Vec<Vec<f64>>> =
    //     vec![vec![vec![0.0, -0.4], vec![-0.4, 0.0]],
    //         vec![vec![0.0, -0.5, -0.2], vec![-0.6, 0.0, -0.3], vec![-0.5, -0.2, 0.0]],
    //         vec![vec![0.0, -0.2], vec![-0.1, 0.0]]];
    //
    // let extra_weights: Vec<Vec<Vec<f64>>> =
    //     vec![vec![vec![]],
    //         vec![vec![1.0, 1.0], vec![0.5, 1.0], vec![1.0, 2.0]],
    //         vec![vec![1.0, 0.5, 1.0], vec![2.0, 1.0, 1.0]]];
    //
    // n.init_values_defined(0.2, 0.1, 0.2, 0.1);
    //
    // n.init_weights_defined(extra_weights, intra_weights);
    //
    // println!("{}", n);
    //
    // let input: Vec<Vec<u8>> = vec![vec![1, 1],
    //                                vec![1, 1],
    //                                vec![1, 1],
    //                                vec![1, 1],];
    // let time: Vec<i32> = vec![1, 2, 3, 4];
    //
    // let components: Componente = Componente::None;
    // let tipo: Tipo = Tipo::None;
    // let bit_number : i32 = 0; // 64 bit --> la mantissa è fino al 51 bit, dal 52 esimo al 63 esimo è l'esponente, il 64 esimo è il segno
    // let error_res = ErrorRes::new(0, 0, components, tipo, -1, bit_number, 5);
    //
    // let res = n.process(input, time, error_res);
    // println!("Res: {:?}", res);


    //----------------------------------------------------------

    // Inizializziamo il generatore di valori casuali
    let mut rng = rand::thread_rng();

    // Definizione della rete
    let mut layer_vec: Vec<i32> = vec![3, 4, 5, 2];
    let mut choice = String::new();
    println!("Vuoi definire la struttura della rete o usarne una standars (strutturan standard: 3, 4, 5, 2)? [y/n]");
    io::stdin()
        .read_line(&mut choice)
        .expect("Lettura fallita");
    if choice.trim().eq_ignore_ascii_case("y") {
        layer_vec = define_network();
    }

    let mut n: Network = Network::new(layer_vec.len() as i32, layer_vec);
    n.init_weight_randomly((0.0, 1.0));

    let mut choice = String::new();
    println!("Vuoi cambiare i valori di default dei potenziali dei neuroni (attualmente: Soglia = 0.2, Reset = 0.0, Riposo = 0.3, Tau = 0.5) [y/n]");
    io::stdin()
        .read_line(&mut choice)
        .expect("Lettura fallita");
    if choice.trim().eq_ignore_ascii_case("y") {
        let t = define_values();
        n.init_values_defined(t.0, t.1, t.2, t.3);
    }

    println!("{}", n);

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

    // Definizione dei componente da testare
    let componente: Vec<Componente> = get_error_component();

    // Calcolo output corretti
    let no_error = ErrorRes::new(0, 0, Componente::None, Tipo::None, 0, 0, 0);
    let result_correct = n.process(input.clone(), input_time.clone(), no_error);

    // Misurazione resilienza
    let mut resilienza = 0;
    for i in 0..iteration_number{
        let componente: Componente = *componente.get(rng.gen_range(0..componente.len())).unwrap();
        let layer_id: usize = if componente == Componente::PesiE { rng.gen_range(1..(n.number_of_layer as usize)) } else { rng.gen_range(0..(n.number_of_layer as usize)) };
        let neuron_id: usize = rng.gen_range(0..((*n.layer_array.get(layer_id).unwrap()).neuron_number as usize));
        let weight_id: i32 = if componente == Componente::PesiE {
            rng.gen_range(0..(n.layer_array.get(layer_id).unwrap().dim_layer_prec as i32))
        } else if componente == Componente::PesiI{
            rng.gen_range(0..(n.layer_array.get(layer_id).unwrap().neuron_number as i32))
        } else {
            0
        };
        let bit_position: i32 = rng.gen_range(52..64);           // Se non crea spike limitare il range solo all'esponente (bit 52, 62)
        let time: i32 = if tipo == Tipo::Flip { rng.gen_range(1..=input_number) } else  { 0 };
        let error_resilience = ErrorRes::new(neuron_id, layer_id, componente, tipo, weight_id, bit_position, time);

        let result = n.process(input.clone(), input_time.clone(), error_resilience);

        let resilienza_single = error::error_computation(result_correct.0.clone(), result.0.clone());

        println!("Iteration number {}\nError configuration: {}\nError detected = {}\n", i, error_resilience, (input_number - resilienza_single));

        resilienza += resilienza_single;
    }

    let resilienza: f64 = (resilienza as f64) / ((input_number * iteration_number) as f64);
    println!("Resilienza: {}", resilienza);
}

fn define_values() -> (f64, f64, f64, f64) {
    fn read_user_input(prompt: &str) -> f64 {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        input.trim().parse::<f64>().expect("Failed to parse floating-point number")
    }

    let value1: f64 = read_user_input("Inserisci il nuovo valore di Soglia:");
    let value2: f64 = read_user_input("Inserisci il nuovo valore di Reset:");
    let value3: f64 = read_user_input("Inserisci il nuovo valore di Riposo:");
    let value4: f64 = read_user_input("Inserisci il nuovo valore di Tau:");

    (value1, value2, value3, value4)
}

fn define_network() -> Vec<i32>{
    let mut layer_vec: Vec<i32> = Vec::new();
    println!("Inserire il numero di Neuroni per ogni layer. '0' per terminare");
    loop {
        let mut layer_dim = String::new();
        io::stdin()
            .read_line(&mut layer_dim)
            .expect("Lettura fallita");
        let layer_dim = layer_dim.trim().parse::<i32>().expect("Input non valido");
        if layer_dim == 0 && !layer_vec.is_empty(){
            return layer_vec.clone();
        }
        else if layer_dim < 0{
            println!("Input non valido");
        }
        else {
            layer_vec.push(layer_dim);
        }
    }
}


fn get_error_component() -> Vec<Componente>{
    let mut components: Vec<Componente> = Vec::new();

    println!("Seleziona i componente da testare:");
    println!("0: Soglia");
    println!("1: Riposo");
    println!("2: Reset");
    println!("3: Memorizzato");
    println!("4: Pesi intra");
    println!("5: Pesi extra");
    println!("6: Sommatore");
    println!("7: Moltiplicatore");
    println!("8: Comparatore");
    println!("9: Fine");

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
