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
use std::{option, usize};


struct Plataforma {
    usuarios: Vec<Usuario>, //actualmente en mis estructuras tendria users activos e inactivos
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
            //si no tengo ese medio de pago, lo agrego y queda en 0
            //hago una especie de vector contador, pero un HashMap seria mas escalable
            *self.medios_pago_contador.entry(usuario.medio_pago.clone()).or_insert(0) += 1;
            *self
                .suscripciones_contador
                .entry(usuario.suscripcion.tipo.clone())
                .or_insert(0) += 1;
            self.usuarios.push(usuario);
        }
    }
    fn upgrade (&mut self, mail: &String){
        if let Some(usuario) = self.retornar_usuario(mail){
            if usuario.suscripcion.tipo == TipoSuscripcion::Basic{
                usuario.suscripcion.tipo = TipoSuscripcion::Clasic;
            }else if usuario.suscripcion.tipo == TipoSuscripcion::Clasic{
                usuario.suscripcion.tipo = TipoSuscripcion::Super;
            }
        }        
    } 
    fn downgrade (&mut self, mail: &String){
        if let Some(usuario) = self.retornar_usuario(mail){
            match usuario.suscripcion.tipo {
                TipoSuscripcion::Basic => usuario.suscripcion.estado = false,
                TipoSuscripcion::Clasic => usuario.suscripcion.tipo = TipoSuscripcion::Basic,
                TipoSuscripcion::Super => usuario.suscripcion.tipo = TipoSuscripcion::Clasic,
            }
        }      
    }
    fn cancelar_suscripcion (&mut self, mail: &String){
        if let Some(usuario) = self.retornar_usuario(mail){
            usuario.suscripcion.estado = false;
        }        
    }
    fn medio_de_pago_mas_utilizado(&self)-> Option<&MedioDePago>{
        let mut mayor = 0;
        let mut medio_pago = None;
        for (medio, cantidad) in &self.medios_pago_contador{
            if *cantidad > mayor{
                mayor = *cantidad;
                medio_pago = Some(medio);
            }
        }
        medio_pago
    }
    fn medio_de_subscripcion_mas_utilizado(&self)-> Option<&TipoSuscripcion>{
        let mut mayor = 0;
        let mut suscripcion = None;
        for (tipo, cantidad) in &self.suscripciones_contador{
            if *cantidad > mayor{
                mayor = *cantidad;
                suscripcion = Some(tipo);
            }
        }
        suscripcion
    }
    fn medio_de_pago_mas_utilizado_activo(&self)-> Option<&MedioDePago>{
        let mut usize = 0;
        let mut medio_pago: Option<&MedioDePago> = None;
        for usuario in &self.usuarios{
            if usuario.suscripcion.estado{
                let cantidad = self.medios_pago_contador.get(&usuario.medio_pago).unwrap();
                if *cantidad > usize{
                    usize = *cantidad;
                    medio_pago = Some(&usuario.medio_pago);
                }
            }
        }
        medio_pago
    }
    fn suscripcion_mas_utilizada_activa(&self)-> Option<&TipoSuscripcion>{
        let mut usize = 0;
        let mut suscripcion: Option<&TipoSuscripcion> = None;
        for usuario in &self.usuarios{
            if usuario.suscripcion.estado{
                let cantidad = self.suscripciones_contador.get(&usuario.suscripcion.tipo).unwrap();
                if *cantidad > usize{
                    usize = *cantidad;
                    suscripcion = Some(&usuario.suscripcion.tipo);
                }
            }
        }
        suscripcion
    }
}

struct Usuario {
    mail: String, //control de unicidad, normalmente usamos el mail para las subscripciones
    suscripcion: Suscripcion,
    medio_pago: MedioDePago,
}
impl Usuario{
    fn new (mail: String, suscripcion: Suscripcion, medio_pago: MedioDePago) -> Usuario{
        Usuario{
            mail,
            suscripcion,
            medio_pago,
        }
    }
}
#[derive(Clone, PartialEq, Debug)]

struct Suscripcion {
    tipo: TipoSuscripcion,
    costo_mensual: f32,
    duracion_de_meses: u32,
    fecha_inicio: Fecha,
    estado: bool,
}

impl Suscripcion {
    fn new(
        tipo: TipoSuscripcion,
        costo_mensual: f32,
        duracion_de_meses: u32,
        fecha_inicio: Fecha,
    ) -> Suscripcion {
        Suscripcion {
            tipo,
            costo_mensual,
            duracion_de_meses,
            fecha_inicio,
            estado: true,
        }
    }

}
#[derive(Clone, Eq, Hash, Debug)]
enum MedioDePago {
    Efectivo(Efectivo),
    MercadoPago(MercadoPago),
    TarjetaCredito(TarjetaCredito),
    TransferenciaBancaria(TransferenciaBancaria),
    Cripto(Cripto),
}
impl MedioDePago{
    fn obtener_medio_pago(&self) -> String {
        match self {
            MedioDePago::MercadoPago => "MercadoPago".to_string(),
            MedioDePago::TarjetaCredito => "TarjetaCredito".to_string(),
            MedioDePago::TransferenciaBancaria => "TransferenciaBancaria".to_string(),
            MedioDePago::Cripto => "Cripto".to_string(),
        }
    }

}
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
struct TransferenciaBancaria {
    cbu: String,
    alias: String,
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

}


trait GestionPlataforma {
    fn buscar_usuario(&self, mail: &String) -> bool;
    fn retornar_usuario(&mut self, mail: &String) -> Option<&mut Usuario>;
}
impl GestionPlataforma for Plataforma {
    fn buscar_usuario(&self, mail: &String) -> bool {
        self.usuarios.iter().any(|usuario| usuario.mail == *mail)
    }
    fn retornar_usuario(&mut self, mail: &String) -> Option<&mut Usuario> {
        self.usuarios.iter_mut().find(|usuario| usuario.mail == *mail)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]

enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}
impl TipoSuscripcion{
    fn costo_mensual(&self) -> f32 {
        match self {
            TipoSuscripcion::Basic => 8200.0,
            TipoSuscripcion::Clasic => 10200.0,
            TipoSuscripcion::Super => 14000.0,
        }
    }
    fn cantidad_de_dispositivos(&self) -> u32 {
        match self {
            TipoSuscripcion::Basic => 1,
            TipoSuscripcion::Clasic => 2,
            TipoSuscripcion::Super => 4,
        }
    }
}

pub fn main() {}