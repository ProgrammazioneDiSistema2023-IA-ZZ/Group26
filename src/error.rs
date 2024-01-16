#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Componente{
    none,
    Soglia,
    Riposo,
    Reset,
    Memorizzato,
    Pesi_i,
    Pesi_e,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tipo{
    none,
    stuckat0,
    stuckat1,
    flip,
}
#[derive(Debug, Clone, Copy)]
pub struct Error_res{
    pub neuron_id: usize,
    pub layer_id: usize,
    pub componenti: Componente,
    pub tipo: Tipo,
    pub weight_id: i32,
    pub bit_number: i32,
}

impl Error_res{
    pub fn new(neuron_id: usize, layer_id: usize, componenti: Componente, tipo: Tipo, weight_id: i32, bit_number: i32) -> Self {
        Error_res {
            neuron_id,
            layer_id,
            componenti,
            tipo,
            weight_id,
            bit_number,
        }
    }

    pub fn apply_error(&self, value: f64) -> f64{
        let mut val: u64 = value.to_bits();
        match self.tipo {
            Tipo::stuckat1 => {
                let mask = 1 << self.bit_number;
                val |= mask; // stuck at 1
            },
            Tipo::stuckat0 => {
                let mask = !(1 << self.bit_number);
                val &= mask;// stuck at 0
            },
            Tipo::flip => {
                val ^= 1 << self.bit_number // Esegue un XOR per invertire il bit
            }
            Tipo::none => {
                panic!("impossible, NoError here!")
            }
        }

        println!("\t\t\tIn appply_error: before = {}, after = {}", value, f64::from_bits(val));
        f64::from_bits(val)
    }

}