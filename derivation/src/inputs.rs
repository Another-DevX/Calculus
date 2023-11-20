extern crate meval;
// pub mod inputs {
    use meval::Expr;
    use std::{io, str::FromStr as _};

    // Get Point
    pub fn input_point_to_f64() -> f64 {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error at read input");

        let num: f64 = match input.trim().parse() {
            Ok(value) => value,

            Err(_) => {
                println!("Please only enter a valid f64");
                input_point_to_f64()
            }
        };
        num
    }
   // Get Function 
    pub fn input_function() -> Option<Box<dyn Fn(f64) -> f64>> {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error at read input");

        match Expr::from_str(&input.trim()).and_then(|expr| expr.bind("x")) {
            Ok(f) => Some(Box::new(f)),
            Err(e) => {
                println!("Error al parsear la función: {}", e);
                None
            }
        }
    }
// }
