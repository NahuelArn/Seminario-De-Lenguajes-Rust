
/*
ayudin 
 iter_v.all(closure);
   iter_v.any(closure);
   iter_v.filter(closure);
   iter_v.filter_map(closure);
   iter_v.skip_while(closure)
*/

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
fn existe_la_persona<'a>(vec: &'a Vec<Persona<'a>>,p: Persona)-{

}

pub fn main() {
    // let persona = Persona::new("Pepe", "Pepe", "Pepe", "Pepe", 1000.0, 23);
    // let mut vec:Vec<Persona> = Vec::new();
    // vec.push(persona);
    // let casa = salario_mayor(&vec, 900.0);
    // if casa.len() == 1 {
    // 	println!("Salario mayor a 900.0: {}", casa[0].nombre);
    // }
}







//Primera version tp
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
