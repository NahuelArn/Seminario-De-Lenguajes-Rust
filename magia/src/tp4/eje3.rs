/*
3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
(Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una
duración de meses y una fecha de inicio,

además los usuarios pueden pagar por sus
suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
Crédito, Transferencia Bancaria, Cripto.

Cada medio de pago tiene sus datos
correspondientes a excepción de Efectivo.
Los usuarios solo pueden tener una suscripción activa a la vez.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:

➢ Crear un usuario con una determinada suscripción y medio de pago.
➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
pasa a Clasic y si está en Clasic pasa a Super.
➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
➢ Dado un usuario cancelar la suscripción.
➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
suscripciones activas
➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
activas.
➢ Saber cuál fue el medio de pago más utilizado.
➢ Saber cuál fue la suscripción más contratada.
*/

use std::collections::HashMap;
use std::hash::Hash;

struct Plataforma {
    usuarios: Vec<Usuario>,
    medios_pago_contador: HashMap<MedioDePago, usize>,
    suscripciones_contador: HashMap<TipoSuscripcion, usize>,
    // medios_pago_contador: Vec<u32>, //para saber cual es el mas utilizado
    // suscripciones_contador: Vec<u32>,
}
impl Plataforma {
    fn new() -> Plataforma {
        Plataforma {
            usuarios: Vec::new(),
            medios_pago_contador: HashMap::new(),
            suscripciones_contador: HashMap::new(),
        }
    }

    fn crear_usuario(&mut self, usuario: Usuario) {
        if !self.buscar_usuario(&usuario.mail) {
            *self
                .medios_pago_contador
                .entry(usuario.medio_pago.clone())
                .or_insert(0) += 1;
            *self
                .suscripciones_contador
                .entry(usuario.suscripcion.tipo.clone())
                .or_insert(0) += 1;
            self.usuarios.push(usuario);
        }
    }
}

struct Usuario {
    mail: String, //control de unicidad
    suscripcion: Suscripcion,
    medio_pago: MedioDePago,
}
#[derive(Clone, PartialEq, Debug)]

struct Suscripcion {
    tipo: TipoSuscripcion,
    costo_mensual: f32,
    duracion_de_meses: u32,
    fecha_inicio: Fecha,
    estado: bool,
}
#[derive(Clone, Copy, Eq, Hash, Debug)]

enum MedioDePago {
    MercadoPago,
    TarjetaCredito,
    TransferenciaBancaria,
    Cripto,
}
impl PartialEq for MedioDePago {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MedioDePago::MercadoPago, MedioDePago::MercadoPago) => true,
            (MedioDePago::TarjetaCredito, MedioDePago::TarjetaCredito) => true,
            (MedioDePago::TransferenciaBancaria, MedioDePago::TransferenciaBancaria) => true,
            (MedioDePago::Cripto, MedioDePago::Cripto) => true,
            _ => false,
        }
    }
}
#[derive(Clone, PartialEq, Hash, Debug)]

struct Fecha {
    //reutilizar mi clase fecha  ya la tenia inplementada
    dia: u32,
    mes: u32,
    anio: u32,
}

// suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
// Crédito, Transferencia Bancaria, Cripto.
// trait MedioDePago {
//     fn obtener_medio_pago(&self) -> String;
//     fn obtener_datos(&self) -> String;
// }

// impl MedioDePago for Efectivo {
//     fn obtener_medio_pago(&self) -> String {
//         "Efectivo".to_string()
//     }
//     fn obtener_datos(&self) -> String {
//         format!("Monto: {}", self.monto)
//     }
// }
struct Efectivo {
    monto: f32,
}

struct MercadoPago {
    email: String,
    alias: String,
}
struct TarjetaCredito {
    numero: String,
    vencimiento: Fecha,
    titular: String,
    cod_seguridad: u8,
}
// Cada medio de pago tiene sus datos
// correspondientes a excepción de Efectivo.
// trait MedioDePago{
//     fn obtener_medio_pago(&self) -> String;
//     fn obtener_datos(&self) -> String;

// }

trait GestionPlataforma {
    fn buscar_usuario(&self, mail: &String) -> bool;
}
impl GestionPlataforma for Plataforma {
    fn buscar_usuario(&self, mail: &String) -> bool {
        self.usuarios.iter().any(|usuario| usuario.mail == *mail)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]

enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}


pub fn main() {}
