/*
Escribir un programa que defina una estructura Estudiante que tenga campos para el
nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
métodos:
❖ Examen:
➢ new: que pasando los parámetros correspondientes, crea un Examen y lo
retorna.
❖ Estudiante:
➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo
retorna.
➢ obtener_promedio: retorna el promedio de las notas.
➢ obtener_calificacion_mas_alta: retorna la nota más alta.
➢ obtener_calificacion_mas_baja: retorna la nota más baja.

*/
struct Examen {
    nombre_de_materia: String,
    nota: f32,
}
//➢ new: que pasando los parámetros correspondientes, crea un Examen y lo retorna.
impl Examen {
    fn new(nombre_de_materia: String, nota: f32) -> Examen {
        Examen {
            nombre_de_materia,
            nota,
        }
    }
}

struct Estudiante {
    nombre: String,
    num_identificacion: u32,
    calificacion_de_examenes: Vec<Examen>,
}

// ➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo retorna.
impl Estudiante {
    fn new(
        nombre: String,
        num_identificacion: u32,
        calificacion_de_examenes: Vec<Examen>,
    ) -> Estudiante {
        Estudiante {
            nombre,
            num_identificacion,
            calificacion_de_examenes,
        }
    }
    // ➢ obtener_promedio: retorna el promedio de las notas.
    fn obtener_promedio(&self) -> f32 {
        let mut suma: f32 = 0.0;
        //obtiene una referencia del vector completo y como esta iterando, obtiene las referencia de cada campo
        for examen in &self.calificacion_de_examenes {
            suma += examen.nota;
        }
        //se abstrae y obtiene una referencia a cada elemento
        // for i in self.calificacion_de_examenes.iter() {
        //     suma += i.nota;
        // }
        let dim_f: usize = self.calificacion_de_examenes.len();
        if dim_f > 0 {
            //aca no existen los operadores ternarios
            suma / dim_f as f32 //parseo a f32 el resultado
        } else {
            0.0
        }
    }
    // ➢ obtener_calificacion_mas_alta: retorna la nota más alta.
    //como no me especifica que las notas del vector estan ordenas
    fn obtener_calificacion_mas_alta(&self) -> f32 {
        let mut max: f32 = f32::MIN;
        for i in &self.calificacion_de_examenes {
            if i.nota > max {
                max = i.nota;
            }
        }
        max
    }
    // ➢ obtener_calificacion_mas_baja: retorna la nota más baja.
    fn obtener_calificacion_mas_baja(&self) -> f32 {
        let mut min: f32 = f32::MAX;
        for i in &self.calificacion_de_examenes {
            if i.nota < min {
                min = i.nota;
            }
        }
        min
    }
}

pub fn main() {}

mod test_calificacion_baja { //faltarian demas test, de las otras funciones as
    use super::*;
    #[test]
    fn prueba1() {
        let examen1 = Examen::new("matematica".to_string(), 7.0);
        let examen2 = Examen::new("matematica".to_string(), 8.0);

        let v: Vec<Examen> = vec![examen1, examen2]; //vector dinamico
        let estudiante: Estudiante = Estudiante::new("manuel".to_string(), 1, v);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), 7.0);
    }

    use super::*;
    #[test]
    fn prueba2() {
        let examen1 = Examen::new("matematica".to_string(), 7.0);
        let examen2 = Examen::new("matematica".to_string(), 8.0);

        let v: Vec<Examen> = vec![examen1, examen2]; //vector dinamico
        let estudiante: Estudiante = Estudiante::new("manuel".to_string(), 1, v);
        assert_ne!(estudiante.obtener_calificacion_mas_baja(), 6.0);
    }
}
