/* Escribir un programa que defina una variable de tipo flotante con algún valor, y luego
permita al usuario ingresar un número decimal por teclado para multiplicar, dividir, sumar y
restar su valor. Se deben imprimir los resultados.*/

use std::io::stdin;
fn main() {
    let mut valor: f32 = 3.0;
    println!("Ingrese un valor decimal");
    let mut entrada = String::new();
    stdin()
        .read_line(&mut entrada)
        .expect("Error al leer un valor"); //trim elimina los espacios en blanco o nueva linea
    let entrada: f32 = entrada.trim().parse().expect("Ingrese un valor valido"); //parseo el string a un flotante

    //multiplicacion
    let mut resultado: f32 = valor * entrada;
    println!("valor default: {} * {} = {}", valor, entrada, resultado);
    //division
    let mut resultado: f32 = valor / entrada;
    println!("valor default: {} / {} = {}", valor, entrada, resultado);
    //suma
    let mut resultado: f32 = valor + entrada;
    println!("valor default: {} + {} = {}", valor, entrada, resultado);
    //resta
    let mut resultado: f32 = valor - entrada;
    println!("valor default: {} - {} = {}", valor, entrada, resultado);
}
