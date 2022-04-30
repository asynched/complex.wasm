use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Complex {
    pub imaginary: f64,
    pub real: f64,
}

fn hypotenuse(real: f64, imaginary: f64) -> f64 {
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
    pub fn new(real: f64, imaginary: f64) -> Complex {
        Complex { imaginary, real }
    }

    #[wasm_bindgen]
    pub fn abs(&self) -> f64 {
        hypotenuse(self.real, self.imaginary)
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
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }

    #[wasm_bindgen]
    pub fn div(&self, other: &Complex) -> Complex {
        let (a, b) = (self.real, self.imaginary);
        let (c, d) = (other.real, other.imaginary);

        if d == 0.0 {
            return Complex {
                imaginary: b / c,
                real: a / c,
            };
        }

        if c.abs() < d.abs() {
            let x = c / d;
            let t = c * x + d;

            return Complex {
                imaginary: (b * x + a) / t,
                real: (b * x - a) / t,
            };
        }

        let x = d / c;
        let t = d * x + c;

        return Complex {
            imaginary: (b * x - a) / t,
            real: (b * x + a) / t,
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantiation() {
        let c = Complex::new(1.0, 2.0);
        assert_eq!(c.real, 1.0);
        assert_eq!(c.imaginary, 2.0);
    }

    #[test]
    fn test_add() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a.add(&b);

        let expected = Complex::new(4.0, 6.0);
        assert_eq!(c.real, expected.real);
        assert_eq!(c.imaginary, expected.imaginary);
    }

    #[test]
    fn test_sub() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a.sub(&b);

        let expected = Complex::new(-2.0, -2.0);
        assert_eq!(c.real, expected.real);
        assert_eq!(c.imaginary, expected.imaginary);
    }

    #[test]
    fn test_mul() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a.mul(&b);

        let expected = Complex::new(-5.0, 10.0);
        assert_eq!(c.real, expected.real);
        assert_eq!(c.imaginary, expected.imaginary);
    }
}
