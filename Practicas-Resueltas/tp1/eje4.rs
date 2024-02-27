/* Escribir un programa que defina una tupla que contenga una cadena, un número entero
con signo y un valor booleano, y luego imprima cada valor de la tupla */

// fn main() {
//     let tupla = ("pepe", -2, true);

//     let (casa, casa1, casa2) = tupla;

//     println!("a: {casa}")
// }
// fn main() {
//     let tup: (String, i32, f64, u8) = (String::from("hola"), 500, 6.4, 1);

//     let (a, b, c, d) = tup;

//     println!("a: {a}");
// }

fn main() {
    let tup: (String, i32, bool) = (String::from("hola"), -400, false);
    let (a, b, c) = tup;
    println!("a: {}, b: {}, c:{}", a, b, c);
    println!("a: {a}, b: {b}, c: {c}");	
}
