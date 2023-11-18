# Cálculo de la Derivada en Rust

Este documento describe el método utilizado en un programa Rust para calcular la derivada de una función en un punto específico.

## Descripción del Código

El código en Rust utiliza dos módulos principales:

- `derivation_mod`: Encargado de realizar la derivación numérica.
- `inputs_mod`: Maneja las entradas del usuario para obtener el punto y la función a derivar.

### Método de Derivación Numérica

La derivada de una función en un punto se aproxima utilizando el cociente de diferencias:

```rust
fn _derivative<F>(x: f64, h: f64, f: &F) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x)) / h
}
```

Aquí, `h` es un pequeño incremento en `x`, y `f` es la función de la cual queremos encontrar la derivada.

## Definición Matemática

La derivada de una función en un punto es el límite del cociente de diferencias cuando `h` tiende a cero:

$$ f'(x) = \lim_{h \to 0} \frac{f(x + h) - f(x)}{h} $$

Este límite describe la tasa de cambio instantánea de la función `f` en el punto `x`.

### Aproximación Numérica

En el código, la derivada se aproxima mediante la reducción gradual de `h` y la evaluación del cociente de diferencias:

```rust
while h > 1e-10 {
    h /= 2.0;
    let current_derivative = _derivative(x, h, &function);

    if (current_derivative - last_derivative).abs() < 1e-5 {
        return Some(current_derivative);
    }

    last_derivative = current_derivative;
}
```

Esta aproximación nos permite calcular un valor cercano al verdadero límite sin la necesidad de alcanzar un `h` exactamente igual a cero, lo cual es impracticable en cálculos numéricos.

## Consideraciones

- La precisión de esta aproximación depende del tamaño inicial de `h` y del criterio de convergencia.
- Métodos más sofisticados pueden proporcionar mejores aproximaciones, especialmente en puntos donde la función tiene un comportamiento complejo.
