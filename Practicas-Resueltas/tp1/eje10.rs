/* Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego
cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos
originales. */

fn main() {
    let arreglo1: [i64; 5] = [1, 2, 3, 4, 5];
    let arreglo2: [i64; 5] = [1, 2, 3, 4, 5];

    let mut arreglo_sumador: [i64; 5] = [0, 0, 0, 0, 0];

    for i in 0..5 {
        arreglo_sumador[i] = arreglo1[i] + arreglo2[i];
    }

    for i in &arreglo_sumador {
        println!("Contendido es: {}", i);	//con esta forma I directamente ya tiene el valor de cada elemento del arreglo, no guarda como tal un numero de indice
    }
    println!();
    for i in 0..5 {
        println!("Contendido en el indice: {} es: {}", i, arreglo_sumador[i]);
    }
}
