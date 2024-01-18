use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
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
pub enum Tipo{
    None,
    StuckAt0,
    StuckAt1,
    Flip,
}
#[derive(Debug, Clone, Copy)]
pub struct ErrorRes{
    pub neuron_id: usize,
    pub layer_id: usize,
    pub componenti: Componente,
    pub tipo: Tipo,
    pub weight_id: i32,
    pub bit_number: i32,
    pub time: i32,
}

impl ErrorRes{
    pub fn new(neuron_id: usize, layer_id: usize, componenti: Componente, tipo: Tipo, weight_id: i32, bit_number: i32, time: i32) -> Self {
        ErrorRes {
            neuron_id,
            layer_id,
            componenti,
            tipo,
            weight_id,
            bit_number,
            time,
        }
    }

    pub fn apply_error(&self, value: f64, time: i32) -> f64{
        let mut val: u64 = value.to_bits();
        match self.tipo {
            Tipo::StuckAt1 => {
                let mask = 1 << self.bit_number;
                val |= mask; // stuck at 1
            },
            Tipo::StuckAt0 => {
                let mask = !(1 << self.bit_number);
                val &= mask;// stuck at 0
            },
            Tipo::Flip => {
                if time == self.time {
                    val ^= 1 << self.bit_number // Esegue un XOR per invertire il bit
                }
            }
            Tipo::None => {
                panic!("IMPOSSIBILE, NO ERRORE QUI!")
            }
        }

        //println!("\t\t\tIn appply_error: before = {}, after = {}", value, f64::from_bits(val));
        f64::from_bits(val)
    }

    pub fn add(self, add1: f64, add2: f64, time: i32) -> f64 {
        let mut result = 0.0;
        if self.componenti == Componente::Sommatore {
            let rng = rand::thread_rng().gen_range(0..2);
            if rng == 0 {
                result = self.apply_error(add1, time) + add2;
            }
            else { result = add1 + self.apply_error(add2, time);}
        }
        else { result = add1 + add2 }
        result
    }

    pub fn mul(self, mul1: f64, mul2: f64, time: i32) -> f64 {
        let mut result = 0.0;
        if self.componenti == Componente::Moltiplicatore {
            let rng = rand::thread_rng().gen_range(0..2);
            if rng == 0 {
                result = self.apply_error(mul1, time) * mul2;
            }
            else { result = mul1 * self.apply_error(mul2, time);}
        }
        else { result = mul1 * mul2 }
        result
    }

    pub fn greater_than(self, com1: f64, com2: f64, time: i32) -> bool {
        let mut result = true;
        if self.componenti == Componente::Comparatore {
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

impl fmt::Display for ErrorRes{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "On neuron {} of layer {}:\n\tType: {:?}, Component affected: {:?}, Bit affected: {}", self.neuron_id, self.layer_id, self.tipo, self.componenti, self.bit_number)?;
        if self.tipo == Tipo::Flip { write!(f, ", At time: {}", self.time)?; }
        if self.componenti == Componente::PesiI || self.componenti == Componente::PesiE { write!(f, ", Weight index: {}", self.weight_id)?; }
        Ok(())
    }
}