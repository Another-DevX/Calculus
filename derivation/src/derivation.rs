// pub mod derivation {

    pub fn derivate<F>(x: f64, function: F) -> Option<f64>
    where
        F: Fn(f64) -> f64,
    {
        let mut h = 1e-5;

        let mut last_derivative = _derivative(x, h, &function);

        while h > 1e-16 {
            h /= 2.0;
            let current_derivative = _derivative(x, h, &function);

            if (current_derivative - last_derivative).abs() < 1e-5 {
                return Some(current_derivative);
            }

            last_derivative = current_derivative;
        }
        None
    }

    fn _derivative<F>(x: f64, h: f64, f: &F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        (f(x + h) - f(x)) / h
    }
// }
