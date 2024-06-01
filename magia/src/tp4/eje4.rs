/*
4 -Se requiere implementar un sistema de ventas de productos.
De cada producto se
  conoce el nombre, una categoría y un precio base, y algunos productos pueden tener
  descuentos aplicables dependiendo de la categoría.

  Además, se debe registrar al vendedor
  que realizó la venta y al cliente. De ellos se conoce nombre, apellido, dirección, dni y del
  vendedor nro de legajo, antigüedad y salario.

  Los clientes pueden tener un beneficio de
  descuento si tienen suscripción al newsletter, de ser así se tiene el correo electrónico del
  mismo.
  El sistema debe permitir registrar las ventas realizadas y asociar el medio de pago utilizado.
  Los medios de pago aceptados son: tarjeta de crédito, tarjeta de débito, transferencia
  bancaria y efectivo.
  Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
  siguientes acciones:
  ➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de
  productos.
  ➢ Calcular el precio final de una venta en base a los productos que hay en ella. Para
  calcularlo tenga en cuenta que pueden haber determinados productos de alguna
  categoría donde debería aplicarse un descuento. Tanto la categoría como el
  porcentaje de descuento a aplicar son datos que le brinda el sistema. Es decir el
  sistema tiene una lista de las categorías con el descuento a aplicar. Además se debe
  aplicar un porcentaje de descuento general si el cliente tiene suscripción al
  newsletter.
  ➢ Para llevar un control de las ventas realizadas, se debe implementar un reporte que
  permita visualizar las ventas totales por categoría de producto y otro por vendedor.
*/

use core::hash;
use std::{collections::HashMap, hash::Hash};
struct SistemaVentas {
    ventas: Vec<Venta>,
    descuento_por_categoria: HashMap<String, f32>,	//lo pense como enum pero no es escalable y estaria muy limitado
    ventas_totales_por_categoria: HashMap<String, usize>,	//categoria del producto el String
    ventas_totales_por_vendedor: HashMap<u32, usize>, //legajo del vendedor
}

impl SistemaVentas {
    fn new() -> SistemaVentas {
        SistemaVentas {
            ventas: Vec::new(),
			descuento_por_categoria: HashMap::new(),
            ventas_totales_por_categoria: HashMap::new(),
            ventas_totales_por_vendedor: HashMap::new(),
        }
    }
    fn crear_venta(&mut self, venta: Venta) {
		// cargo las ventas totales por categoria
		let legajo_actual = venta.vendedor.legajo;
		*self.ventas_totales_por_vendedor.entry(legajo_actual).or_insert(0) += 1;
		// cargo las ventas totales por vendedor
		for producto in venta.productos.iter() {
			*self.ventas_totales_por_categoria.entry(producto.categoria.clone()).or_insert(0) += 1;
		}
        self.ventas.push(venta);
    }
	fn agregar_descuento_categoria(&mut self, categoria: String, descuento: f32) {
		self.descuento_por_categoria.insert(categoria, descuento);
	}
	fn quitar_descuento_categoria(&mut self, categoria: &String) {
		self.descuento_por_categoria.remove(categoria);
	}
	fn precio_final_venta(&self, venta: &Venta) -> f32 {
		let mut precio_final = 0.0;
		for producto in venta.productos.iter() {
			let mut descuento = 0.0;
			if let Some(desc) = self.descuento_por_categoria.get(&producto.categoria) {
				descuento = *desc;
			}
			precio_final += producto.get_precio() * (1.0 - descuento);
		}
		if venta.es_subscriptor {
			precio_final *= 0.9; //10% de descuento generoso el lugar
		}
		precio_final
	}
    /*
    ➢ Para llevar un control de las ventas realizadas, se debe implementar un reporte que
    permita visualizar las ventas totales por categoría de producto y otro por vendedor.
    */
    fn reporte_ventas_totales(&self) {
		println!("Ventas totales por categoria de producto:");
		for (categoria, cantidad) in self.ventas_totales_por_categoria.iter() {
			println!("Categoria: {} - Cantidad: {}", categoria, cantidad);
		}
		println!("Ventas totales por vendedor:");
		for (legajo, cantidad) in self.ventas_totales_por_vendedor.iter() {
			println!("Legajo: {} - Cantidad: {}", legajo, cantidad);
		}
	
    }
}
// enum CategoriasConDescuento {
//     // tiene sentido usar un enum? yyy escalable no es
//     //pero nose si da para usar una estructura dinamica tambien(tantas categorias vas a tener?). Un hashmap con la clave como categoria? podria ser, pero posible problema de hash flotante
//     Carnes,
//     Verduras,
//     ProductosDeLimpieza,
//     ProductosDeBelleza,
// }
// impl CategoriasConDescuento {
//     fn descuento(&self, categoria: &String) -> Option<f32> {
// 		match self {
// 			CategoriasConDescuento::Carnes => {
// 				if categoria == "Carnes" {
// 					Some(0.1)
// 				} else {
// 					None
// 				}
// 			}
// 			CategoriasConDescuento::Verduras => {
// 				if categoria == "Verduras" {
// 					Some(0.2)
// 				} else {
// 					None
// 				}
// 			}
// 			CategoriasConDescuento::ProductosDeLimpieza => {
// 				if categoria == "ProductosDeLimpieza" {
// 					Some(0.3)
// 				} else {
// 					None
// 				}
// 			}
// 			CategoriasConDescuento::ProductosDeBelleza => {
// 				if categoria == "ProductosDeBelleza" {
// 					Some(0.4)
// 				} else {
// 					None
// 				}
// 			}
// 		}
//     }
// }
#[derive(PartialEq, Debug, Clone)]
struct Venta {
    fecha: Fecha,
    cliente: Cliente,
    vendedor: Vendedor,
    medio_pago: MedioPago,
    productos: Vec<Producto>,
    es_subscriptor: bool,
}

impl Venta {
    fn new(
        fecha: Fecha,
        cliente: Cliente,
        vendedor: Vendedor,
        medio_pago: MedioPago,
        productos: Vec<Producto>,
        es_subscriptor: bool,
    ) -> Venta {
        Venta {
            fecha: fecha,
            cliente: cliente,
            vendedor: vendedor,
            medio_pago: medio_pago,
            productos: productos,
            es_subscriptor: es_subscriptor,
        }
    }
}
#[derive(PartialEq, Debug, Clone, Copy)]
struct Fecha {
    dia: u32,
    mes: u32,
    anio: u32,
}
#[derive(PartialEq, Debug, Clone)]
struct Persona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u32,
}
#[derive(PartialEq, Debug, Clone)]
struct Cliente {
    persona: Persona,
}
#[derive(PartialEq, Debug, Clone)]
struct Vendedor {
    persona: Persona,
    legajo: u32,
    antiguedad: u32,
    salario: f32,
}
#[derive(PartialEq, Debug, Clone)]
enum MedioPago {
    TarjetaDeCredito(TarjetaCredito),
    MercadoPago(MercadoPago),
    TransferenciaBancaria(TransferenciaBancaria),
    Efectivo,
    Cripto(Cripto), //Q buen lugar
}

#[derive(PartialEq, Debug, Clone)]
struct TarjetaCredito {
    numero: String,
    vencimiento: Fecha,
    titular: String,
    cod_seguridad: u8,
}

#[derive(PartialEq, Debug, Clone)]
struct MercadoPago {
    email: String,
    alias: String,
}
#[derive(PartialEq, Debug, Clone)]
struct TransferenciaBancaria {
    cbu: String,
    alias: String,
}
#[derive(PartialEq, Debug, Clone)]
struct Cripto {
    addres: String,
    red: String,
}
#[derive(PartialEq, Debug, Clone)]
struct Producto {
    nombre: String,
    categoria: String,
    precio: f32,
}
impl Producto {
	fn new(nombre: String, categoria: String, precio: f32) -> Producto {
		Producto {
			nombre: nombre,
			categoria: categoria,
			precio: precio,
		}
	}
	fn get_precio(&self) -> f32 {
		self.precio
	}
}
pub fn main() {}


#[test]
fn test_crear_venta() {
	let fecha = Fecha {
		dia: 1,
		mes: 1,
		anio: 2021,
	};
	let cliente = Cliente {
		persona: Persona {
			nombre: "Juan".to_string(),
			apellido: "Perez".to_string(),
			direccion: "Av. Siempre Viva 123".to_string(),
			dni: 12345678,
		},
	};
	let vendedor = Vendedor {
		persona: Persona {
			nombre: "Pedro".to_string(),
			apellido: "Gomez".to_string(),
			direccion: "Calle Falsa 123".to_string(),
			dni: 87654321,
		},
		legajo: 123,
		antiguedad: 5,
		salario: 50000.0,
	};
	let medio_pago = MedioPago::Efectivo;
	let productos = vec![
		Producto::new("Carne".to_string(), "Carnes".to_string(), 100.0),
		Producto::new("Tomate".to_string(), "Verduras".to_string(), 50.0),
	];
	let mut venta = Venta::new(fecha.clone(), cliente.clone(), vendedor.clone(), medio_pago.clone(), productos.clone(), false);
	
	assert_eq!(venta.fecha, fecha);
	assert_eq!(venta.cliente, cliente);
	assert_eq!(venta.vendedor, vendedor);
	assert_eq!(venta.medio_pago, medio_pago);
	assert_eq!(venta.productos, productos);
	assert_eq!(venta.es_subscriptor, false);
}

#[test]
fn test_precio_final_venta() {	
	let fecha = Fecha {
		dia: 1,
		mes: 1,
		anio: 2021,
	};
	let cliente = Cliente {
		persona: Persona {
			nombre: "Juan".to_string(),
			apellido: "Perez".to_string(),
			direccion: "direccion".to_string(),
			dni: 12345678,
		},
	};
	let vendedor = Vendedor {
		persona: Persona {
			nombre: "Pedro".to_string(),
			apellido: "Gomez".to_string(),
			direccion: "direccion".to_string(),
			dni: 87654321,
		},
		legajo: 123,
		antiguedad: 5,
		salario: 50000.0,
	};
	let medio_pago = MedioPago::Efectivo;
	let productos = vec![
		Producto::new("Carne".to_string(), "Carnes".to_string(), 100.0),
		Producto::new("Tomate".to_string(), "Verduras".to_string(), 50.0),
	];
	let venta = Venta::new(fecha.clone(), cliente.clone(), vendedor.clone(), medio_pago.clone(), productos.clone(), false);
	
	let mut sistema = SistemaVentas::new();
	sistema.agregar_descuento_categoria("Carnes".to_string(), 0.1);
	sistema.agregar_descuento_categoria("Verduras".to_string(), 0.2);
	
	sistema.crear_venta(venta.clone());
	
	assert_eq!(sistema.precio_final_venta(&venta), 130.0);
}

#[test]
fn	test_reporte_ventas_totales() {
	let fecha = Fecha {
		dia: 1,
		mes: 1,
		anio: 2021,
	};
	let cliente = Cliente {
		persona: Persona {
			nombre: "Juan".to_string(),
			apellido: "Perez".to_string(),
			direccion: "direccion".to_string(),
			dni: 12345678,
		},
	};
	let vendedor = Vendedor {
		persona: Persona {
			nombre: "Pedro".to_string(),
			apellido: "Gomez".to_string(),
			direccion: "direccion".to_string(),
			dni: 87654321,
		},
		legajo: 123,
		antiguedad: 5,
		salario: 50000.0,
	};
	let medio_pago = MedioPago::Efectivo;
	let productos = vec![
		Producto::new("Carne".to_string(), "Carnes".to_string(), 100.0),
		Producto::new("Tomate".to_string(), "Verduras".to_string(), 50.0),
	];
	let mut venta = Venta::new(fecha.clone(), cliente.clone(), vendedor.clone(), medio_pago.clone(), productos.clone(), false);
	
	let mut sistema = SistemaVentas::new();
	sistema.agregar_descuento_categoria("Carnes".to_string(), 0.1);
	sistema.agregar_descuento_categoria("Verduras".to_string(), 0.2);
	
	sistema.crear_venta(venta.clone());
	sistema.reporte_ventas_totales();
	
	assert_eq!(sistema.ventas_totales_por_categoria.get("Carnes"), Some(&1));
	assert_eq!(sistema.ventas_totales_por_categoria.get("Verduras"), Some(&1));
	assert_eq!(sistema.ventas_totales_por_vendedor.get(&123), Some(&1));
}

#[test]
fn test_quitar_descuento_categoria() {
	let mut sistema = SistemaVentas::new();
	sistema.agregar_descuento_categoria("Carnes".to_string(), 0.1);
	sistema.agregar_descuento_categoria("Verduras".to_string(), 0.2);
	
	sistema.quitar_descuento_categoria(&"Carnes".to_string());
	
	assert_eq!(sistema.descuento_por_categoria.get("Carnes"), None);
	assert_eq!(sistema.descuento_por_categoria.get("Verduras"), Some(&0.2));
}

#[test]
fn test_precio_final_venta_con_descuento_categoria() {
	let fecha = Fecha {
		dia: 1,
		mes: 1,
		anio: 2021,
	};
	let cliente = Cliente {
		persona: Persona {
			nombre: "Juan".to_string(),
			apellido: "Perez".to_string(),
			direccion: "direccion".to_string(),
			dni: 12345678,
		},
	};
	let vendedor = Vendedor {
		persona: Persona {
			nombre: "Pedro".to_string(),
			apellido: "Gomez".to_string(),
			direccion: "direccion".to_string(),
			dni: 87654321,
		},
		legajo: 123,
		antiguedad: 5,
		salario: 50000.0,
	};
	let medio_pago = MedioPago::Efectivo;
	let productos = vec![
		Producto::new("Carne".to_string(), "Carnes".to_string(), 100.0),
		Producto::new("Tomate".to_string(), "Verduras".to_string(), 50.0),
	];
	let venta = Venta::new(fecha.clone(), cliente.clone(), vendedor.clone(), medio_pago.clone(), productos.clone(), false);
	
	let mut sistema = SistemaVentas::new();
	sistema.agregar_descuento_categoria("Carnes".to_string(), 0.1);
	sistema.agregar_descuento_categoria("Verduras".to_string(), 0.2);
	
	sistema.crear_venta(venta.clone());
	
	assert_eq!(sistema.precio_final_venta(&venta), 130.0);
}