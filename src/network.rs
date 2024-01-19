use crate::layer::Layer;
use std::fmt;
use std::sync::{mpsc, Mutex, Arc};
//use std::sync::mpsc::Sender;
use std::thread;
use crate::error::ErrorRes;

/*
    Struttura della Rete nella sua interezza:
    - number_of_layer contiene il numero di Layer presenti nella rete
    - layer_array è il vettore contenente tutti i Layer
 */
pub struct Network{
    pub number_of_layer: i32,
    pub layer_array: Vec<Layer>,
}

impl Network{
    /*
        In fase di creazione della rete creiamo in cascata tutti i Layer e li inseriamo nel vettore
     */
    pub fn new(nl:i32, vec: Vec<i32>) -> Self{
        let mut layer_vec: Vec<Layer> = Vec::new();
        let mut dim_layer_prec: i32;
        for (i, &value) in vec.iter().enumerate() {
            if layer_vec.is_empty(){
                dim_layer_prec = 0;
            }
            else {
                dim_layer_prec = layer_vec.last().unwrap().neuron_number;
            }
            let tmp: Layer = Layer::new(i, value, dim_layer_prec);
            layer_vec.push(tmp);
        }

        Network{number_of_layer:nl,layer_array:layer_vec}
    }

    /*
        Funzione per inizializzare i pesi in modo casuale
     */
    pub fn init_weight_randomly(&mut self, range: (f64, f64)){
        for lasagna in self.layer_array.iter_mut(){
            lasagna.init_weights_randomly(range);
        }
    }

    /*
        Funzione di supporto per inizializzare i pesi con valori predefiniti
     */
    pub fn init_weights_defined(&mut self, extra_weights: Vec<Vec<Vec<f64>>>, intra_weights: Vec<Vec<Vec<f64>>>){
        for (indice, layer) in self.layer_array.iter_mut().enumerate(){
            layer.init_weights_defined(extra_weights.get(indice).unwrap().clone(), intra_weights.get(indice).unwrap().clone());
        }
    }

    /*
        Funzione per la modifica dei valori di potenziale all'interno dei Neuroni, li modifichiamo tutti con gli stessi valori
     */
    pub fn init_values_defined(&mut self, soglia: f64, reset: f64, riposo: f64, tau: f64) {
        for layer in self.layer_array.iter_mut(){
            layer.init_values_defined(soglia, reset, riposo, tau);
        }
    }

    /*
        Funzione principale per la gestione della rete e la propagazione degli ingressi al primo Layer
     */
    pub fn process(&mut self, input_v: Vec<Vec<u8>>, input_t: Vec<i32>, error_res: ErrorRes) -> (Vec<Vec<u8>>, Vec<i32>){
        let mut handles = Vec::new();

        // Creiamo il primo canale prima del ciclo for perché ci servirà fuori dallo scope per inoltrare gli ingressi al primo Layer
        let (sender, mut r) = mpsc::channel::<(Vec<u8>, i32)>();

        /*
            Iteriamo su tutti i Layer per creare i canali e i Threads corrsipondenti.
            Abbiamo adottato una strategia che tratta i Layer come più piccola unità e creiamo un Thread per ogni Layer.
            I Layer propagano e gestiscono e propagano, tra i vari Neuroni, gli input spike in modo autonomo.
         */
        for layer in self.layer_array.iter_mut() {
            // Creiamo il canale che metterà in comunicazione il Layer attuale con il successivo, di conseguenza il Receiver verrà passato al prissimo Layer
            let (s, r_next) = mpsc::channel::<(Vec<u8>, i32)>();

            // Cloniamo il layer per evitare delle Race Condition così che ogni Thread abbia la copia del proprio Layer
            let mut layer_cloned = layer.clone();

            let handle = thread::spawn(move || {
                layer_cloned.process(Arc::new(Mutex::new(r)), s, error_res);
            });
            r = r_next;

            handles.push(handle);
        }

        /*
            Propaghiamo tutti i vettori in ingresso al primo canale
         */
        for (v, &t) in input_v.iter().zip(input_t.iter()){
            sender.send((v.clone(), t));
        }

        /*
            Chiudiamo il canale manualmente in modo che in cascata i canali vengano chiusi, le funzioni 'process' dei Layer terminino e quindi i thread possano terminare.
         */
        drop(sender);

        let mut result: (Vec<Vec<u8>>, Vec<i32>) = (Vec::new(), Vec::new());
        let mut dif = 0;
        let segnaposto: Vec<u8> = vec![0; self.layer_array.last().unwrap().neuron_number as usize];
        //Intanto che riceviamo gli Output dalla rete ci preoccupiamo di riempire gli sapzi mancanti nella matrice risultato in modo da avere l'output formattato sempre allo stesso modo
        while let Ok(output) = r.recv() {
            if output.1 >= 1 && result.1.is_empty() {
                dif = output.1 - 1;
                for i in 0..dif {
                    result.0.push(segnaposto.clone());
                    result.1.push(i + 1);
                }
            } else {
                dif = output.1 - result.1.last().unwrap();
                if dif > 1 {
                    let last_time = *result.1.last().unwrap();
                    for i in 0..(dif - 1) {
                        result.0.push(segnaposto.clone());
                        result.1.push(last_time + i + 1);
                    }
                }
            }
            result.0.push(output.0);
            result.1.push(output.1);
        }
        dif = (input_t.len() - result.1.len()) as i32;
        if dif > 0 {
            let mut last_time = 0;
            if !result.1.is_empty(){
                last_time = *result.1.last().unwrap();
            }
            for i in 0..dif {
                result.0.push(segnaposto.clone());
                result.1.push(last_time + i + 1);
            }
        }

        /*
            Attendiamo che tutti i Thread abbino terminato
         */
        for handle in handles{
            handle.join().unwrap();
        }

        result
    }
}

impl fmt::Display for Network{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number of layer: {}\n", self.number_of_layer)?;
        for layer in self.layer_array.iter(){
            write!(f, "{}\n", layer)?;
        }
        Ok(())
    }
}