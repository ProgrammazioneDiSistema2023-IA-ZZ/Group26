use crate::layer::Layer;
use std::fmt;
use std::sync::{mpsc, Mutex, Arc};
use std::thread;

pub struct Network{
    pub number_of_layer: i32,
    pub layer_array: Vec<Layer>,
    pub sender: mpsc::Sender<(Vec<u8>, i32)>,
    pub receiver: Arc<Mutex<mpsc::Receiver<(Vec<u8>, i32)>>>
}

impl Network{
    pub fn new(nl:i32, vec: Vec<i32>) -> Self{
        let mut layer_vec: Vec<Layer> = Vec::new();
        let mut dim_layer_prec: i32;

        //Layer's communication channel definition
        let mut sender_array = Vec::new();
        let mut receiver_array = Vec::new();
        for _ in 0..(nl + 1){
            let (s, r) = mpsc::channel::<(Vec<u8>, i32)>();
            sender_array.push(s);
            receiver_array.push(Arc::new(Mutex::new(r)));
        }

        for (i, &value) in vec.iter().enumerate() {             // se il valore implementa il tratto copy allora lo copia e non serve &
            if layer_vec.is_empty(){
                dim_layer_prec = 0;
            }
            else {
                dim_layer_prec = layer_vec.last().unwrap().neuron_numer;
            }
            let tmp: Layer = Layer::new(i, value, dim_layer_prec, receiver_array.get(i as usize).unwrap().clone(), sender_array.get((i + 1) as usize).unwrap().clone());
            layer_vec.push(tmp);
        }

        Network{number_of_layer:nl,layer_array:layer_vec, sender:sender_array.get(0).unwrap().clone(), receiver: receiver_array.get(nl as usize).unwrap().clone()}
    }

    pub fn init_weight_randomly(&mut self){                                     // Da &mut self a self
        for lasagna in self.layer_array.iter_mut(){                     //Da iter_mut a iter e da &mut mut lasagna a lasagna
            lasagna.init_weights_randomly();
        }
    }

    pub fn process(&self, input_v: Vec<u8>, input_t: i32){
        //let mut input_data = input;
        let mut handles = Vec::new();

        // Iterate through layers and create threads
        for layer in &self.layer_array {
            //let data_to_send = input_v.clone();
            //let sender_clone = self.sender.clone();
            //let receiver_clone = Arc::clone(&layer.receiver);
            let mut layer_cloned = layer.clone();

            let handle = thread::spawn(move || {

                // Send data to the next layer
                //sender_clone.send((data_to_send, input_t)).unwrap();

                // Receive data from the previous layer
                layer_cloned.process();

                //let received_data = receiver_clone.lock().unwrap().recv().unwrap();
                //println!("Layer {} received data: {:?}", layer.neuron_numer, received_data);
            });

            handles.push(handle);
            //input_data = Vec::new(); // Clear input for the next layer
        }

        self.sender.send((input_v.clone(), input_t));

        for handle in handles{
            handle.join().unwrap();
        }

        // Simulate the final processing or use the received data
        let final_result = self.receiver.lock().unwrap().recv().unwrap();
        println!("Final result: {:?}", final_result);
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