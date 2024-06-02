/*
4- En base al ejercicio 10 del tp#3 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Tanto los libros con sus copias como la administración de préstamos se realizan
sobre archivos en formato JSON. Realice las modificaciones pertinentes para poder hacerlo
así. No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
haga métodos nuevos para cumplir con este punto . Recuerde también que se debe seguir
manteniendo un coverage de al menos 90%.


*/

/*
    10-Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
    biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
    prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
    la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
    cada libro se conoce el isbn, el título, autor, número de páginas, género(novela, infantil,
    técnico, otros).

    Para registrar un préstamo se requiere el libro, el cliente, la fecha de
    vencimiento del préstamo, la fecha de devolución y el estado que puede ser devuelto o en
    préstamo. Del cliente se conoce el nombre, teléfono y dirección de correo electrónico.
    Implemente los métodos necesarios para realizar las siguientes acciones


  ➔ obtener cantidad de copias: dado un determinado libro retorna la cantidad de
  copias a disposición que hay para prestar de dicho libro.
  ➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1
  la cantidad de copias de libros a disposición para prestar.
  ➔ incrementar cantidad de copias a disposición: dado un libro incrementa en 1
  la cantidad de copias del libro a disposición para ser prestado.
  ➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado
  “en préstamo” de un determinado cliente.

  ➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro
      para un determinado cliente cumpliendo con lo siguiente
      ◆ elcliente no tenga más de 5 préstamos en el estado “en préstamo”
      ◆ haya almenos una copia disponible en el registro de copias a
      disposición.
  De ser así descuenta 1 en el registro de “copias a disposición” y
  retorna true, si no cumple con alguna de las condiciones retorna false.

  ➔ verpréstamos a vencer el los próximos días: retorna una lista de préstamos a
  vencer el los próximos días, el valor de días es pasado por parámetro.
  ➔ verlos préstamos vencidos: retorna una lista de préstamos en el estado “en
  préstamos” donde la fecha de vencimiento es menor a la fecha actual.
  ➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si
  existe.
  ➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
  estado “devuelto”, se registra la fecha de devolución y se incrementa la
  cantidad de libros en 1 del libro devuelto en el registro de copias a
  disposición.
  Nota: para la fecha utilice lo implementado en el punto 3

*/
use chrono::Datelike;
use chrono::{DateTime, Local};

struct Biblioteca {
    nombre: String,
    direccion: String,
    libros_disponibles: Vec<Registro_libros>,
    libros_prestados: Vec<Prestamo>,
}
#[derive(Debug, Clone)]
struct Registro_libros {
    libro: Libro,
    cantidad_ejemplares: u32,
}
#[derive(Debug, Clone)]

struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    num_paginas: u32,
    genero: Genero,
}
#[derive(Debug, Clone)]

enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Fecha,
    estado: Estado,
}
enum Estado {
    Devuelto,
    EnPrestamo,
}
#[derive(Debug, Clone)]
struct Cliente {
    nombre: String,
    telefono: String,
    email: String,
}
//============================Implementaciones_clone_eq=======================================
impl PartialEq for Libro {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
            && self.titulo == other.titulo
            && self.autor == other.autor
            && self.num_paginas == other.num_paginas
            && self.genero == other.genero
    }
}
impl PartialEq for Genero {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Genero::Novela, Genero::Novela)
            | (Genero::Infantil, Genero::Infantil)
            | (Genero::Tecnico, Genero::Tecnico)
            | (Genero::Otros, Genero::Otros) => true,
            _ => false,
        }
    }
}
impl PartialEq for Prestamo {
    fn eq(&self, other: &Self) -> bool {
        self.libro == other.libro
            && self.cliente == other.cliente
            && self.fecha_vencimiento == other.fecha_vencimiento
            && self.fecha_devolucion == other.fecha_devolucion
            && self.estado == other.estado
    }
}
impl PartialEq for Estado {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Estado::Devuelto, Estado::Devuelto) | (Estado::EnPrestamo, Estado::EnPrestamo) => true,
            _ => false,
        }
    }
}
impl PartialEq for Cliente {
    fn eq(&self, other: &Self) -> bool {
        self.nombre == other.nombre && self.telefono == other.telefono && self.email == other.email
    }
}
impl PartialEq for Fecha {
    fn eq(&self, other: &Self) -> bool {
        self.dia == other.dia && self.mes == other.mes && self.anho == other.anho
    }
}
impl Prestamo {
    fn new(
        libro: Libro,
        cliente: Cliente,
        fecha_vencimiento: Fecha,
        fecha_devolucion: Fecha,
        estado: Estado,
    ) -> Prestamo {
        Prestamo {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion,
            estado,
        }
    }
}
//============================Funciones_asociadas=======================================

impl Biblioteca {
    //helper que busca un libro y me devuelve el registro de ese libro
    fn buscar_prestamo(&self, libro: &Libro) -> Option<&Registro_libros> {
        let dim_f: usize = self.libros_disponibles.len();
        let mut i: usize = 0;
        let mut encontre: bool = false;
        if dim_f > 0 {
            while i < dim_f && !encontre {
                if self.libros_disponibles[i].libro == *libro {
                    encontre = true;
                }
                i += 1;
            }
        }
        if encontre {
            return Some(&self.libros_disponibles[i - 1]);
        } else {
            None
        }
    }
    //➔ obtener cantidad de copias: dado un determinado libro retorna la cantidad de copias a disposición que hay para prestar de dicho libro.
    fn obtener_cantidad_de_copias(&self, libro: &Libro) -> u32 {
        let mut cant: u32 = 0;
        if let Some(registro) = self.buscar_prestamo(&libro) {
            cant = registro.cantidad_ejemplares;
        }
        cant
    }

    fn buscar_prestamo_mutable(&mut self, libro: &Libro) -> Option<&mut Registro_libros> {
        let dim_f: usize = self.libros_disponibles.len();
        let mut i: usize = 0;
        let mut encontre: bool = false;
        if dim_f > 0 {
            while i < dim_f && !encontre {
                if self.libros_disponibles[i].libro == *libro {
                    encontre = true;
                }
                i += 1;
            }
        }
        if encontre {
            return Some(&mut self.libros_disponibles[i - 1]);
        } else {
            None
        }
    }
    //➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1
    // la cantidad de copias de libros a disposición para prestar.
    fn decrementar_cantidad_de_copias(&mut self, libro: &Libro) -> bool {
        let mut pude = false;
        if let Some(registro) = self.buscar_prestamo_mutable(&libro) {
            if registro.cantidad_ejemplares > 0 {
                registro.cantidad_ejemplares -= 1;
                pude = true;
            }
        }
        pude
    }
    //➔ incrementar cantidad de copias a disposición: dado un libro incrementa en 1
    // la cantidad de copias del libro a disposición para ser prestado.
    fn incrementar_cantidad_de_copias(&mut self, libro: &Libro) -> bool {
        let mut pude: bool = false;
        if let Some(registro) = self.buscar_prestamo_mutable(&libro) {
            registro.cantidad_ejemplares += 1;
            pude = true;
        }
        pude
    }
    //➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado
    // “en préstamo” de un determinado cliente.
    fn contar_prestamos_de_un_cliente(&self, cliente: &Cliente) -> u32 {
        let mut cantidad: u32 = 0;
        for prestamo in &self.libros_prestados {
            if prestamo.cliente == *cliente && prestamo.estado == Estado::EnPrestamo {
                cantidad += 1;
            }
        }
        cantidad
    }

    //   ➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro
    //   para un determinado cliente cumpliendo con lo siguiente
    //   ◆ elcliente no tenga más de 5 préstamos en el estado “en préstamo”
    //   ◆ haya almenos una copia disponible en el registro de copias a
    //   disposición.
    //   De ser así descuenta 1 en el registro de “copias a disposición” y
    //   retorna true, si no cumple con alguna de las condiciones retorna false.
    fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente) -> bool {
        let mut pude: bool = false;
        let mut cantidad_prestamos = self.contar_prestamos_de_un_cliente(&cliente);
        if cantidad_prestamos < 5 {
            if let Some(registro) = self.buscar_prestamo_mutable(&libro) {
                if registro.cantidad_ejemplares > 0 {
                    registro.cantidad_ejemplares -= 1;
                    pude = true;
                    // let fecha_vencimiento = Fecha::new(1, 1, 2022);
                    // let fecha_devolucion = Fecha::new(1, 5, 2022);
                    //====================Probando_con_fecha_del_OS================================

                    let fecha_actual = Local::now();
                    let fecha_actual = Fecha::new(
                        fecha_actual.day() as u32,
                        fecha_actual.month() as u32,
                        fecha_actual.year() as u32,
                    );
                    let fecha_vencimiento = fecha_actual;

                    let fecha_actual = Local::now();
                    let fecha_actual = Fecha::new(
                        fecha_actual.day() as u32,
                        fecha_actual.month() as u32,
                        fecha_actual.year() as u32,
                    );
                    let fecha_devolucion = fecha_actual;
                    //====================================================

                    let prestamo: Prestamo = Prestamo::new(
                        libro,
                        cliente,
                        fecha_vencimiento,
                        fecha_devolucion,
                        Estado::EnPrestamo,
                    );
                    self.libros_prestados.push(prestamo);
                }
            }
        }
        pude
    }

    //  ➔ verpréstamos a vencer el los próximos días: retorna una lista de préstamos a vencer el los próximos días, el valor de días es pasado por parámetro.
    fn ver_prestamos_a_vencer(&self, dias: u32) -> Vec<&Prestamo> {
        let mut prestamos_a_vencer: Vec<&Prestamo> = Vec::new();
        let fecha_actual = Local::now();
        let fecha_actual = Fecha::new(
            fecha_actual.day() as u32,
            fecha_actual.month() as u32,
            fecha_actual.year() as u32,
        );
        for prestamo in &self.libros_prestados {
            let dias_que_faltan = fecha_actual.dia - prestamo.fecha_vencimiento.dia;
            if dias_que_faltan <= dias {
                prestamos_a_vencer.push(prestamo);
            }
        }
        prestamos_a_vencer
    }

    // ➔ verlos préstamos vencidos: retorna una lista de préstamos en el estado “en  préstamos” donde la fecha de vencimiento es menor a la fecha actual.
    fn ver_prestamos_vencidos(&self) -> Vec<&Prestamo> {
        let mut prestamos_vencidos: Vec<&Prestamo> = Vec::new();
        let fecha_actual = Local::now();
        let fecha_actual = Fecha::new(
            fecha_actual.day() as u32,
            fecha_actual.month() as u32,
            fecha_actual.year() as u32,
        );
        for prestamo in &self.libros_prestados {
            if prestamo.fecha_vencimiento._es_menor(&fecha_actual) {
                prestamos_vencidos.push(prestamo);
            }
        }
        prestamos_vencidos
    }

    // ➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si  existe.
    fn buscar_prestamo_a(&mut self, libro: &Libro, cliente: &Cliente) -> Option<&mut Prestamo> {
        let mut prestamo: Option<&mut Prestamo> = None; // Si no encuentra nada en el for retorno un None
        for p in &mut self.libros_prestados {
            if p.libro == *libro && p.cliente == *cliente {
                prestamo = Some(p);
                break; // Termina el bucle una vez que se encuentra el préstamo
            }
        }
        prestamo
    }
    // ➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
    // estado “devuelto”, se registra la fecha de devolución y se incrementa la
    // cantidad de libros en 1 del libro devuelto en el registro de copias a
    // disposición.
    fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente) -> bool {
        let mut pude: bool = false;
        if let Some(prestamo) = self.buscar_prestamo_a(&libro, &cliente) {
            pude = true;
            prestamo.estado = Estado::Devuelto;
            let fecha_actual = Local::now();
            let fecha_actual = Fecha::new(
                fecha_actual.day() as u32,
                fecha_actual.month() as u32,
                fecha_actual.year() as u32,
            );
            prestamo.fecha_devolucion = fecha_actual;
            self.incrementar_cantidad_de_copias(&libro);
        }
        pude
    }
}

//=============================FECHA================================
#[derive(Debug, Clone)]
struct Fecha {
    dia: u32,
    mes: u32,
    anho: u32,
}

impl Fecha {
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
    fn _es_menor(&self, otra_fecha: &Fecha) -> bool {
        if self.anho < otra_fecha.anho {
            return true;
        } else if self.anho == otra_fecha.anho && self.mes < otra_fecha.mes {
            return true;
        } else if self.anho == otra_fecha.anho
            && self.mes == otra_fecha.mes
            && self.dia < otra_fecha.dia
        {
            return true;
        }
        false
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
//=============================FECHA================================

pub fn main() {}
#[test]
fn test_obtener_cantidad_de_copias() {
    let libro1 = Libro {
        isbn: "123".to_string(),
        titulo: "titulo1".to_string(),
        autor: "autor1".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro2 = Libro {
        isbn: "124".to_string(),
        titulo: "titulo2".to_string(),
        autor: "autor2".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro3 = Libro {
        isbn: "125".to_string(),
        titulo: "titulo3".to_string(),
        autor: "autor3".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let registro_libro1 = Registro_libros {
        libro: libro1.clone(),
        cantidad_ejemplares: 5,
    };
    let registro_libro2 = Registro_libros {
        libro: libro2.clone(),
        cantidad_ejemplares: 3,
    };
    let registro_libro3 = Registro_libros {
        libro: libro3.clone(),
        cantidad_ejemplares: 2,
    };
    let biblioteca = Biblioteca {
        nombre: "biblioteca".to_string(),
        direccion: "direccion".to_string(),
        libros_disponibles: vec![
            registro_libro1.clone(),
            registro_libro2.clone(),
            registro_libro3.clone(),
        ],
        libros_prestados: vec![],
    };
    assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro1), 5);
    assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro2), 3);
    assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro3), 2);
}
#[test]
fn test_decrementar_cantidad_de_copias() {
    let libro1 = Libro {
        isbn: "123".to_string(),
        titulo: "titulo1".to_string(),
        autor: "autor1".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro2 = Libro {
        isbn: "124".to_string(),
        titulo: "titulo2".to_string(),
        autor: "autor2".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro3 = Libro {
        isbn: "125".to_string(),
        titulo: "titulo3".to_string(),
        autor: "autor3".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let registro_libro1 = Registro_libros {
        libro: libro1.clone(),
        cantidad_ejemplares: 5,
    };
    let registro_libro2 = Registro_libros {
        libro: libro2.clone(),
        cantidad_ejemplares: 3,
    };
    let registro_libro3 = Registro_libros {
        libro: libro3.clone(),
        cantidad_ejemplares: 2,
    };
    let mut biblioteca = Biblioteca {
        nombre: "biblioteca".to_string(),
        direccion: "direccion".to_string(),
        libros_disponibles: vec![
            registro_libro1.clone(),
            registro_libro2.clone(),
            registro_libro3.clone(),
        ],
        libros_prestados: vec![],
    };
    assert_eq!(biblioteca.decrementar_cantidad_de_copias(&libro1), true);
    assert_eq!(biblioteca.decrementar_cantidad_de_copias(&libro1), true);
    assert_eq!(biblioteca.decrementar_cantidad_de_copias(&libro1), true);
    assert_eq!(biblioteca.decrementar_cantidad_de_copias(&libro1), true);
    assert_eq!(biblioteca.decrementar_cantidad_de_copias(&libro1), true);
    assert_eq!(biblioteca.decrementar_cantidad_de_copias(&libro1), false);
}

#[test]
fn test_incrementar_cantidad_de_copias() {
    let libro1 = Libro {
        isbn: "123".to_string(),
        titulo: "titulo1".to_string(),
        autor: "autor1".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro2 = Libro {
        isbn: "124".to_string(),
        titulo: "titulo2".to_string(),
        autor: "autor2".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro3 = Libro {
        isbn: "125".to_string(),
        titulo: "titulo3".to_string(),
        autor: "autor3".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let registro_libro1 = Registro_libros {
        libro: libro1.clone(),
        cantidad_ejemplares: 5,
    };
    let registro_libro2 = Registro_libros {
        libro: libro2.clone(),
        cantidad_ejemplares: 3,
    };
    let registro_libro3 = Registro_libros {
        libro: libro3.clone(),
        cantidad_ejemplares: 2,
    };
    let mut biblioteca = Biblioteca {
        nombre: "biblioteca".to_string(),
        direccion: "direccion".to_string(),
        libros_disponibles: vec![
            registro_libro1.clone(),
            registro_libro2.clone(),
            registro_libro3.clone(),
        ],
        libros_prestados: vec![],
    };
    assert_eq!(biblioteca.incrementar_cantidad_de_copias(&libro1), true);
    assert_eq!(biblioteca.incrementar_cantidad_de_copias(&libro1), true);
    assert_eq!(biblioteca.incrementar_cantidad_de_copias(&libro1), true);
    assert_eq!(biblioteca.incrementar_cantidad_de_copias(&libro1), true);
    assert_eq!(biblioteca.incrementar_cantidad_de_copias(&libro1), true);
    assert_eq!(biblioteca.incrementar_cantidad_de_copias(&libro1), true);
}
#[test]
fn test_contar_prestamos_de_un_cliente() {
    let libro1 = Libro {
        isbn: "123".to_string(),
        titulo: "titulo1".to_string(),
        autor: "autor1".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro2 = Libro {
        isbn: "124".to_string(),
        titulo: "titulo2".to_string(),
        autor: "autor2".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro3 = Libro {
        isbn: "125".to_string(),
        titulo: "titulo3".to_string(),
        autor: "autor3".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let registro_libro1 = Registro_libros {
        libro: libro1.clone(),
        cantidad_ejemplares: 5,
    };
    let registro_libro2 = Registro_libros {
        libro: libro2.clone(),
        cantidad_ejemplares: 3,
    };
    let registro_libro3 = Registro_libros {
        libro: libro3.clone(),
        cantidad_ejemplares: 2,
    };
    let cliente1 = Cliente {
        nombre: "cliente1".to_string(),
        telefono: "telefono1".to_string(),
        email: "email1".to_string(),
    };
    let cliente2 = Cliente {
        nombre: "cliente2".to_string(),
        telefono: "telefono2".to_string(),
        email: "email2".to_string(),
    };
    let mut biblioteca = Biblioteca {
        nombre: "biblioteca".to_string(),
        direccion: "direccion".to_string(),
        libros_disponibles: vec![
            registro_libro1.clone(),
            registro_libro2.clone(),
            registro_libro3.clone(),
        ],
        libros_prestados: vec![],
    };
    biblioteca.realizar_prestamo(libro1.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro1.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro1.clone(), cliente2.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente2.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente2.clone());
    assert_eq!(biblioteca.contar_prestamos_de_un_cliente(&cliente1), 5);
    assert_eq!(biblioteca.contar_prestamos_de_un_cliente(&cliente2), 3);
}

#[test]
fn test_ver_prestamo() {
    let libro1 = Libro {
        isbn: "123".to_string(),
        titulo: "titulo1".to_string(),
        autor: "autor1".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro2 = Libro {
        isbn: "124".to_string(),
        titulo: "titulo2".to_string(),
        autor: "autor2".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro3 = Libro {
        isbn: "125".to_string(),
        titulo: "titulo3".to_string(),
        autor: "autor3".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let registro_libro1 = Registro_libros {
        libro: libro1.clone(),
        cantidad_ejemplares: 5,
    };
    let registro_libro2 = Registro_libros {
        libro: libro2.clone(),
        cantidad_ejemplares: 3,
    };
    let registro_libro3 = Registro_libros {
        libro: libro3.clone(),
        cantidad_ejemplares: 2,
    };
    let cliente1 = Cliente {
        nombre: "cliente1".to_string(),
        telefono: "telefono1".to_string(),
        email: "email1".to_string(),
    };
    let cliente2 = Cliente {
        nombre: "cliente2".to_string(),
        telefono: "telefono2".to_string(),
        email: "email2".to_string(),
    };
    let mut biblioteca = Biblioteca {
        nombre: "biblioteca".to_string(),
        direccion: "direccion".to_string(),
        libros_disponibles: vec![
            registro_libro1.clone(),
            registro_libro2.clone(),
            registro_libro3.clone(),
        ],
        libros_prestados: vec![],
    };
    biblioteca.realizar_prestamo(libro1.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro1.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro1.clone(), cliente2.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente2.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente2.clone());
    assert_eq!(
        biblioteca
            .buscar_prestamo_a(&libro1, &cliente1)
            .unwrap()
            .libro,
        libro1
    );
    assert_eq!(
        biblioteca
            .buscar_prestamo_a(&libro2, &cliente1)
            .unwrap()
            .libro,
        libro2
    );
    assert_eq!(
        biblioteca
            .buscar_prestamo_a(&libro3, &cliente1)
            .unwrap()
            .libro,
        libro3
    );
    assert_eq!(
        biblioteca
            .buscar_prestamo_a(&libro1, &cliente2)
            .unwrap()
            .libro,
        libro1
    );
    assert_eq!(
        biblioteca
            .buscar_prestamo_a(&libro2, &cliente2)
            .unwrap()
            .libro,
        libro2
    );
    assert_eq!(
        biblioteca
            .buscar_prestamo_a(&libro3, &cliente2)
            .unwrap()
            .libro,
        libro3
    );
}

#[test]
fn ver_prestamos_vencidos() {
    let libro1 = Libro {
        isbn: "123".to_string(),
        titulo: "titulo1".to_string(),
        autor: "autor1".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro2 = Libro {
        isbn: "124".to_string(),
        titulo: "titulo2".to_string(),
        autor: "autor2".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro3 = Libro {
        isbn: "125".to_string(),
        titulo: "titulo3".to_string(),
        autor: "autor3".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let registro_libro1 = Registro_libros {
        libro: libro1.clone(),
        cantidad_ejemplares: 5,
    };
    let registro_libro2 = Registro_libros {
        libro: libro2.clone(),
        cantidad_ejemplares: 3,
    };
    let registro_libro3 = Registro_libros {
        libro: libro3.clone(),
        cantidad_ejemplares: 2,
    };
    let cliente1 = Cliente {
        nombre: "cliente1".to_string(),
        telefono: "telefono1".to_string(),
        email: "email1".to_string(),
    };
    let cliente2 = Cliente {
        nombre: "cliente2".to_string(),
        telefono: "telefono2".to_string(),
        email: "email2".to_string(),
    };
    let mut biblioteca = Biblioteca {
        nombre: "biblioteca".to_string(),
        direccion: "direccion".to_string(),
        libros_disponibles: vec![
            registro_libro1.clone(),
            registro_libro2.clone(),
            registro_libro3.clone(),
        ],
        libros_prestados: vec![],
    };
    biblioteca.realizar_prestamo(libro1.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro1.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro1.clone(), cliente2.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente2.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente2.clone());
    let fecha_actual = Local::now();
    let fecha_actual = Fecha::new(
        fecha_actual.day() as u32,
        fecha_actual.month() as u32,
        fecha_actual.year() as u32,
    );
    let fecha_vencimiento = Fecha::new(1, 1, 2022);
    let fecha_devolucion = Fecha::new(1, 5, 2035);
    let prestamo1 = Prestamo::new(
        libro1.clone(),
        cliente1.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo2 = Prestamo::new(
        libro2.clone(),
        cliente1.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo3 = Prestamo::new(
        libro3.clone(),
        cliente1.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo4 = Prestamo::new(
        libro1.clone(),
        cliente2.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo5 = Prestamo::new(
        libro2.clone(),
        cliente2.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo6 = Prestamo::new(
        libro3.clone(),
        cliente2.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );

    biblioteca.libros_prestados = vec![
        prestamo1, prestamo2, prestamo3, prestamo4, prestamo5, prestamo6,
    ];
    let casos = biblioteca.ver_prestamos_vencidos();
    assert_eq!(casos.len(), 6);
}

#[test]
fn ver_prestamos_a_vencer() {
    let libro1 = Libro {
        isbn: "123".to_string(),
        titulo: "titulo1".to_string(),
        autor: "autor1".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro2 = Libro {
        isbn: "124".to_string(),
        titulo: "titulo2".to_string(),
        autor: "autor2".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro3 = Libro {
        isbn: "125".to_string(),
        titulo: "titulo3".to_string(),
        autor: "autor3".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let registro_libro1 = Registro_libros {
        libro: libro1.clone(),
        cantidad_ejemplares: 5,
    };
    let registro_libro2 = Registro_libros {
        libro: libro2.clone(),
        cantidad_ejemplares: 3,
    };
    let registro_libro3 = Registro_libros {
        libro: libro3.clone(),
        cantidad_ejemplares: 2,
    };
    let cliente1 = Cliente {
        nombre: "cliente1".to_string(),
        telefono: "telefono1".to_string(),
        email: "email1".to_string(),
    };
    let cliente2 = Cliente {
        nombre: "cliente2".to_string(),
        telefono: "telefono2".to_string(),
        email: "email2".to_string(),
    };
    let mut biblioteca = Biblioteca {
        nombre: "biblioteca".to_string(),
        direccion: "direccion".to_string(),
        libros_disponibles: vec![
            registro_libro1.clone(),
            registro_libro2.clone(),
            registro_libro3.clone(),
        ],
        libros_prestados: vec![],
    };
    biblioteca.realizar_prestamo(libro1.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro1.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro1.clone(), cliente2.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente2.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente2.clone());
    let fecha_actual = Local::now();
    let fecha_actual = Fecha::new(
        fecha_actual.day() as u32,
        fecha_actual.month() as u32,
        fecha_actual.year() as u32,
    );
    let fecha_vencimiento = Fecha::new(1, 1, 2022);
    let fecha_devolucion = Fecha::new(1, 5, 2022);
    let prestamo1 = Prestamo::new(
        libro1.clone(),
        cliente1.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo2 = Prestamo::new(
        libro2.clone(),
        cliente1.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo3 = Prestamo::new(
        libro3.clone(),
        cliente1.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo4 = Prestamo::new(
        libro1.clone(),
        cliente2.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo5 = Prestamo::new(
        libro2.clone(),
        cliente2.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo6 = Prestamo::new(
        libro3.clone(),
        cliente2.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    assert!(biblioteca.ver_prestamos_a_vencer(1).len() > 0);
}
#[test]
fn devolver_libro() {
    let libro1 = Libro {
        isbn: "123".to_string(),
        titulo: "titulo1".to_string(),
        autor: "autor1".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let libro2 = Libro {
        isbn: "124".to_string(),
        titulo: "titulo2".to_string(),
        autor: "autor2".to_string(),
        num_paginas: 100,
        genero: Genero::Novela,
    };
    let Registro_libros1 = Registro_libros {
        libro: libro1.clone(),
        cantidad_ejemplares: 5,
    };
    let Registro_libros2 = Registro_libros {
        libro: libro2.clone(),
        cantidad_ejemplares: 3,
    };
    let cliente1 = Cliente {
        nombre: "cliente1".to_string(),
        telefono: "telefono1".to_string(),
        email: "email1".to_string(),
    };
    let cliente2 = Cliente {
        nombre: "cliente2".to_string(),
        telefono: "telefono2".to_string(),
        email: "email2".to_string(),
    };
    let mut biblioteca = Biblioteca {
        nombre: "biblioteca".to_string(),
        direccion: "direccion".to_string(),
        libros_disponibles: vec![Registro_libros1.clone(), Registro_libros2.clone()],
        libros_prestados: vec![],
    };
    biblioteca.realizar_prestamo(libro1.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente1.clone());
    biblioteca.realizar_prestamo(libro1.clone(), cliente2.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente2.clone());
    let fecha_actual = Local::now();
    let fecha_actual = Fecha::new(
        fecha_actual.day() as u32,
        fecha_actual.month() as u32,
        fecha_actual.year() as u32,
    );
    let fecha_vencimiento = Fecha::new(1, 1, 2022);
    let fecha_devolucion = Fecha::new(1, 5, 2022);
    let prestamo1 = Prestamo::new(
        libro1.clone(),
        cliente1.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo2 = Prestamo::new(
        libro2.clone(),
        cliente1.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo3 = Prestamo::new(
        libro1.clone(),
        cliente2.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    let prestamo4 = Prestamo::new(
        libro2.clone(),
        cliente2.clone(),
        fecha_vencimiento.clone(),
        fecha_devolucion.clone(),
        Estado::EnPrestamo,
    );
    biblioteca.libros_prestados = vec![prestamo1, prestamo2, prestamo3, prestamo4];
    assert!(biblioteca.devolver_libro(&libro1, &cliente1));
    assert!(biblioteca.devolver_libro(&libro2, &cliente1));
    assert!(biblioteca.devolver_libro(&libro1, &cliente2));
    assert!(biblioteca.devolver_libro(&libro2, &cliente2));
}
#[test]
fn test_es_bisiesto() {
    //deberian ser 2 test separados,igual andan, q solo me da la data de uno (si falla me dice igual)
    // Año bisiesto
    assert_eq!(Fecha::new(1, 1, 2020).es_bisiesto(), true);
    // Año no bisiesto
    assert_eq!(Fecha::new(1, 1, 2021).es_bisiesto(), false);
}

#[test]
fn test_es_fecha_valida() {
    //pasan los 2
    // Fecha válida
    assert_eq!(Fecha::new(1, 1, 2022)._es_fecha_valida(), true);
    // Fecha inválida
    assert_eq!(Fecha::new(32, 2, 2022)._es_fecha_valida(), false);
}

#[test]
fn test_sumar_dias() {
    let mut fecha = Fecha::new(31, 12, 2022);
    fecha._sumar_dias(1);
    assert_eq!(fecha.to_string(), "dia: 1 mes: 1 anho: 2023");
    // Suma de días que pasan a otro mes
    let mut fecha = Fecha::new(30, 12, 2022);
    fecha._sumar_dias(3);
    assert_eq!(fecha.to_string(), "dia: 2 mes: 1 anho: 2023");
}

#[test]
fn test_restar_dias() {
    let mut fecha = Fecha::new(1, 1, 2023);
    fecha.restar_dias(1);
    assert_eq!(fecha.to_string(), "dia: 31 mes: 12 anho: 2022");
    // Resta de días que pasan a otro mes
    let mut fecha = Fecha::new(1, 1, 2023);
    fecha.restar_dias(3);
    assert_eq!(fecha.to_string(), "dia: 29 mes: 12 anho: 2022");
}

#[test]
fn test_es_mayor() {
    let fecha1 = Fecha::new(1, 1, 2023);
    let fecha2 = Fecha::new(31, 12, 2022);
    assert_eq!(fecha1._es_mayor(&fecha2), true);
}

#[test]
fn test_es_menor() {
    let fecha1 = Fecha::new(1, 1, 2023);
    let fecha2 = Fecha::new(31, 12, 2022);
    assert_eq!(fecha2._es_mayor(&fecha1), false);
}
#[test]
fn test_es_menor_true() {
    let fecha1 = Fecha::new(1, 1, 2020);
    let fecha2 = Fecha::new(2, 1, 2020);
    let fecha3 = Fecha::new(1, 2, 2020);
    let fecha4 = Fecha::new(1, 1, 2021);

    assert!(fecha1._es_menor(&fecha2)); // Día es menor
    assert!(fecha1._es_menor(&fecha3)); // Mes es menor
    assert!(fecha1._es_menor(&fecha4)); // Año es menor
}

#[test]
fn test_es_menor_false() {
    let fecha1 = Fecha::new(1, 1, 2020);
    let fecha2 = Fecha::new(1, 1, 2020);
    let fecha3 = Fecha::new(31, 12, 2019);

    assert!(!fecha1._es_menor(&fecha2)); // Fechas iguales
    assert!(!fecha1._es_menor(&fecha3)); // Fecha es mayor
}
#[test]
fn test_cantidad_de_dias() {
    let fecha_bisiesta = Fecha::new(1, 1, 2020); // 2020 es un año bisiesto
    let fecha_no_bisiesta = Fecha::new(1, 1, 2021); // 2021 no es un año bisiesto

    // Meses con 31 días
    assert_eq!(fecha_bisiesta.cantidad_de_dias(1), 31);
    assert_eq!(fecha_bisiesta.cantidad_de_dias(3), 31);
    assert_eq!(fecha_bisiesta.cantidad_de_dias(5), 31);
    assert_eq!(fecha_bisiesta.cantidad_de_dias(7), 31);
    assert_eq!(fecha_bisiesta.cantidad_de_dias(8), 31);
    assert_eq!(fecha_bisiesta.cantidad_de_dias(10), 31);
    assert_eq!(fecha_bisiesta.cantidad_de_dias(12), 31);

    // Meses con 30 días
    assert_eq!(fecha_bisiesta.cantidad_de_dias(4), 30);
    assert_eq!(fecha_bisiesta.cantidad_de_dias(6), 30);
    assert_eq!(fecha_bisiesta.cantidad_de_dias(9), 30);
    assert_eq!(fecha_bisiesta.cantidad_de_dias(11), 30);
}
