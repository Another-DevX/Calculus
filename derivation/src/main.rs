mod derivation_mod;
mod inputs_mod;
use inputs_mod::inputs::{input_function, input_point_to_f64};
use derivation_mod::derivation::derivate;

fn main() {
    println!("Por favor ingrese el punto en el que desea derivar");
    let point: f64 = input_point_to_f64();
    println!("Por favor ingrese la función que desea derivar");
    let function = input_function();
    match (point, function) {
        (point, Some(function)) => {
            let image = derivate(point, function);
            match image {
                Some(x) => println!("La derivada aproximada es = {:?} ", x),
                None => println!("No se pudo calcular la derivada"),
            }
        }
        _ => println!("Error al parsear la función"),
    }
}
