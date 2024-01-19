use crate::error::{ErrorRes, Componente};
//use crate::error::Tipo;
use rand::Rng;
use std::fmt;


#[derive(Clone)]
/*
    Struttura che contine tutte le informazioni necessarie per i calcoli:
    - index è l'indice del Neurone all'interno del Layer a cui appartiene
    - layer_index è l'indice del Layer, di cui il Neurone fa parte, all'interno della Rete
    - v_soglia indica il valore di soglia oltre il quale il Neurone si attiva mandando uno spike in uscita
    - v_riposo indica il valore a cui tende il potenziale del Neurone quando non riceve spike in ingresso
    - v_reset è il valore a cui viene riportato il potenziale del vettore dopo aver superato la soglia e aver emesso uno spike in uscita
    - v_memorizzato contiene il valore del potenziale del Neurone ad ogni istante di tempo
    - tau serve a definire la velocità con cui il potenziale del Neurone decade al passare del tempo
    - t_prec memorizza l'ultimo istante temporare in cui è stato ricevuto uno spike per poter calcolare il valore di decadimento del potenziale del Neurone stesso
    - layer_prec_dim contiene la dimensione del layer precednete è indica la dimensione del vettore degli 'extra_weights'
    - layer_actual_dim contiene la dimensione del layer attuale è indica la dimensione del vettore degli 'intra_weights'
    - intra_weights contiene il vettore dei pesi degli archi provenienti dagli altri Neuroni dello stesso Layer. All'indice del vettore uguale al campo 'index' il valore, nonostante venga
        ignorato dei calcoli successivi, viene posto a 0 per correttezza
    - extra_weights contiene il vettore dei pesi da tutti i neuroni del Layer precedente al Neurone attuale
 */
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
    /*
        In fase di creazine del Neurone assegnamo dei valori arbitrari alle soglie. Questi valori potranno essere modificati in futuro
     */
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

    // Funzione di supporto per settare il potenziale di soglia del Neurone
    pub fn set_v_soglia(&mut self, val: f64) {
        self.v_soglia = val;
    }

    // Funzione di supporto per settare il valore del potenziale di riposo del Neurone
    pub fn set_v_riposo(&mut self, val: f64) {
        self.v_riposo = val;
    }

    // Funzione di supporto per settare il valore del potenziale di reset del Neurone
    pub fn set_v_reset(&mut self, val: f64) {
        self.v_reset = val;
    }

    // Funzione di supporto per settare il valore di tau del Neurone
    pub fn set_tau(&mut self, val: f64) {
        self.tau = val;
    }

    /*
        Funzione per inizializzare i pesi del Neurone in modo casuale ma rimanendo all'interno di un intervallo definito dall'utente.
        I pesi interni rimangono all'interno dell'iintervallo definito dall'utente ma vengono posti negativi.
     */
    pub fn init_weights_random(&mut self, range: (f64, f64)){
        let mut rng = rand::thread_rng();
        for i in 0..self.layer_actual_dim{
            if self.index == i as usize {
                self.intra_weights.push(0.0);
            }
            else {
                self.intra_weights.push(-rng.gen_range(range.0..=range.1));
            }
        }
        for _ in 0..self.layer_prec_dim{
            self.extra_weights.push(rng.gen_range(range.0..=range.1));
        }
    }

    /*
        Funzione di supporto per la definizione manuale dei pesi. Richiede in ingresso i due vettori associati ali pesi
     */
    pub fn init_weights_defined(&mut self, intra_weights: Vec<f64>, extra_weights: Vec<f64>){
        self.intra_weights = intra_weights;
        self.extra_weights = extra_weights;
    }

    /*
        Funzione principale del Neurone, serve ad elaborare gli spike in ingresso, ricevuti come vettore di u8, ad un certo istante temporale.
     */
    pub fn process(&mut self, spikes_extra: Vec<u8>, spikes_intra:Vec<u8>, time: i32, error_res: ErrorRes) -> u8{
        let mut ret_val= 0;
        let mut summation: f64 = 0.0;
        // Questo primo controllo serve a discriminare sel il neurone su cui sto lavorando sia quello affetto dall'errore o meno.
        if error_res.neuron_id == self.index && error_res.layer_id == self.layer_index {
            // Nel caso in cui ci troviamo nel primo Layer allora il vettore di extra weights sarà vuoto e il vettore di spike in ingresso
            // verrà usato per settare il valore iniziale del Neurone
            if self.layer_index != 0 {
                // Per ogni Neurone del Layer precedente moltiplico il valore dello spike per il peso corrispondente e incremento la sommatoria delgi ingressi di consseguenza
                for (index, &e) in spikes_extra.iter().enumerate() {
                    // Prima controllo se l'errore sia sui PesiE e di conseguenza modifico o meno il valore del peso specificato
                    if error_res.componente == Componente::PesiE && index == error_res.weight_id as usize {
                        let w = error_res.apply_error(*self.extra_weights.get(index).unwrap(), time);
                        let p = error_res.mul(e as f64, w, time);
                        summation = error_res.add(summation, p, time);
                    } else {
                        let w = *self.extra_weights.get(index).unwrap();
                        let p = error_res.mul(e as f64, w, time);
                        summation = error_res.add(summation, p, time);
                    }
                }
            } else {
                self.v_memorizzato = *spikes_extra.get(self.index).unwrap() as f64;
            }
            // Calcolo l'effetto che gli spike al tempo precedente hanno sulla sommatoria del potenziale
            for (index, &e) in spikes_intra.iter().enumerate() {
                if index != self.index {
                    if error_res.componente == Componente::PesiI && index == error_res.weight_id as usize {
                        let w = error_res.apply_error(*self.intra_weights.get(index).unwrap(), time);
                        let p = error_res.mul(e as f64, w, time);
                        summation = error_res.add(summation, p, time);
                    } else {
                        let w = *self.intra_weights.get(index).unwrap();
                        let p = error_res.mul(e as f64, w, time);
                        summation = error_res.add(summation, p, time);
                    }
                }
            }

            // Modifico il valore del componente affetto dall'errore
            let new_v_mem = if error_res.componente == Componente::Memorizzato { error_res.apply_error(self.v_memorizzato, time) } else { self.v_memorizzato };
            let new_v_th = if error_res.componente == Componente::Soglia { error_res.apply_error(self.v_soglia, time) } else { self.v_soglia };
            let new_v_reset = if error_res.componente == Componente::Reset { error_res.apply_error(self.v_reset, time) } else { self.v_reset };
            let new_v_rest = if error_res.componente == Componente::Riposo { error_res.apply_error(self.v_riposo, time) } else { self.v_riposo };

            // v_mem(ts) = v_rest + [v_mem(ts-1) - v_rest] * e^-((ts-(ts-1))/tau)

            // Calcolo il nuovo valore di potenziale del neurone
            let s = error_res.add(new_v_mem, -(new_v_rest), time);
            let p = error_res.mul(s, (-(time - self.t_prec) as f64 / (self.tau)).exp(), time);
            let y = error_res.add(p, summation, time);
            self.v_memorizzato = error_res.add(y, new_v_rest, time);
            // Aggiorno l'ultimo istante temporale al quale ho ricevuto un ingresso con il tempo attuale
            self.t_prec = time;
            // Se il potenziale memorizzato supera la soglia allora genero uno spike in uscita e aggiorno il potenziale memorizzato al valore di reset
            if error_res.greater_than(self.v_memorizzato, new_v_th, time) {
                ret_val = 1;
                self.v_memorizzato = new_v_reset;
            }
        }
        // Questo ramo tratta tutti i neuroni restanti non affetti da errore. Il procedimetno è lo stesso
        else {
            if self.layer_index != 0 {
                for (index, &e) in spikes_extra.iter().enumerate() {
                    let w = *self.extra_weights.get(index).unwrap();
                    let p = error_res.mul(e as f64, w, time);
                    summation = error_res.add(summation, p, time);
                }
            } else {
                self.v_memorizzato = *spikes_extra.get(self.index).unwrap() as f64;
            }
            for (index, &e) in spikes_intra.iter().enumerate() {
                if index != self.index {
                    let w = *self.intra_weights.get(index).unwrap();
                    let p = error_res.mul(e as f64, w, time);
                    summation = error_res.add(summation, p, time);
                }
            }

            // v_mem(ts) = v_rest + [v_mem(ts-1) - v_rest] * e^-((ts-(ts-1))/tau)
            let s = error_res.add(self.v_memorizzato, -(self.v_riposo), time);
            let p = error_res.mul(s, (-(time - self.t_prec) as f64 / (self.tau)).exp(), time);
            let y = error_res.add(p, summation, time);
            self.v_memorizzato = error_res.add(y, self.v_riposo, time);
            self.t_prec = time;
            if error_res.greater_than(self.v_memorizzato, self.v_soglia, time) {
                ret_val = 1;
                self.v_memorizzato = self.v_reset;
            }
        }
        ret_val
    }
}

impl fmt::Display for Neuron{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Neuron number {}:\n\t\tValues: [v_th:{}, v_rest:{}, v_reset:{}, v_mem:{}, tau:{}]\n\t\tIntra weights: {:?}\n\t\tExtra weights: {:?}\n", self.index, self.v_soglia, self.v_riposo, self.v_reset, self.v_memorizzato, self.tau, self.intra_weights, self.extra_weights)
    }
}