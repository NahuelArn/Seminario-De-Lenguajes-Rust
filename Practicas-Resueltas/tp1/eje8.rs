/* Escribir un programa que defina una constante de tipo cadena, y luego imprima el
número de veces que un caracter específico ingresado por el usuario aparece en la cadena.
Se debe imprimir el resultado. */
use std::io;
const CADENA: &str = "casa";	//si defino directamete String, no lo acepta porq una const no puede ser mutable
fn main() {
    println!("Ingrese un caracter");

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .ok()
        .expect("Erro de linea");

    let caracter: char = match entrada.trim().parse() {
        Ok(caden) => caden,
        Err(_) => {
            println!("Error ingrese un numero valido ");
            return;
        }
    };
    let mut contador: u64 = 0;

    for c in CADENA.chars() {	//recorrer la constante, caracter x caracter
        if c == caracter {
            contador += 1;
        }
    }
    println!("La cantidad que aparece el caracter: {caracter} en la cadena es: {contador}")
}
