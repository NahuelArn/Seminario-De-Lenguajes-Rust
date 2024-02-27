/* Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al
usuario ingresar un número entero por teclado para sumarse con la variable definida. El
programa debe imprimir el valor del número elevado al cuadrado. */

use std::io;
fn main() {
    let mut variable_definida: u64 = 5;

	println!("Ingrese un numero entero");
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .ok()
        .expect("Fallo de linea");
    let numero: u64 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error ingrese un numero valido: ");
            return;
        }
    };

    variable_definida += numero;
    println!("El valor de {} al cuadrado es: {}",variable_definida,variable_definida * variable_definida);
}
