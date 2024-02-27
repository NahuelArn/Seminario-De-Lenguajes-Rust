/* 5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario
ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la
cadena en mayúsculas */

use std::io;
fn main() {
    let mut cadena_definida: String = String::from("pepe");
    println!("Ingrese una cadena de caracteres: ");

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .ok()
        .expect("fallo en la entrada");
    let caracteres: String = match entrada.trim().parse() {
        Ok(caracter) => caracter,
        Err(_) => {
            println!("Error");
            return;
        }
    };

    // let casa: String = caracteres + cadena_definida;
    cadena_definida += &caracteres;
    println!("cadena en minusculas {}", cadena_definida.to_uppercase()); //convierte el String en Mayusculas
    println!("cadena en mayusculas {}", cadena_definida.to_lowercase()); //convierte el String en Minusculas
}
