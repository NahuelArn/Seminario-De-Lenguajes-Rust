// use std::cmp::PartialEq;
/*
ayudin
 iter_v.all(closure);
   iter_v.any(closure);
   iter_v.filter(closure);
   iter_v.filter_map(closure);
   iter_v.skip_while(closure)
*/

use std::i128::MIN;

trait Mayor {
    fn mayor_salario(&self, num: f64) -> bool;
    fn mayor_edad(&self, num: u8) -> bool;
}
impl<'a> Mayor for Persona<'a> {
    fn mayor_salario(&self, num: f64) -> bool {
        self.salario > num
    }
    fn mayor_edad(&self, num: u8) -> bool {
        self.edad > num
    }
}
#[derive(PartialEq, Debug)]
struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}
impl Persona<'_> {
    fn new<'a>(
        nombre: &'a str,
        apellido: &'a str,
        direccion: &'a str,
        ciudad: &'a str,
        salario: f64,
        edad: u8,
    ) -> Persona<'a> {
        Persona {
            nombre,
            apellido,
            direccion,
            ciudad,
            salario,
            edad,
        }
    }
}
/*
a- Escriba una función que reciba un vector de personas y otro parámetro que indica un
salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.
*/
fn salario_mayor<'a>(vec: &'a Vec<Persona>, salario: f64) -> Vec<&'a Persona<'a>> {
    // let resuelve = |x: &Persona| x.salario > salario;
    let mut retorno: Vec<&Persona> = Vec::new();
    for i in vec.iter() {
        if i.mayor_salario(salario) {
            retorno.push(i);
        }
    }
    retorno
}

/*
 b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
 y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro
 ciudad.
*/

//mas rustico
// fn persona_perfil<'a>(vec:&'a Vec<Persona<'a>>, edad: u8,ciudad: String) -> Vec<& 'a Persona<'a>>{
// 	let resuelve = |x: &Persona| x.edad > edad && x.ciudad == ciudad;
// 	let iter = vec.iter();
// 	iter.filter(|&x| resuelve(x)).collect()
// }
fn persona_perfil<'a>(vec: &'a Vec<Persona<'a>>, edad: u8, ciudad: String) -> Vec<&'a Persona<'a>> {
    // Closure que verifica si una persona cumple con las condiciones
    let resuelve = |x: &Persona| x.edad > edad && x.ciudad == ciudad;

    // Crear un iterador sobre las referencias a las personas en el vector
    let iter = vec.iter();

    // Filtrar el iterador para incluir solo las personas que cumplen con las condiciones
    let filtrado = iter.filter(|&x| resuelve(x));

    // Colectar el resultado filtrado en un vector de referencias
    filtrado.collect()
}

/*
 c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
 retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso
 contrario.
*/
//forma con mas convencional
// fn todos_en_la_ciudad<'a>(vec: &'a Vec<Persona>, ciudad: String) -> bool {
//     let mut estado: bool = true;
//     let mut i: usize = 0;
//     while estado && i < vec.len() {
//         if vec[i].ciudad != ciudad {
//             estado = false;
//         }
//         i += 1;
//     }
//     estado
// }
//forma utilizando iter/clousure bastante resumida
fn tamos_todos_ciudad<'a>(vec: &'a Vec<Persona<'a>>, ciudad: &str) -> bool {
    vec.iter().all(|x| x.ciudad == ciudad)
}

/*
d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
 retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso
 contrario.
*/
//any una vez que encuentra uno que cumple la condicion corta
fn hay_alguien_en_la_ciudad<'a>(vec: &'a Vec<Persona<'a>>, ciudad: &str) -> bool {
    vec.iter().any(|x| x.ciudad == ciudad)
}

/*
e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la
 persona existe en el arreglo, false caso contrario
*/
// fn existe<'a>(vec: &'a Vec<Persona<'a>>, p: Persona) -> bool {
//     vec.contains(&p)
// }
fn existe_la_persona<'a>(vec: &'a Vec<Persona<'a>>, p: &Persona) -> bool {
    vec.iter().any(|x| x == p)
}

/*
f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
edades de las personas.
*/
fn edades_de_la_persona<'a>(vec: &'a Vec<Persona<'a>>) -> Vec<u8> {
    vec.iter().map(|persona| persona.edad).collect()
}

/*
g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
categoría desempatar por la edad más grande.

*/
// una referencia de un vector de referencias a personas que contiene referencias a personas
// fn mayor_menor_salario<'a>(vec: & 'a Vec<&'a Persona<'a>>) {
// una referencia de un vector a personas que contiene referencias a personas

//arranco los el min,  max en el valor inicial de mi vector
fn mayor_menor_salario<'a>(vec: &'a Vec<Persona<'a>>) -> (&'a Persona<'a>, &'a Persona<'a>) {
    let mut max: &Persona = &vec[0];
    let mut min: &Persona = &vec[0];
    let decide = |x: &Persona| (max.salario == x.salario && max.edad > x.edad);
    let resuelve_max = |x: &Persona| x.salario > max.salario || (decide(x));
    let resuelve_min = |x: &Persona| x.salario < min.salario || (decide(x));
    for i in vec.iter() {
        if resuelve_max(&i) {
            max = i;
        }
        if resuelve_min(&i) {
            min = i;
        }
    }
    (max, min)
}

pub fn main() {}

#[test]
fn test_salario_mayor() {
    let p1 = Persona::new("Pepe", "Pepe", "Pepe", "Pepe", 1000.0, 23);
    let p2 = Persona::new("Pepe", "Pepe", "Pepe", "Pepe", 2000.0, 23);
    let p3 = Persona::new("Pepe", "Pepe", "Pepe", "Pepe", 3000.0, 23);
    let vec = vec![p1, p2, p3]; // Change the type of vec from &Vec<&Persona<'_>> to &Vec<Persona<'_>>
    let res = salario_mayor(&vec, 2000.0);
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].salario, 3000.0);
}

#[test]
fn test_persona_perfil() {
    let p1 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 1000.0, 23);
    let p2 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 2000.0, 23);
    let p3 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 3000.0, 23);
    let vec = vec![p1, p2, p3];
    let res = persona_perfil(&vec, 20, "CiudadDePepe".to_string());
    assert_eq!(res.len(), 3);
    let res = persona_perfil(&vec, 20, "Como? ciudad?".to_string());
    assert_ne!(res.len(), 2); //que calidad el assert_ne!
}

#[test]
fn test_todos_en_la_ciudad() {
    let p1 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 1000.0, 23);
    let p2 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 2000.0, 23);
    let p3 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 3000.0, 23);
    let vec = vec![p1, p2, p3];
    assert_eq!(tamos_todos_ciudad(&vec, "CiudadDePepe"), true);
    assert_eq!(tamos_todos_ciudad(&vec, "Como? ciudad?"), false);
}

#[test]
fn test_hay_alguien_en_la_ciudad() {
    let p1 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 1000.0, 23);
    let p2 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 2000.0, 23);
    let p3 = Persona::new("Pepe", "Pepe", "Pepe", "Buenos Aires", 3000.0, 23);
    let vec = vec![p1, p2, p3];
    assert_eq!(hay_alguien_en_la_ciudad(&vec, "CiudadDePepe"), true);
    assert_eq!(hay_alguien_en_la_ciudad(&vec, "Buenos Aires"), true);
    assert_eq!(hay_alguien_en_la_ciudad(&vec, "La"), false);
}

#[test]
fn test_existe_la_persona() {
    let p1 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 1000.0, 23);
    let p2 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 2000.0, 23);
    let p3 = Persona::new("Fernandez", "Macri", "nose1", "Fernandez", 3000.0, 23);
    let vec = vec![p1, p2, p3];
    assert_eq!(existe_la_persona(&vec, &vec[0]), true);
    assert_eq!(existe_la_persona(&vec, &vec[1]), true);
    assert_eq!(existe_la_persona(&vec, &vec[2]), true);
    assert_eq!(
        existe_la_persona(
            &vec,
            &Persona::new("Fernandez", "Macri", "nose1", "Fernandez", 3000.0, 23)
        ),
        true
    );
    assert_eq!(
        existe_la_persona(
            &vec,
            &Persona::new("Macri", "Pepe", "Pepe", "CiudadDePepe", 1000.0, 24)
        ),
        false
    );
}

#[test]
fn test_edades_de_la_persona() {
    let p1 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 1000.0, 22);
    let p2 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 2000.0, 23);
    let p3 = Persona::new("Fernandez", "Macri", "nose1", "Fernandez", 3000.0, 44);
    let vec = vec![p1, p2, p3];
    assert_eq!(edades_de_la_persona(&vec), vec![22, 23, 44]);
}

#[test]
fn test_mayor_menor_salario() {
    let p1 = Persona::new("Pepe1", "Pepe", "Pepe", "CiudadDePepe", 1000.0, 22);
    let p2 = Persona::new("Pepe", "Pepe", "Pepe", "CiudadDePepe", 2000.0, 23);
    let p3 = Persona::new("Fernandez", "Macri", "nose1", "Fernandez", 3000.0, 44);
    let vec = vec![p1, p2, p3];
    let (max, min) = mayor_menor_salario(&vec);
    // print!("{:?}", min);
    // println!("{:?}", max);
    // print!("HOLAAAAAAAAAAAAAAAAAA");
    assert_eq!(max, &vec[2]);
    assert_eq!(min, &vec[0]);
}

//Primera version haciendo el tp
// struct Persona<'a> {
//     nombre: &'a str,
//     apellido: &'a str,
//     direccion: &'a str,
//     ciudad: &'a str,
//     salario: f64,
//     edad: u8,
// }
// impl Persona<'_> {
// 	fn new<'a>(nombre: &'a str, apellido: &'a str, direccion: &'a str, ciudad: &'a str, salario: f64, edad: u8) -> Persona<'a> {
// 		Persona {
// 			nombre,
// 			apellido,
// 			direccion,
// 			ciudad,
// 			salario,
// 			edad,
// 		}
// 	}
// }
// /*
// a- Escriba una función que reciba un vector de personas y otro parámetro que indica un
// salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.
// */
// fn salario_mayor<'a>(vec: &'a Vec<Persona>, salario: f64) -> Vec<&'a Persona<'a>> {
//     let resuelve = |x: &Persona| x.salario > salario;
//     let mut retorno: Vec<&Persona> = Vec::new();
//     for i in vec.iter() {
//         if resuelve(i) {
//             retorno.push(i);
//         }
//     }
//     retorno
// }

// /*
//  b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
//  y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro
//  ciudad.
// */
// fn persona_perfil<'a>(vec:&'a Vec<Persona>, edad: u8,ciudad: String) -> Vec<& 'a Persona<'a>>{
// 	let resuelve = |x: &Persona| x.edad > edad && x.ciudad == ciudad;
// 	let mut retorno: Vec<&Persona> = Vec::new();
// 	for i in vec.iter() {
// 		if resuelve(i) {
// 			retorno.push(i);
// 		}
// 	}
// 	retorno
// }

// /*
//  c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
//  retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso
//  contrario.
// */
// fn todos_en_la_ciudad<'a>(vec: &'a Vec<Persona>, ciudad: String)-> bool{
// 	let mut estado:bool = true;
// 	let mut i :usize= 0;
// 	while estado && i < vec.len(){
// 		if vec[i].ciudad != ciudad{
// 			estado = false;
// 		}
// 		i += 1;
// 	}
// 	estado
// }
// pub fn main() {
//     // let persona = Persona::new("Pepe", "Pepe", "Pepe", "Pepe", 1000.0, 23);
//     // let mut vec:Vec<Persona> = Vec::new();
// 	// vec.push(persona);
//     // let casa = salario_mayor(&vec, 900.0);
// 	// if casa.len() == 1 {
// 	// 	println!("Salario mayor a 900.0: {}", casa[0].nombre);
// 	// }
// }
