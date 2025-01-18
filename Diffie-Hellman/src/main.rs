pub struct DiffieHellman {
    p: u64,
    g: u64,
}

impl DiffieHellman {
    pub fn new(p: u64, g: u64) -> DiffieHellman {
        DiffieHellman { p, g }
    }

    pub fn private_key(&self, min: u64) -> u64 {
        min + 1
    }

    pub fn public_key(&self, private_key: u64) -> u64 {
        self.modular_pow(self.g, private_key, self.p)
    }

    pub fn secret(&self, public_key: u64, private_key: u64) -> u64 {
        self.modular_pow(public_key, private_key, self.p)
    }

    fn modular_pow(&self, base: u64, exponent: u64, modulus: u64) -> u64 {
        let mut result = 1;
        let mut base = base % modulus;
        let mut exp = exponent;

        while exp > 0 {
            if exp & 1 == 1 {
                result = (result * base) % modulus;
            }
            base = (base * base) % modulus;
            exp >>= 1;
        }
        result
    }
}

fn main() {}