/* Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario
ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena
ingresada por el usuario se encuentra en el arreglo.  */

use std::io;

fn main() {
    let array1: [&str; 5] = [
        String::from(""),
        String::from("pepe"),
        String::from(""),
        String::from("", String::from()),
    ];

    println!("Ingresar una cadena para ver si se encuentra en el arreglo");

    let mut cadena = String::new();
    stdin().read_line(&mut cadena).expect("error en la entrada");

    for i in &array1 {
        if (i = cadena) {
            println!("Se encontraba en el array")
        }
    }
}
