/* 1- Escribir un programa que defina una variable de tipo flotante con algún valor, y luego
permita al usuario ingresar un número decimal por teclado para multiplicar, dividir, sumar y
restar su valor. Se deben imprimir los resultados */
use std::io;

fn main() {
    let mut variable_flotante: f64 = 3.14; //Convencion snake case

    println!("Ingrese un numero: ");

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .ok()
        .expect("Fallo de linea");
    let numero: f64 = match entrada.trim().parse() {
        //trim elimina los espacio del princio/final de la entrada del usuario
        Ok(num) => num, 
        Err(_) => {
            println!("Error ingrese un numero valido: ");
            return;
        }
    };
    
    //Multiplicacion
    let mut multiplicacion: f64 = numero * variable_flotante;
    println!("Resultado de la multiplicacion: {}", multiplicacion); //{} me sirven para ubicar el valor a mostrar

    //Division
    let mut division: f64 = numero / variable_flotante;
    println!("Resultado de la division: {}", division);

    let mut suma: f64 = numero + variable_flotante;
    println!("Resultado de la suma: {}", suma);

    let mut resta: f64 = numero - variable_flotante;
    println!("Resultado de la resta: {}", resta);
}
