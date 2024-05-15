/* 9.-Dada una cadena de veterinarias se desea implementar un sistema de atención de
pacientes para cada veterinaria, de la veterinaria se conoce el nombre, la dirección y un id.

Para la atención de mascotas se requiere administrar una cola de atención. De la mascota
se conoce el nombre, la edad, el tipo de animal(perro, gato, caballo, otros) y su dueño.

Del dueño se conoce el nombre, la dirección y un teléfono de contacto. Luego de la atención se
desea tener un registro de las atenciones realizadas guardando los datos de la mascota, el
diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere.

Dado todo lo mencionado anteriormente implemente los métodos para realizar las
siguientes acciones:
➔ crear una veterinaria.
➔ agregar una nueva mascota a la cola de atención de la veterinaria.
➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente
en atender porque tiene la máxima prioridad.
➔ atender la próxima mascota de la cola. x
➔ eliminar una mascota específica de la cola de atención dado que se retira. x
➔ registrar una atención.  x
➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el
teléfono. x
➔ modificar el diagnóstico de una determinada atención. x
➔ modificar la fecha de la próxima visita de una determinada atención. x
➔ eliminar una determinada atención.
Nota: para la fecha utilice lo implementado en el punto 3 */

// mod eje3;	//no me funciono el importar...funciones del eje3

use chrono::prelude::*; //esto es para sacar la fecha del OS
use std::collections::VecDeque;
#[derive(Debug)]
struct Persona {
    nombre: String,
    direccion: String,
    telefono: u32,
}
impl Persona {
    fn new(nombre: String, direccion: String, telefono: u32) -> Persona {
        Persona {
            nombre,
            direccion,
            telefono,
        }
    }
}
impl PartialEq for Persona {
    fn eq(&self, other: &Self) -> bool {
        self.nombre == other.nombre
            && self.direccion == other.direccion
            && self.telefono == other.telefono
    }
}
#[derive(Debug)]
enum Tipo_Animal {
    Perro,
    Gato,
    Caballo,
    Otros,
}
impl PartialEq for Tipo_Animal {
    //para los enums el PartialEq es mas facil con match
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tipo_Animal::Perro, Tipo_Animal::Perro)
            | (Tipo_Animal::Gato, Tipo_Animal::Gato)
            | (Tipo_Animal::Caballo, Tipo_Animal::Caballo)
            | (Tipo_Animal::Otros, Tipo_Animal::Otros) => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Tipo_Animal,
    duenho: Persona,
}

impl Mascota {
    fn new(nombre: String, edad: u32, tipo: Tipo_Animal, duenho: Persona) -> Mascota {
        Mascota {
            nombre,
            edad,
            tipo,
            duenho,
        }
    }
}
impl PartialEq for Mascota {
    fn eq(&self, other: &Self) -> bool {
        self.nombre == other.nombre
            && self.edad == other.edad
            && self.tipo == other.tipo
            && self.duenho == other.duenho
    }
}
#[derive(Debug)]
struct Registro {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    requiere_proxima_visita: bool,
    fecha_proxima_visita: Fecha,
}
impl Registro {
    fn new(
        mascota: Mascota,
        diagnostico: String,
        tratamiento: String,
        requiere_proxima_visita: bool,
        fecha_proxima_visita: Fecha,
    ) -> Registro {
        Registro {
            mascota,
            diagnostico,
            tratamiento,
            requiere_proxima_visita,
            fecha_proxima_visita,
        }
    }
}
impl PartialEq for Registro {
    fn eq(&self, other: &Self) -> bool {
        self.mascota == other.mascota
            && self.diagnostico == other.diagnostico
            && self.tratamiento == other.tratamiento
            && self.requiere_proxima_visita == other.requiere_proxima_visita
            && self.fecha_proxima_visita == other.fecha_proxima_visita
    }
}
struct Veterinaria {
    nombre: String,
    direccio: String,
    id: u32,
    cola: VecDeque<Mascota>,
    registros: Vec<Registro>,
}
impl Veterinaria {
    //➔ crear una veterinaria.
    fn new(nombre: String, direccio: String, id: u32) -> Veterinaria {
        Veterinaria {
            nombre,
            direccio,
            id,
            cola: VecDeque::new(),
            registros: Vec::new(),
        }
    }
    //➔ agregar una nueva mascota a la cola de atención de la veterinaria.
    fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola.push_back(mascota);
    }
    //➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente en atender porque tiene la máxima prioridad.
    fn agregar_mascota_prioridad(&mut self, mascota: Mascota) {
        self.cola.push_front(mascota);
    }
    //➔ atender la próxima mascota de la cola.
    fn atender_proxima_mascota(&mut self) {
        if let Some(mascota) = self.cola.pop_front() {
            println!("Atendiendo a la mascota: {}", mascota.nombre);
            println!("guardando registro de la mascota atendida");
            self.registrar_atencion(mascota);
        }
    }
    //➔ registrar una atención.
    fn registrar_atencion(&mut self, mascota: Mascota) {
        self.registros.push(Registro::new(
            mascota,
            String::from("diagnostico"),
            String::from("tratamiento"),
            false,
            Fecha::new(0, 0, 0),
        ));
    }
    //➔ eliminar una mascota específica de la cola de atención dado que se retira.
    fn eliminar_una_mascota(&mut self, mascota: Mascota) -> bool {
        let dim_f: usize = self.cola.len();
        let mut i: usize = 0;
        let mut encontre: bool = false;
        if dim_f > 0 {
            while i < dim_f && !encontre {
                if self.cola[i] == mascota {
                    encontre = true;
                }
                i += 1;
            }
        }
        if encontre {
            i -= 1;
            self.cola.remove(i);
        }
        encontre
    }
    //➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el teléfono.
    //Mi estructuar Mascota ya tiene nombre de dueño y telefono...
    fn buscar_atencion(&self, mascota: Mascota) -> Option<&Registro> {
        let dim_f: usize = self.registros.len();
        let mut i: usize = 0;
        let mut encontre: bool = false;
        if dim_f > 0 {
            while i < dim_f && !encontre {
                if self.registros[i].mascota == mascota {
                    encontre = true;
                }
                i += 1;
            }
        }
        if encontre {
            i -= 1;
            return Some(&self.registros[i]);
        }
        None
    }

    //==============================Funcion_Helper======================================================
    fn buscar_atencion_registro(&mut self, mascota: &Mascota) -> Option<&mut Registro> {
        let dim_f = self.registros.len();
        let mut encontre = false;
        let mut i = 0;
        while i < dim_f && !encontre {
            if self.registros[i].mascota == *mascota {
                encontre = true;
            } else {
                i += 1;
            }
        }
        if encontre {
            Some(&mut self.registros[i])
        } else {
            None
        }
    }

    //==============================Busca_una_atencion_y_la_retorna_mutable======================================================

    //➔ modificar el diagnóstico de una determinada atención.   //Tema busquedas
    fn modificar_diagnostico(&mut self, mascota: Mascota, nuevo_diagnostico: String) -> bool {
        if let Some(registro) = self.buscar_atencion_registro(&mascota) {
            //reutilizo helper
            registro.diagnostico = nuevo_diagnostico;
            true
        } else {
            false
        }
    }
    //===========================================cache========================================================================
    //➔ modificar el diagnóstico de una determinada atención.   //Tema busquedas
    // en este caso no puedo reulitizar el buscar_atencion porque necesito un retorno de registro mutable
    //la estructura de busqueda que tengo actualmente arriba, es para retornar un registro inmutable (solo hace consulta)
    // fn modificar_diagnostico(&mut self, mascota: Mascota, nuevo_diagonostico: String) -> bool {
    //     let dim_f: usize = self.registros.len();
    //     let mut i: usize = 0;
    //     let mut encontre: bool = false;
    //     if dim_f > 0 {
    //         while i < dim_f && !encontre {
    //             if self.registros[i].mascota == mascota {
    //                 encontre = true;
    //             }
    //             i += 1;
    //         }
    //     }
    //     if encontre {
    //         i -= 1;
    //         self.registros[i].diagnostico = nuevo_diagonostico;
    //     }
    //     encontre
    // }
    // fn buscar_atencion(&mut self, mascota: &Mascota) -> Option<&mut Registro> {
    //     let dim_f = self.registros.len();
    //     let mut encontre = false;
    //     let mut i = 0;
    //     while i < dim_f && !encontre {
    //         if self.registros[i].mascota == *mascota {
    //             encontre = true;
    //         } else {
    //             i += 1;
    //         }
    //     }
    //     if encontre {
    //         Some(&mut self.registros[i])
    //     } else {
    //         None
    //     }
    // }

    //al haacer esto deberia modificar el buscar_atencion para que devuelva un mutable, pero necesaria de igual manera un modulo de busqueda(consulta)
    // fn modificar_diagnostico(&mut self, mascota: &Mascota, nuevo_diagnostico: String) -> bool {
    //     if let Some(registro) = self.buscar_atencion(mascota) {
    //         // Si encontramos el registro, actualizamos el diagnóstico
    //         registro.diagnostico = nuevo_diagnostico;
    //         true // Indicamos que se realizó la modificación exitosamente
    //     } else {
    //         false // Indicamos que la modificación no se pudo realizar porque no se encontró la mascota
    //     }
    // }

    //===========================================cache========================================================================

    //➔ modificar la fecha de la próxima visita de una determinada atención.
    fn modificar_fecha_proxima_visita(&mut self, mascota: Mascota, fecha: Fecha) -> bool {
        if let Some(registro) = self.buscar_atencion_registro(&mascota) {
            //reutilizo helper
            registro.fecha_proxima_visita = fecha;
            true
        } else {
            false
        }
    }

    //➔ eliminar una determinada atención.
    fn eliminar_atencion_registro(&mut self, mascota: Mascota) -> bool {
        let dim_f: usize = self.registros.len();
        let mut i: usize = 0;
        let mut encontre: bool = false;
        if dim_f > 0 {
            while i < dim_f && !encontre {
                if self.registros[i].mascota == mascota {
                    encontre = true;
                }
                i += 1;
            }
        }
        if encontre {
            i -= 1;
            self.registros.remove(i);
        }
        encontre
    }
}
//=====================================Fecha===================================================
#[derive(Debug)]
struct Fecha {
    dia: u32,
    mes: u32,
    anho: u32,
}
impl PartialEq for Fecha {
    fn eq(&self, other: &Self) -> bool {
        self.dia == other.dia && self.mes == other.mes && self.anho == other.anho
    }
}
impl Fecha {
    //todas las estructuras deberian hacer un llamado si fecha es valida
    //➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
    fn new(dia: u32, mes: u32, anho: u32) -> Fecha {
        Fecha { dia, mes, anho }
    }
    //➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
    fn es_bisiesto(&self) -> bool {
        let estado: bool = (self.anho % 4 == 0) && (self.anho % 100 != 0) || (self.anho % 400 == 0);
        return estado;
    }
    //helper que le paso un mes y me tira la cantidad de dias que tiene ese mes, incluso evalua si el anho es bisiesto
    fn cantidad_de_dias(&self, mes: u32) -> u32 {
        //funcion helper
        let dias_del_mes: u32 = match mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.es_bisiesto() {
                    29
                } else {
                    28
                }
            }
            _ => return 0,
        };
        dias_del_mes
    }
    //➢ es_fecha_valida: retorna true si es una fecha válida, false caso contrario.//tenga en cuenta los años bisiestos también.
    fn _es_fecha_valida(&self) -> bool {
        let mut estado: bool = false;
        if self.mes > 0 && self.mes < 13 && self.anho > 0 {
            println!("mes::: {}", self.mes);
            let dias_del_mes: u32 = self.cantidad_de_dias(self.mes);
            println!("cumpleeasdasd::: {}", dias_del_mes);
            println!("fechaEséradpda: {}", self.dia);
            estado = dias_del_mes != 0 && (self.dia >= 1 && self.dia <= dias_del_mes);
        }
        println!("{}", estado);
        estado
    }
    //➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
    fn _sumar_dias(&mut self, dias: u32) {
        let mut dias_del_mes = self.cantidad_de_dias(self.mes);
        self.dia += dias;

        // Mientras el dia actual sea mayor que la cantidad de dias en el mes actual
        while self.dia > dias_del_mes {
            // Restamos la cantidad de dias en el mes actual
            self.dia -= dias_del_mes;
            // Avanzamos al siguiente mes
            self.mes += 1;

            // Si el mes actual supera diciembre, avanzamos al siguiente año
            if self.mes > 12 {
                self.mes = 1;
                self.anho += 1;
            }
            // Actualizamos la cantidad de días en el nuevo mes
            dias_del_mes = self.cantidad_de_dias(self.mes);
        }
    }
    //➢ restar_dias(dias): resta la cantidad de dias a la fecha, modificandose
    fn restar_dias(&mut self, mut dias: u32) {
        // Mientras haya días para restar
        while dias != 0 {
            // Si la cantidad de dias a restar es mayor o igual que el dia actual
            if dias >= self.dia {
                // Restamos el día actual
                dias -= self.dia;
                // Retrocedemos al mes anterior
                self.mes -= 1;
                // Si el mes es menor que 1, retrocedemos al año anterior
                if self.mes < 1 {
                    self.mes = 12;
                    self.anho -= 1;
                }
                // Actualizamos la cantidad de dias en el nuevo mes
                self.dia = self.cantidad_de_dias(self.mes);
            } else {
                // Si la cantidad de dias a restar es menor que el dia actual,
                // simplemente restamos esa cantidad de días
                self.dia -= dias;
                dias = 0; // Ya no quedan dias por restar
            }
        }
    }
    //➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a la fecha pasada por parámetro.
    fn _es_mayor(&self, otra_fecha: &Fecha) -> bool {
        let mut mayor: bool = false;
        if self.anho > otra_fecha.anho {
            mayor = true;
        } else if self.anho == otra_fecha.anho && self.mes > otra_fecha.mes {
            mayor = true;
        } else if self.anho == otra_fecha.anho
            && self.mes == otra_fecha.mes
            && self.dia > otra_fecha.dia
        {
            mayor = true;
        }
        mayor
    }

    //esta funcion asociada no me la piden.. pero para probar
    fn to_string(&self) -> String {
        return format!("dia: {} mes: {} anho: {}", self.dia, self.mes, self.anho);
    }
}

//=====================================Fin_Fecha===================================================

pub fn main() {
    // Obtener la fecha y hora local actual
    // let fecha_actual: DateTime<Local> = Local::now();
    // println!(
    //     "Fecha actual del sistema operativo: {}",
    //     fecha_actual.format("%Y-%m-%d %H:%M:%S")
    // );
    //------------------------------Fecha---------------------------------------------
    // Obtener solo la fecha actual (sin la hora)
    // let fecha_actual_solo: NaiveDate = Local::now().date_naive();
    // println!(
    //     "Fecha actual (solo fecha): {}",
    //     fecha_actual_solo.format("%Y-%m-%d")
    // );
    //---------------------------------------------------------------------------
}
//solo dios y yo sabemos como funcionan estos test
#[test]
fn test_agregar_mascota() {
    let mut veterinaria = Veterinaria::new(String::from("Vet1"), String::from("Direccion 1"), 1);
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Dueño1"),
            String::from("Direccion 2"),
            123456789,
        ),
    );
    veterinaria.agregar_mascota(mascota);
    assert_eq!(veterinaria.cola.len(), 1);
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Dueño1"),
            String::from("Direccion 2"),
            123456789,
        ),
    );

    assert_eq!(veterinaria.cola[0], mascota);
    //pasa bien
}

#[test]
fn test_agregar_mascota_prioridad() {
    let mut veterinaria = Veterinaria::new(String::from("Vet2"), String::from("Direccion 2"), 2);
    let mascota1 = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Duenho2"),
            String::from("Direcci 3"),
            987654321,
        ),
    );
    let mascota2 = Mascota::new(
        String::from("Mascota2"),
        3,
        Tipo_Animal::Gato,
        Persona::new(
            String::from("Duenho3"),
            String::from("Direcci 4"),
            456789123,
        ),
    );
    veterinaria.agregar_mascota(mascota1);
    veterinaria.agregar_mascota_prioridad(mascota2);
    assert_eq!(veterinaria.cola.len(), 2);
    let mascota1 = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Duenho2"),
            String::from("Direcci 3"),
            987654321,
        ),
    );
    let mascota2 = Mascota::new(
        String::from("Mascota2"),
        3,
        Tipo_Animal::Gato,
        Persona::new(
            String::from("Duenho3"),
            String::from("Direcci 4"),
            456789123,
        ),
    );
    assert_eq!(veterinaria.cola[0], mascota2);
    assert_eq!(veterinaria.cola[1], mascota1);
    //pasa bien
}
#[test]
fn test_atender_proxima_mascota() {
    let mut veterinaria = Veterinaria::new(String::from("Vet3"), String::from("Direccion 3"), 3);
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Duenho4"),
            String::from("Direcci 5"),
            987654321,
        ),
    );
    veterinaria.agregar_mascota(mascota);
    veterinaria.atender_proxima_mascota();
    assert_eq!(veterinaria.cola.len(), 0);
    assert_eq!(veterinaria.registros.len(), 1); // Deberia ser 1 porque se atendio a la mascota
                                                //pasa bien
}
#[test]
fn test_eliminar_una_mascota() {
    let mut veterinaria = Veterinaria::new(String::from("Vet4"), String::from("Direccion 4"), 4);
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Dueño5"),
            String::from("Direccion 6"),
            123456789,
        ),
    );
    veterinaria.agregar_mascota(mascota);
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Dueño5"),
            String::from("Direccion 6"),
            123456789,
        ),
    );
    let eliminada = veterinaria.eliminar_una_mascota(mascota);
    assert_eq!(eliminada, true);
    assert_eq!(veterinaria.cola.len(), 0);
}

#[test]
fn test_registrar_atencion() {
    let mut veterinaria = Veterinaria::new(String::from("Vet5"), String::from("Direccion 5"), 5);
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Dueño6"),
            String::from("Direccion 7"),
            123456789,
        ),
    );
    veterinaria.registrar_atencion(mascota);
    assert_eq!(veterinaria.registros.len(), 1);
}
#[test]

fn test_buscar_atencion() {
    let mut veterinaria = Veterinaria::new(String::from("Vet6"), String::from("Direccion 6"), 6);
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Dueño7"),
            String::from("Direccion 8"),
            123456789,
        ),
    );
    veterinaria.registrar_atencion(mascota);
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Dueño7"),
            String::from("Direccion 8"),
            123456789,
        ),
    );
    let registro: Registro = Registro::new(
        mascota,
        String::from("diagnostico"),
        String::from("tratamiento"),
        false,
        Fecha::new(0, 0, 0),
    );
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Dueño7"),
            String::from("Direccion 8"),
            123456789,
        ),
    );
    // Verificamos que el registro devuelto sea igual al que esperamos
    assert_eq!(veterinaria.buscar_atencion(mascota), Some(&registro));
}

#[test]
fn test_modificar_diagnostico() {
    let mut veterinaria = Veterinaria::new(String::from("Vet7"), String::from("Direccion 7"), 7);
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Dueño8"),
            String::from("Direccion 9"),
            123456789,
        ),
    );
    veterinaria.registrar_atencion(mascota);
    let mascota = Mascota::new(
        String::from("Mascota1"),
        5,
        Tipo_Animal::Perro,
        Persona::new(
            String::from("Dueño8"),
            String::from("Direccion 9"),
            123456789,
        ),
    );
    let modificado = veterinaria.modificar_diagnostico(mascota, String::from("Nuevo Diagnóstico"));
    assert_eq!(modificado, true);
}
