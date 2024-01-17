use crate::layer::Layer;
use std::fmt;
use std::sync::{mpsc, Mutex, Arc};
//use std::sync::mpsc::Sender;
use std::thread;
use crate::error::Error_res;

pub struct Network{
    pub number_of_layer: i32,
    pub layer_array: Vec<Layer>,
    //pub sender: mpsc::Sender<(Vec<u8>, i32)>,
    //pub receiver: Arc<Mutex<mpsc::Receiver<(Vec<u8>, i32)>>>
}

impl Network{
    pub fn new(nl:i32, vec: Vec<i32>) -> Self{
        let mut layer_vec: Vec<Layer> = Vec::new();
        let mut dim_layer_prec: i32;

        //Layer's communication channel definition
        /*let mut sender_array = Vec::new();
        let mut receiver_array = Vec::new();
        for _ in 0..(nl + 1){
            let (s, r) = mpsc::channel::<(Vec<u8>, i32)>();
            sender_array.push(s);
            receiver_array.push(Arc::new(Mutex::new(r)));
        }*/

        for (i, &value) in vec.iter().enumerate() {             // se il valore implementa il tratto copy allora lo copia e non serve &
            if layer_vec.is_empty(){
                dim_layer_prec = 0;
            }
            else {
                dim_layer_prec = layer_vec.last().unwrap().neuron_number;
                dim_layer_prec = layer_vec.last().unwrap().neuron_number;
            }
            let tmp: Layer = Layer::new(i, value, dim_layer_prec/*, receiver_array.get(i as usize).unwrap().clone(), sender_array.get((i + 1) as usize).unwrap().clone()*/);
            layer_vec.push(tmp);
        }

        Network{number_of_layer:nl,layer_array:layer_vec/*, sender:sender_array.get(0).unwrap().clone(), receiver: receiver_array.get(nl as usize).unwrap().clone()*/}
    }

    pub fn init_weight_randomly(&mut self, range: (f64, f64)){                                     // Da &mut self a self
        for lasagna in self.layer_array.iter_mut(){                     //Da iter_mut a iter e da &mut mut lasagna a lasagna
            lasagna.init_weights_randomly(range);
        }
    }

    pub fn init_weights_defined(&mut self, extra_weights: Vec<Vec<Vec<f64>>>, intra_weights: Vec<Vec<Vec<f64>>>){
        for (indice, layer) in self.layer_array.iter_mut().enumerate(){
            layer.init_weights_defined(extra_weights.get(indice).unwrap().clone(), intra_weights.get(indice).unwrap().clone());
        }
    }

    pub fn init_values_defined(&mut self, soglia: f64, reset: f64, riposo: f64, tau: f64) {
        for layer in self.layer_array.iter_mut(){
            layer.init_values_defined(soglia, reset, riposo, tau);
        }
    }

    pub fn process(&mut self, input_v: Vec<Vec<u8>>, input_t: Vec<i32>, error_res: Error_res){
        //let mut input_data = input;
        let mut handles = Vec::new();

        /*let mut sender_array = Vec::new();
        let mut receiver_array = Vec::new();
        for _ in 0..(self.number_of_layer + 1){
            let (s, r) = mpsc::channel::<(Vec<u8>, i32)>();
            sender_array.push(s);
            receiver_array.push(Arc::new(Mutex::new(r)));
        }*/
        let (sender, mut r) = mpsc::channel::<(Vec<u8>, i32)>();

        // Iterate through layers and create threads
        for (index, layer) in self.layer_array.iter_mut().enumerate() {
            //let data_to_send = input_v.clone();
            //let sender_clone = self.sender.clone();
            //let receiver_clone = Arc::clone(&layer.receiver);
            //let rec = receiver_array.get(index).unwrap().clone();
            //let send = sender_array.get(index + 1).unwrap().clone();
            let (s, r_next) = mpsc::channel::<(Vec<u8>, i32)>();

            let mut layer_cloned = layer.clone();

            let handle = thread::spawn(move || {

                // Send data to the next layer
                //sender_clone.send((data_to_send, input_t)).unwrap();

                // Receive data from the previous layer
                layer_cloned.process(Arc::new(Mutex::new(r)), s, error_res);
                //layer_cloned.process(receiver_array.get(index + 1).unwrap().clone(), sender_array.get(index).unwrap().clone());

                //let received_data = receiver_clone.lock().unwrap().recv().unwrap();
                //println!("Layer {} received data: {:?}", layer.neuron_numer, received_data);
            });
            r = r_next;

            handles.push(handle);
            //input_data = Vec::new(); // Clear input for the next layer
        }

        for (v, &t) in input_v.iter().zip(input_t.iter()){
            sender.send((v.clone(), t));
        }

        /*for (&ref v, &t) in input_v.iter().zip(input_t.iter()){
            sender.send((v.clone(), t));
        }*/
        //sender.send((input_v.clone(), input_t));
        //self.sender.send((input_v.clone(), input_t));
        drop(sender);
        //sender_array.get(0).unwrap().close();
        //let terminazione: i32 = -1;
        //sender_array.get(0).unwrap().send((Vec::new(), terminazione));

        for handle in handles{
            handle.join().unwrap();
        }


        // Simulate the final processing or use the received data
        //let final_result = r.recv().unwrap();
        //let final_result = receiver.lock().unwrap().recv().unwrap();

        //let mut _result: (Vec<Vec<u8>>, Vec<i32>) = (Vec::new(), Vec::new());
        while let Ok(output) = r.recv(){
            //result.0.push(output.0);
            //result.1.push(output.1);
            println!("Output -> {:?}", output);
        }


    }
}

impl fmt::Display for Network{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Network dim: {}. Layer:\n", self.number_of_layer)?;
        for layer in self.layer_array.iter(){
            write!(f, "{}", layer)?;
        }
        Ok(())
    }
}