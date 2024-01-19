use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
/*
    Definisce il tipo di componente che possono guasarsi.
 */
pub enum Componente{
    None,
    Soglia,
    Riposo,
    Reset,
    Memorizzato,
    PesiI,
    PesiE,
    Sommatore,
    Moltiplicatore,
    Comparatore,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/*
    Defisisce il tipo di errore che può presentarsi su un certo componente
 */
pub enum Tipo{
    None,
    StuckAt0,
    StuckAt1,
    Flip,
}

#[derive(Debug, Clone, Copy)]
/*
    Struttura dell'errore che contiene tutte le informazioni necessarie per il calcolo della resilienza.
    - neuron_id è l'indice del Neurone all'interno di un dato Layer
    - layer_id indica l'indice del Layer all'interno della Rete
    - componente indica il tipo di conponente affetto dell'errore specificato
    - tipo indica il tipo di errore che afflige il dato componente
    - weight_id serve ad indicare l'indice del peso affetto, solo nel caso in cui 'componente' sia PesiI o PesiE.
        Nel caso in cui il 'componente' sia altro allora il valore di questa variabile viene ignorato
    - bit_number inidica il bit affetto dall'errore all0interno del componente
    - time indica l'istante temporale al quale applicare l'errore Flip, altrimenti il valore viene ignorato
*/
pub struct ErrorRes{
    pub neuron_id: usize,
    pub layer_id: usize,
    pub componente: Componente,
    pub tipo: Tipo,
    pub weight_id: i32,
    pub bit_number: i32,
    pub time: i32,
}

impl ErrorRes{
    pub fn new(neuron_id: usize, layer_id: usize, componente: Componente, tipo: Tipo, weight_id: i32, bit_number: i32, time: i32) -> Self {
        ErrorRes {
            neuron_id,
            layer_id,
            componente,
            tipo,
            weight_id,
            bit_number,
            time,
        }
    }

    /*
        La rappresentazione interna dei f64 in Rust è IEEE-754.
        Il MSB è di segno poi ci sono 11 bits di esponente e poi 52 bits di mantissa e il valore finale viene calcolato come (-1)^S * 2^E * M.
        Modificando quindi i bits alle posizioni più significative sarà quindi più probabile ottenere dei cambiamenti nel risultato finale e quindi ottenere
        dei valori di resilenza più bassi.
     */
    pub fn apply_error(&self, value: f64, time: i32) -> f64{
        let mut val: u64 = value.to_bits();     // Trasforma il valore f64 dell'ingresso in una sequenza di bit, in Russt rappresentata da un u64
        match self.tipo {
            Tipo::StuckAt1 => {
                let mask = 1 << self.bit_number;
                val |= mask;                    // OR con una maschera di tutti 0 e 1 alla posizione bit_number così l'unico bit a essere forzato a 1 sarà quello specificato
            },
            Tipo::StuckAt0 => {
                let mask = !(1 << self.bit_number);
                val &= mask;                    // AND con una maschera di tutti 1 e 0 alla posizione bit_number così l'unico bit a essere forzato a 0 sarà quello indicato
            },
            Tipo::Flip => {
                if time == self.time {
                    val ^= 1 << self.bit_number // XOR con una maschera di tutti 0 e 1 alla posizione bit_number così l'unico bit a essere invertito sarà quello indicato
                }
            }
            Tipo::None => {
                panic!("IMPOSSIBILE, NO ERRORE QUI!")
            }
        }
        f64::from_bits(val)                 // Riporta il valore dalla sequenza di bit ad un f64
    }

    /*
        Funzione che rimpiazza la somma di due valori.
        Se l'errore definito riguarda il Sommatore allora applichiamo il tipo di errore specificato ad uno dei due input in modo randomico, ma sempre alla posizione
        definita da 'bit_number'.
        Altrimenti viene eseguita la somma corretta tra i due valori in ingresso.
        Il campo time serve solo per essere propagato alla 'apply_error' per poter valutare correttamente l'errore di tipo Flip.
     */
    pub fn add(self, add1: f64, add2: f64, time: i32) -> f64 {
        let mut result = 0.0;
        if self.componente == Componente::Sommatore {
            let rng = rand::thread_rng().gen_range(0..2);
            if rng == 0 {
                result = self.apply_error(add1, time) + add2;
            }
            else { result = add1 + self.apply_error(add2, time);}
        }
        else { result = add1 + add2 }
        result
    }

    /*
        Funzione che rimpiazza il prodotto di due valori.
        Se l'errore definito riguarda il Moltiplicatore allora applichiamo il tipo di errore specificato ad uno dei due input in modo randomico, ma sempre alla posizione
        definita da 'bit_number'.
        Altrimenti viene eseguito il prodotto corretto tra i due valori in ingresso.
        Il campo time serve solo per essere propagato alla 'apply_error' per poter valutare correttamente l'errore di tipo Flip.
     */
    pub fn mul(self, mul1: f64, mul2: f64, time: i32) -> f64 {
        let mut result = 0.0;
        if self.componente == Componente::Moltiplicatore {
            let rng = rand::thread_rng().gen_range(0..2);
            if rng == 0 {
                result = self.apply_error(mul1, time) * mul2;
            }
            else { result = mul1 * self.apply_error(mul2, time);}
        }
        else { result = mul1 * mul2 }
        result
    }

    /*
        Funzione che rimpiazza il confronto tra due valori.
        Se l'errore definito riguarda il Comparatore allora applichiamo il tipo di errore specificato ad uno dei due input in modo randomico, ma sempre alla posizione
        definita da 'bit_number'.
        Altrimenti viene eseguito il confronto corretto tra i due valori in ingresso.
        Il campo time serve solo per essere propagato alla 'apply_error' per poter valutare correttamente l'errore di tipo Flip.
        Ci concentriamo unicamente sul confronto di maggiore uguale perché abbiamo fatto si di usare unicamente questo tipo di confronto.
     */
    pub fn greater_than(self, com1: f64, com2: f64, time: i32) -> bool {
        let mut result = true;
        if self.componente == Componente::Comparatore {
            let rng = rand::thread_rng().gen_range(0..2);
            if rng == 0 {
                result = self.apply_error(com1, time) >= com2 ;
            }
            else { result = com1 >= self.apply_error(com2, time);}
        }
        else { result = com1 >= com2 }
        result
    }
}

/*
    Questa funzione calcola la differenza tra le due matrici in ingresso alla funzione.
    Nel nostro caso specifico le matrici rappresentano l'uscita, sotto forma di vettore di vettori [Input_numer x Network_output_dim],
    della propagazione degli input attraverso la rete. 'correct' indica l'output ottenuto dalla rete senza errori, 'recived' indica l'output
    della rete in cui è stato introdotto un errore.
*/
pub fn error_computation(correct: Vec<Vec<u8>>, recived: Vec<Vec<u8>>) -> i32{
    let mut v = 0;
    let mut tmp: bool = true;
    for (c, w) in correct.iter().zip(recived.iter()){
        tmp = true;
        for (cc, ww) in c.iter().zip(w.iter()){
            if cc != ww { tmp = false };
        }
        if tmp { v += 1 }
    }
    v
}

impl fmt::Display for ErrorRes{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "On neuron {} of layer {}:\n\tType: {:?}, Component affected: {:?}, Bit affected: {}", self.neuron_id, self.layer_id, self.tipo, self.componente, self.bit_number)?;
        if self.tipo == Tipo::Flip { write!(f, ", At time: {}", self.time)?; }
        if self.componente == Componente::PesiI || self.componente == Componente::PesiE { write!(f, ", Weight index: {}", self.weight_id)?; }
        Ok(())
    }
}