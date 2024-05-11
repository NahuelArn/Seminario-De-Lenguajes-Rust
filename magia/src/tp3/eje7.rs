/*
- Defina una estructura llamada Concesionario Auto donde se conoce el nombre, la
dirección y tiene una capacidad máxima para albergar X cantidad de autos.
De los autos se conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser:rojo,
verde, azul, amarillo, blanco o negro.
Para dichas estructuras implemente los siguientes métodos:
❖ ConcesionarioAuto:
➢ new: que pasando los parámetros correspondientes, crea un
ConcesionarioAuto y lo retorna.
➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
no lo agrega y retorna false.
➢ eliminar_auto(auto): elimina un auto de la lista de autos.
➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
❖ Auto:
➢ new: que pasando los parámetros correspondientes, crea un Auto y lo
retorna.
➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
■ si es de color primario le aplica un recargo del 25%, sino le aplica un
descuento del 10%.
■ si la marca es BMW le aplica un recargo del 15%-
■ si el año es menor a 2000 le aplica un descuento del 5%.
*/

struct Concesionario_Auto {
    nombre: String,
    direccion: String,
    capacidad_max_de_autos: usize,
    capacidad_usada: usize,
    almacenamiento: Vec<Auto>, //Owner de Autos
}
impl Concesionario_Auto {
    fn new(
        //preguntar como cancelar el formateo o anhadir mas with de formato en esto
        nombre: String,
        direccion: String,
        capacidad_max_de_autos: usize,
    ) -> Concesionario_Auto {
        Concesionario_Auto {
            nombre,
            direccion,
            capacidad_max_de_autos,
            capacidad_usada: 0,
            almacenamiento: Vec::new(),
        }
    }
    fn agregar_auto(&mut self, auto: Auto) -> bool {
        let estado: bool;
        if self.capacidad_usada < self.capacidad_max_de_autos {
            self.capacidad_usada += 1;
            self.almacenamiento.push(auto);
            estado = true;
        } else {
            estado = false;
        }
        estado
    }
    //➢ eliminar_auto(auto): elimina un auto de la lista de autos.
    //no me pide que retorne un boolean.. pero por cuestiones del facilitarme el test, le mando bool
    fn eliminar_auto(&mut self) -> bool {
        let estado: bool;
        if self.capacidad_usada > 0 {
            self.almacenamiento.pop();
            self.capacidad_usada -= 1;
            estado = true;
        } else {
            estado = false;
        }
        estado
    }
    //lo que pide
    // fn eliminar_auto(&mut self, auto: Auto) {
    // 	if self.capacidad_usada > 0{
    // 		self.almacenamiento.pop();
    // 	}
    // }

    //➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
    fn buscar_auto(&self, auto: Auto) -> Option<Auto> {
        let mut i: usize = 0;
        let mut encontre: bool = false;
        while i < self.almacenamiento.len() && !encontre {
            if self.almacenamiento[i].eq(&auto) {
                //implemento el eq, de las del Enum y Auto, es como Java
                encontre = true;
            }
            i += 1;
        }
        if encontre {
            Some(auto)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone)]

enum Color {
    //TIPO MAtCH pero de opciones
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        // Comparo si matcheo con algun par de colores (los dos valores del enum)
        //para acceder a los campos en un Enum uso " :: " eje nombreEnum::atributo
        match (self, other) {
            //izq prop, der contra quien lo comparo
            (Color::Rojo, Color::Rojo)
            | (Color::Verde, Color::Verde)
            | (Color::Azul, Color::Azul)
            | (Color::Amarillo, Color::Amarillo)
            | (Color::Blanco, Color::Blanco)
            | (Color::Negro, Color::Negro) => true,
            _ => false,
        }
    }
}
impl PartialEq for Auto {
    fn eq(&self, other: &Self) -> bool {
        self.marca == other.marca
            && self.modelo == other.modelo
            && self.anho == other.anho
            && self.precio_bruto == other.precio_bruto
            && self.color == other.color
    }
}
#[derive(Debug)]

struct Auto {
    marca: String,
    modelo: String,
    anho: u32,
    precio_bruto: f32,
    color: Color,
}
impl Clone for Auto {
    fn clone(&self) -> Self {
        Auto {
            marca: self.marca.clone(),
            modelo: self.modelo.clone(),
            anho: self.anho,
            precio_bruto: self.precio_bruto,
            color: self.color.clone(),
        }
    }
}
// ➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
// ■ si es de color primario le aplica un recargo del 25%, sino le aplica un
// descuento del 10%.
// ■ si la marca es BMW le aplica un recargo del 15%-
// ■ si el año es menor a 2000 le aplica un descuento del 5%.
impl Auto {
    fn new(marca: String, modelo: String, anho: u32, precio_bruto: f32, color: Color) -> Auto {
        Auto {
            marca,
            modelo,
            anho,
            precio_bruto,
            color,
        }
    }
    //color primario: Rojo, Amarillo, Azul... son los q se no pueden obtener mezclando otros colores
    fn calcular_precio(&self) -> f32 {
        let mut total: f32 = match self.color {
            Color::Rojo | Color::Verde | Color::Azul => self.precio_bruto * 1.25,
            _ => self.precio_bruto - ((self.precio_bruto * 10.0) / 100.0), // Descuento del 10% //self.precio_bruto*0.90
        };
        if self.marca == "BMW".to_string() {
            total *= 1.15;
        }
        if self.anho < 2000 {
            total *= 1.05;
        }
        total
    }

    pub(crate) fn saludo(anho: i32) {
        println!("Hola ");
    }
}

pub fn main() {
    // let auto1 = Auto::new(
    //     "Toyota".to_string(),
    //     "Corolla".to_string(),
    //     2015,
    //     20000.0,
    //     Color::Azul,
    // );
    // let auto2 = Auto::new(
    //     "BMW".to_string(),
    //     "X5".to_string(),
    //     2020,
    //     50000.0,
    //     Color::Blanco,
    // );
    // let auto3 = Auto::new(
    //     "Ford".to_string(),
    //     "Fiesta".to_string(),
    //     1998,
    //     5000.0,
    //     Color::Rojo,
    // );
    // let mut concesionario = Concesionario_Auto::new(
    //     "Concesionario XYZ".to_string(),
    //     "Calle Principal, 123".to_string(),
    //     10,
    // );
}

mod concesionario_agregar_tests {
    use super::*;

    #[test]
    fn test_agregar_auto_exitoso() {
        let mut concesionario = Concesionario_Auto::new(
            "Concesionario XYZ".to_string(),
            "Calle Principal, 123".to_string(),
            2,
        );

        let auto1 = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2015,
            20000.0,
            Color::Azul,
        );

        assert_eq!(concesionario.agregar_auto(auto1), true);
        assert_eq!(concesionario.capacidad_usada, 1);
        assert_eq!(concesionario.almacenamiento.len(), 1);

        let auto2 = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::Blanco,
        );

        assert_eq!(concesionario.agregar_auto(auto2), true);
        assert_eq!(concesionario.capacidad_usada, 2);
        assert_eq!(concesionario.almacenamiento.len(), 2);
    }

    #[test]
    fn test_agregar_auto_fallido() {
        let mut concesionario = Concesionario_Auto::new(
            "Concesionario XYZ".to_string(),
            "Calle Principal, 123".to_string(),
            1,
        );

        let auto1 = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2015,
            20000.0,
            Color::Azul,
        );

        let auto2 = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::Blanco,
        );

        assert_eq!(concesionario.agregar_auto(auto1), true);
        assert_eq!(concesionario.agregar_auto(auto2), false);
        assert_eq!(concesionario.capacidad_usada, 1);
        assert_eq!(concesionario.almacenamiento.len(), 1);
    }
}

mod concesionario_eliminar_tests {
    use super::*;

    #[test]
    fn test_eliminar_auto_exitoso() {
        let mut concesionario = Concesionario_Auto::new(
            "Concesionario XYZ".to_string(),
            "Calle Principal, 123".to_string(),
            2,
        );

        let auto1 = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2015,
            20000.0,
            Color::Azul,
        );

        let auto2 = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::Blanco,
        );

        concesionario.agregar_auto(auto1);
        concesionario.agregar_auto(auto2);

        assert_eq!(concesionario.eliminar_auto(), true);
        assert_eq!(concesionario.capacidad_usada, 1);
        assert_eq!(concesionario.almacenamiento.len(), 1);
    }

    #[test]
    fn test_eliminar_auto_fallido() {
        let mut concesionario = Concesionario_Auto::new(
            "Concesionario XYZ".to_string(),
            "Calle Principal, 123".to_string(),
            2,
        );

        let auto1 = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2015,
            20000.0,
            Color::Azul,
        );

        assert_eq!(concesionario.eliminar_auto(), false);
        assert_eq!(concesionario.capacidad_usada, 0);
        assert_eq!(concesionario.almacenamiento.len(), 0);
    }
}

mod concesionario_busqueda_tests {
    use super::*;
    #[test]
    fn test_buscar_auto_exitoso() {
        let mut concesionario = Concesionario_Auto::new(
            "Concesionario XYZ".to_string(),
            "Calle Principal, 123".to_string(),
            2,
        );

        let auto1 = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2015,
            20000.0,
            Color::Azul,
        );

        let auto2 = Auto::new(
            "BMW".to_string(),
            "X5".to_string(),
            2020,
            50000.0,
            Color::Blanco,
        );

        concesionario.agregar_auto(auto1.clone()); // Clonar auto1 para evitar el movimiento
        concesionario.agregar_auto(auto2);

        assert_eq!(concesionario.buscar_auto(auto1.clone()), Some(auto1));
    }

    #[test]
    fn test_buscar_auto_fallido() {
        let mut concesionario = Concesionario_Auto::new(
            "Concesionario XYZ".to_string(),
            "Calle Principal, 123".to_string(),
            2,
        );

        let auto1 = Auto::new(
            "Toyota".to_string(),
            "Corolla".to_string(),
            2015,
            20000.0,
            Color::Azul,
        );

        assert_eq!(concesionario.buscar_auto(auto1), None);
    }
}
