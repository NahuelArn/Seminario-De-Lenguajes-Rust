

/*Ejemplo Trait Concreto y Abstracto */
//
// pub trait MulI32 {
//  fn mul(&self, other:i32) -> f64;// método abstracto
//  fn hace_algo_concreto(&self){// método concreto con implementación por defecto
//  println!("hace_algo_concreto aaa");
//  }
// }
// impl MulI32 for f64 {
//     fn mul(&self, other: i32) -> f64 {
//         self * other as f64
//     }
// }
// pub fn main() {
//     let v1 = 2.8;
//     let v2 = 4;
//     let r = v1.mul(v2);  // uso del método mul del trait MulI32
//     println!("{}", r);
//     v1.hace_algo_concreto();// imprime el resultado de la multiplicacion
// }


//-------------------Ejemplo de Trait Generico-------------------
/* 
//Forzar el polimorfismo, "generico de un abstracto"
struct Perro{}  
struct Gato{}
// fn main(){
//  let gato = Gato{};
//  let perro = Perro{};
//  println!("{} {}", gato.hablar(), perro.hablar());
// }
pub trait Animal{
 fn hablar(&self) -> String;
}
impl Animal for Perro{
 fn hablar(&self) -> String{
 "Guau!".to_string()
 }
}
impl Animal for Gato{
 fn hablar(&self) -> String{
 "Miau!".to_string()
 }
}

fn imprimir_hablar<T: Animal>(animal: &T) {
 println!("Hablo! {}", animal.hablar());
}
pub fn main(){
 let gato = Gato{};
 let perro = Perro{};
 imprimir_hablar(&gato);
 imprimir_hablar(&perro);
}

*/


//-------------------Trait impl como parametro-------------------
//al definir la firma con impl, rust se asegura que el tipo implemente el trait, asegurando la seguridad y evitando errores en tiempo de ejecución.
// pub fn imprimir_hablar(animal1: &impl Animal, animal2: &impl Animal) {
//  println!("Hablando! {} {}", animal1.hablar(), animal2.hablar());
// }
// fn main(){
//  let gato = Gato{};
//  let perro = Perro{};
//  imprimir_hablar(&gato, &perro);
// }


//asi le especifico qu quiero que implemente el trait animal como el trait OtroTrait
// pub fn imprimir_hablar(animal: &(impl Animal + OtroTrait)) {
//     println!("Hablo! {}", animal.hablar());
// }

// fn main() {
//     let gato = Gato{};
//     imprimir_hablar(&gato);
// }

//mismo resultado pero mas generico
// pub fn imprimir_hablar<T>(animal: &T)
// where
//     T: Animal + OtroTrait
// {
//     println!("Hablo! {}", animal.hablar());
// }

// fn main() {
//     let gato = Gato;
//     imprimir_hablar(&gato);
// }