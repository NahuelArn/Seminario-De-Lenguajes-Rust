/*
  Escribir un programa que defina una estructura Producto que tenga campos para el
nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
el precio bruto
➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
descuento sobre el precio bruto
➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
parámetros son opcionales.

*/
struct Producto {
    nombre: String,
    precio_bruto: f32,
    num_identificatorio: u32, //id
}

impl Producto {
    //➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
    fn new(nombre: String, precio_bruto: f32, num_identificatorio: u32) -> Producto {
        Producto {
            nombre,
            precio_bruto,
            num_identificatorio,
        }
    }
    //➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre el precio bruto
    fn calcular_impuestos(&self, porcentaje_de_impuestos: f32) -> f32 {
        let valor_impuestos: f32 = (self.precio_bruto * porcentaje_de_impuestos) / 100.0;
        valor_impuestos
    }
    //➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de descuento sobre el precio bruto
    fn aplicar_descuento(&self, porcentaje_de_descuento: f32) -> f32 {
        return (self.precio_bruto * porcentaje_de_descuento) / 100.0;
    }
    /*➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
    precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
    parámetros son opcionales. */
    fn calcular_precio_total(
        &self,
        procentaje_de_impuestos: Option<f32>,
        porcentaje_de_descuento: Option<f32>,
    ) -> f32 {
        let mut precio_total: f32 = self.precio_bruto;
        match procentaje_de_impuestos {
            Some(a) => precio_total += self.calcular_impuestos(a),
            None => (),
        }
        match porcentaje_de_descuento {
            Some(a) => precio_total -= self.aplicar_descuento(a),
            None => (),
        }
        // if let Some(a) = porcentaje_de_descuento {
        // 	precio_total -= self.aplicar_descuento(a);
        // }
        precio_total
    }
}
pub fn main() {}

mod impuestos {
    use super::*;
    #[test]
    fn impuesto_prueba1() {
        let p = Producto::new("pepe".to_string(), 1000.0, 1);
        assert_eq!(p.calcular_impuestos(15.0), 150.0);
    }
}

#[test]
fn test_calcular_impuestos() {
    let producto = Producto::new("Producto 1".to_string(), 100.0, 1);
    assert_eq!(producto.calcular_impuestos(10.0), 10.0);
}

#[test]
fn test_aplicar_descuento() {
    let producto = Producto::new("Producto 2".to_string(), 100.0, 2);
    assert_eq!(producto.aplicar_descuento(20.0), 20.0);
}

#[test]
fn test_calcular_precio_total_impuestos() {
    let producto = Producto::new("Producto 3".to_string(), 100.0, 3);
    assert_eq!(producto.calcular_precio_total(Some(10.0), None), 110.0);
}

#[test]
fn test_calcular_precio_total_descuento() {
    let producto = Producto::new("Producto 4".to_string(), 100.0, 4);
    assert_eq!(producto.calcular_precio_total(None, Some(20.0)), 80.0);
}

#[test]
fn test_calcular_precio_total_ambos() {
    let producto = Producto::new("Producto 5".to_string(), 100.0, 5);
    assert_eq!(producto.calcular_precio_total(Some(10.0), Some(20.0)), 90.0);
}

#[test]
fn test_calcular_precio_total_ninguno() {
    let producto = Producto::new("Producto 6".to_string(), 100.0, 6);
    assert_eq!(producto.calcular_precio_total(None, None), 100.0);
}
