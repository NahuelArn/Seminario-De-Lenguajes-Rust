/* 3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario
ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones
and y or. Se deben imprimir ambos resultados */
use std::io;

fn main() {
    let mut logico: bool = true;

    println!("Ingrese un true o false ");

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .ok()
        .expect("Error en la entrada");

    let valor_ingresado: bool = match entrada.trim().parse() {
        Ok(valor) => valor, //valor: es una variable local q nace y muere con Ok para ver si la entrada es correcta, despues solo usamos valor_ingresado
        Err(_) => {
            println!("Error: Por favor ingrese 'true' o 'false' en minuscula.");
            return;
        }
    };

    let resultado: bool = logico && valor_ingresado;
    println!("Valor logico con un AND: {}", logico);
    println!("Valor logico entre: {} AND {} es: {}", logico, valor_ingresado, resultado);

    
    let resultado2: bool = logico || valor_ingresado;
    println!("Valog logico entre {} OR {} es: {}",logico,valor_ingresado,resultado2)
}
