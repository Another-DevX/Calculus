fn main() {
    let x = 2.0; // Punto en el que queremos calcular la derivada
    let mut h = 1e-1; // Comenzamos con un valor de h relativamente grande

    let mut last_derivative = derivative(x, h);

    while h > 1e-10 { // Continuamos reduciendo h hasta que sea muy pequeño
        h /= 2.0;
        let current_derivative = derivative(x, h);

        // Comprobar si la diferencia entre las dos aproximaciones sucesivas es muy pequeña
        if (current_derivative - last_derivative).abs() < 1e-5 {
            println!("La derivada aproximada de f(x) en x = {} es: {}", x, current_derivative);
            break;
        }

        last_derivative = current_derivative;
    }
}

fn f(x: f64) -> f64 {
    // Define aquí tu función, por ejemplo, x^2
    x * x
}

fn derivative(x: f64, h: f64) -> f64 {
    // Diferencia finita hacia adelante
    (f(x + h) - f(x)) / h
}
