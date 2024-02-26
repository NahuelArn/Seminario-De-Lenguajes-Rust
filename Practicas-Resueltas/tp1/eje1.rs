/* 1- Escribir un programa que defina una variable de tipo flotante con algún valor, y luego
permita al usuario ingresar un número decimal por teclado para multiplicar, dividir, sumar y
restar su valor. Se deben imprimir los resultados */
use std::io;

fn main() {
    let mut variable_Flotante: f64 = 3.14;

    println!("Ingrese un numero: ");

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .ok()
        .expect("Fallo de linea");
    let numero: i32 = match entrada.trim().parse() {
        //trim elimina los espacio del princio/final de la entrada del usuario
        Ok(num) => num,
        Err(_) => {
            println!("Error ingrese un numero valido: ");
            return;
        }
    };

    let mut multiplicacion: f64 = numero * variable_Flotante;
    println!("Resultado de {}la multiplicacion: ", multiplicacion);
}
