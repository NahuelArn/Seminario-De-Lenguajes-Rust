/* 3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario
ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones
and y or. Se deben imprimir ambos resultados. */

use std::io::stdin;

fn main() {
    let x: bool = false;

    println!("Ingrese un operacion true o false");
    let mut entrada = String::new();
    stdin().read_line(&mut entrada).expect("error");

    let entrada: bool = entrada.trim().parse().expect("error");

    println!("valor booleano default {}", x);
    let resultado_and = x && entrada;
    println!("valor default: {} AND {} = {}", x, entrada, resultado_and);

    let resultado_or = x || entrada;
    println!("valor default: {} OR {} = {}", x, entrada, resultado_or);
}
