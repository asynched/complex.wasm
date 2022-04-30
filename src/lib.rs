use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Complex {
    pub imaginary: f64,
    pub real: f64,
}

fn hypot(real: f64, imaginary: f64) -> f64 {
    let mut a = real.abs();
    let mut b = imaginary.abs();

    if a < 3_000.0 && b < 3_000.0 {
        return (a * a + b * b).sqrt();
    }

    if a < b {
        a = b;
        b = real / imaginary;
    } else {
        b = imaginary / real;
    }

    return a * (1.0 + b * b).sqrt();
}

#[wasm_bindgen]
impl Complex {
    #[wasm_bindgen(constructor)]
    pub fn new(imaginary: f64, real: f64) -> Complex {
        Complex { imaginary, real }
    }

    #[wasm_bindgen]
    pub fn abs(&self) -> f64 {
        hypot(self.real, self.imaginary)
    }

    #[wasm_bindgen]
    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            imaginary: self.imaginary + other.imaginary,
            real: self.real + other.real,
        }
    }

    #[wasm_bindgen]
    pub fn sub(&self, other: &Complex) -> Complex {
        Complex {
            imaginary: self.imaginary - other.imaginary,
            real: self.real - other.real,
        }
    }

    #[wasm_bindgen]
    pub fn mul(&self, other: &Complex) -> Complex {
        if self.imaginary == 0.0 && other.imaginary == 0.0 {
            return Complex {
                imaginary: 0.0,
                real: self.real * other.real,
            };
        }

        Complex {
            imaginary: self.imaginary * other.imaginary - self.real * other.real,
            real: self.imaginary * other.real + self.real * other.imaginary,
        }
    }

    #[wasm_bindgen]
    pub fn sign(&self) -> Complex {
        let abs = self.abs();
        Complex {
            imaginary: self.imaginary / abs,
            real: self.real / abs,
        }
    }
}
