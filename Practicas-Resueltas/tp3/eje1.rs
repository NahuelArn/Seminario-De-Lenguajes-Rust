/*1- Escribir un programa que defina una estructura Persona que tenga campos para el
nombre, la edad y la dirección(que puede ser nulo al momento de la creación de una
persona). Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Persona y la retorna.
➢ to_string: que retorna un string con los datos de la persona concatenados sobre el
mensaje ejecutado por ej:
person.to_string() , donde person es una variable del tipo Persona.
➢ obtener_edad: retorna la edad de la persona.
➢ actualizar_direccion(nueva_direccion)
 */

struct Persona {
    nombre: String,
    edad: u32,
    direccion: Option<String>,
}

impl Persona {
    fn new(nombre: String, edad: u32, direccion: Option<String>) -> Persona {
        //no es necesario guardarlo en una variable pero para hacerlo mas legible
        let persona = Persona {
            nombre,
            edad,
            direccion,
        };
        persona
    }

    fn to_string(&self) -> String {
        let mut result: String = format!("Nombre: {}, Edad: {}", self.nombre, self.edad);
        match &self.direccion {
            Some(a) => result += &format!(", Su direccion es {}", a),
            None => result += ", Sin direccion cargada",
        }
        result
    }
    fn obtener_edad(&self) -> u32 {
        return self.edad;
    }

    //fn sin sentindo xD asjdjasdja
    // fn actualizar_direccion(&mut self, nueva_direccion: Option<String>) {
    //     match &self.direccion {
    //         Some(a) => self.direccion = nueva_direccion,
    //         None => print!("No tiene ninguna direccion cargada, asi que no puedo actualizarla?"),
    //     }
    // }
    fn actualizar_direccion(&mut self, nueva_direccion: Option<String>) {
        self.direccion = nueva_direccion;
    }
}

pub fn main() {
    let alumno = Persona::new("Juan".to_string(), 15, Some("161".to_string()));

    println!("{}", alumno.to_string());

    let mut alumno2 = Persona::new("Juan".to_string(), 15, None);

    println!("{}", alumno2.to_string());

    println!("{}", alumno2.obtener_edad());

    alumno2.actualizar_direccion(Some("1333".to_string()));
}

//test no es necesario
