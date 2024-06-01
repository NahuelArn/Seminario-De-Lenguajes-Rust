/*
5- La empresa XYZ es una plataforma de intercambio de criptoactivos que permite a los
  usuarios comprar y vender distintas criptomonedas.

  La plataforma permite el registro de
  usuarios y la gestión de sus balances en distintas criptomonedas y en dinero fíat.

  De los usuarios se conoce nombre, apellido, email, dni, y si está validada su identidad o no. Cada
  usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma.

  De las criptomonedas se conoce: nombre, prefijo y un listado de blockchains donde se pueden
  enviar o recibir. De cada blockchain se conoce el nombre, prefijo.

  Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
  siguientes acciones relacionadas al usuario:

  ➢ Ingresar dinero: se recibe un monto en fiat de un usuario y se acredita al balance de
  fiat de dicho usuario. Además se crea una transacción del hecho donde los datos
  que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.

  ➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad
  de determinada criptomoneda, tenga en cuenta que al momento de realizar la
  operación se obtiene del sistema la cotización actual de la criptomoneda para
  acreditar la correspondiente proporción en el balance de la cripto y desacreditar en
  el balance de fiat. Luego de ello se registra la transacción con los siguientes datos:
  fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.

  ➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat, tenga
  en cuenta que al momento de realizar la operación se obtiene del sistema la
  cotización actual de la criptomoneda para acreditar la correspondiente proporción en
  el balance de fiat y desacreditar en el balance de la criptomoneda. Luego de ello se
  registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo:
  venta de cripto,monto de cripto y cotización.

  ➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se
  le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve
  un hash que representa una transacción en ella (esto hágalo retornando el nombre
  de la blockchain + un número random). Luego se genera una transacción con los
  siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto,
  cotización

  ➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain
  se le acredita al balance de dicha cripto al usuario el monto. Luego se genera una
  transacción con los siguientes datos: fecha, usuario, tipo: recepción cripto,
  blockchain, cripto, monto, cotización.

  ➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho
  monto del balance al usuario y se genera una transacción con la siguiente
  información: fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago
  o Transferencia Bancaria)

  Nota:: Tanto para comprar. vender, retirar el usuario debe estar validado.
  Se debe validar siempre que haya balance suficiente para realizar la operación en los casos
  de compra, venta, retiro.

  Además la empresa desea saber lo siguiente en base a sus operaciones:
  ➢ Saber cual es la criptomoneda que más cantidad de ventas tiene
  ➢ Saber cual es la criptomoneda que más cantidad de compras tiene
  ➢ Saber cual es la criptomoneda que más volumen de ventas tiene
  ➢ Saber cual es la criptomoneda que más volumen de compras tiene
*/
use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

struct Plataforma {
    nombre: String,
    usuarios: HashMap<String, Usuario>, //adress de usuario, usuario
    transacciones: HashMap<String, TransaccionPlataforma>, //Hash de la transaccion, transaccion
    // transacciones: Vec<Transaccion>,
    balance: Vec<Criptomoneda>,
}
/*
 ➢ Ingresar dinero: se recibe un monto en fiat de un usuario y se acredita al balance de
  fiat de dicho usuario. Además se crea una transacción del hecho donde los datos
  que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.
*/
impl Plataforma {
    fn ingresar_dinero(&mut self, monto: f32, usuario: Usuario) {
        if let Some(u) = self.usuarios.get_mut(&usuario.adress) {
            u.fiat += monto;
            let ahora = Utc::now();
            let fecha = Fecha {
                dia: ahora.day(),
                mes: ahora.month(),
                anio: ahora.year() as u32,
            };
            // Generar el hash para la transacción
            let hash = generar_hash(&u.adress, monto, &fecha);
            let transaccion = TransaccionPlataforma {
                fecha,
                tipo: "ingreso de dinero".to_string(),
                monto,
                hash: hash.clone(),
                medioDeRetiro: MedioRetiro::None,
                usuario: usuario.clone(),
                criptomoneda: None,
            };
            self.transacciones.insert(hash, transaccion);
        }
    }
    /*
     ➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad
       de determinada criptomoneda, tenga en cuenta que al momento de realizar la
       operación se obtiene del sistema la cotización actual de la criptomoneda para
       acreditar la correspondiente proporción en el balance de la cripto y desacreditar en
       el balance de fiat. Luego de ello se registra la transacción con los siguientes datos:
       fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.
    */
    fn comprar_criptomoneda(
        &mut self,
        monto: f32,
        usuario: &mut Usuario,
        criptomoneda: &Criptomoneda,
    ) {
        if let Some(u) = self.usuarios.get_mut(&usuario.adress) {
            if u.validado {
                if u.fiat >= monto {
                    let cotizacion = criptomoneda.cotizacion(); //se podria usando una api pero al momento del test va ser durisimo
                    let cantidad = monto / cotizacion;
                    u.fiat -= monto;
                    let index = u
                        .balance
                        .iter()
                        .position(|x| x.prefijo == criptomoneda.prefijo);
                    if let Some(i) = index {
                        u.balance[i].monto += cantidad;
                    } else {
                        u.balance.push(Criptomoneda {
                            nombre: criptomoneda.nombre.clone(),
                            prefijo: criptomoneda.prefijo.clone(),
                            monto: cantidad,
                            listado_de_blockchain: vec![],
                        });
                        let tamanho = u.balance.len();
                        u.balance[tamanho - 1].monto = cantidad;
                    }
                    let ahora = Utc::now();
                    let fecha = Fecha {
                        dia: ahora.day(),
                        mes: ahora.month(),
                        anio: ahora.year() as u32,
                    };
                    let hash = generar_hash(&u.adress, monto, &fecha);
                    let transsaccion = TransaccionPlataforma {
                        fecha,
                        tipo: "compra de cripto".to_string(),
                        monto: cantidad,
                        hash: hash.clone(),
                        medioDeRetiro: MedioRetiro::None,
                        usuario: usuario.clone(),
                        criptomoneda: Some(criptomoneda.clone()),
                    };
                    self.transacciones.insert(hash, transsaccion);
                }
            }
        }
    }
    /*
     ➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat, tenga
       en cuenta que al momento de realizar la operación se obtiene del sistema la
       cotización actual de la criptomoneda para acreditar la correspondiente proporción en
       el balance de fiat y desacreditar en el balance de la criptomoneda. Luego de ello se
       registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo:
       venta de cripto,monto de cripto y cotización.
    */
    fn vender_criptomoneda(
        &mut self,
        monto: f32,
        usuario: &mut Usuario,
        criptomoneda: &Criptomoneda,
    ) {
        if let Some(u) = &mut self.usuarios.get_mut(&usuario.adress) {
            if u.validado {
                let index = u
                    .balance
                    .iter()
                    .position(|x| x.prefijo == criptomoneda.prefijo);
                if let Some(i) = index {
                    if u.balance[i].monto >= monto {
                        let cotizacion = criptomoneda.cotizacion();
                        let cantidad = monto * cotizacion;
                        u.balance[i].monto -= monto;
                        u.fiat += cantidad;
                        let ahora = Utc::now();
                        let fecha = Fecha {
                            dia: ahora.day(),
                            mes: ahora.month(),
                            anio: ahora.year() as u32,
                        };
                        let hash = generar_hash(&u.adress, monto, &fecha);
                        let transsaccion = TransaccionPlataforma {
                            fecha,
                            tipo: "venta de cripto".to_string(),
                            monto: cantidad,
                            hash: hash.clone(),
                            medioDeRetiro: MedioRetiro::None,
                            usuario: usuario.clone(),
                            criptomoneda: Some(criptomoneda.clone()),
                        };
                        self.transacciones.insert(hash, transsaccion);
                    }
                }
            }
        }
    }
    /*
    ➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se
        le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve
        un hash que representa una transacción en ella (esto hágalo retornando el nombre
        de la blockchain + un número random). Luego se genera una transacción con los
        siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto,
        cotización
     */
    //llevo 2 transacciones, una en la blockchain y otra en la plataforma
    //el hasheo es la misma, cosa de poder validad la transaccion si quiero
    fn retirar_criptomoneda(
        &mut self,
        monto: f32,
        usuario: &mut Usuario,
        criptomoneda: Criptomoneda,
        blockchain: &mut Blockchain,
    ) {
        if let Some(u) = self.usuarios.get_mut(&usuario.adress) {
            let index = u
                .balance
                .iter()
                .position(|x| x.prefijo == criptomoneda.prefijo);
            if let Some(i) = index {
                if u.balance[i].monto >= monto {
                    u.balance[i].monto -= monto;
                    let ahora = Utc::now();
                    let fecha = Fecha {
                        dia: ahora.day(),
                        mes: ahora.month(),
                        anio: ahora.year() as u32,
                    };
                    let hash = generar_hash(&u.adress, monto, &fecha);
                    let transaccion = TransaccionBlockchain {
                        fecha: fecha.clone(),
                        tipo: "retiro cripto".to_string(),
                        monto,
                        blockchain: blockchain.clone(),
                        hash: hash.clone(),
                        criptomoneda: criptomoneda.clone(),
                        cotizacion: criptomoneda.cotizacion(),
                    };
                    blockchain.transacciones.insert(hash.clone(), transaccion);
                    let transaccion = TransaccionPlataforma {
                        fecha,
                        tipo: "retiro cripto".to_string(),
                        monto,
                        hash: hash.clone(),
                        medioDeRetiro: MedioRetiro::None,
                        usuario: usuario.clone(),
                        criptomoneda: Some(criptomoneda.clone()),
                    };
                    self.transacciones.insert(hash, transaccion);
                }
            }
        }
    }
    /*
    ➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain
        se le acredita al balance de dicha cripto al usuario el monto. Luego se genera una
        transacción con los siguientes datos: fecha, usuario, tipo: recepción cripto,
        blockchain, cripto, monto, cotización.
     */
    fn recibir_criptomoneda(
        &mut self,
        monto: f32,
        usuario: &mut Usuario,
        criptomoneda: Criptomoneda,
        blockchain: &mut Blockchain,
    ) {
        if let Some(u) = self.usuarios.get_mut(&usuario.adress) {
            let index = u
                .balance
                .iter()
                .position(|x| x.prefijo == criptomoneda.prefijo);
            if let Some(i) = index {
                u.balance[i].monto += monto;
                let ahora = Utc::now();
                let fecha = Fecha {
                    dia: ahora.day(),
                    mes: ahora.month(),
                    anio: ahora.year() as u32,
                };
                let hash = generar_hash(&u.adress, monto, &fecha);
                let transaccion = TransaccionBlockchain {
                    fecha: fecha.clone(),
                    tipo: "recepción cripto".to_string(),
                    monto,
                    blockchain: blockchain.clone(),
                    hash: hash.clone(),
                    criptomoneda: criptomoneda.clone(),
                    cotizacion: criptomoneda.cotizacion(),
                };
                blockchain.transacciones.insert(hash.clone(), transaccion);
                let transaccion: TransaccionPlataforma = TransaccionPlataforma {
                    fecha,
                    tipo: "recepción cripto".to_string(),
                    monto,
                    hash: hash.clone(),
                    medioDeRetiro: MedioRetiro::None,
                    usuario: usuario.clone(),
                    criptomoneda: Some(criptomoneda.clone()),
                };
                self.transacciones.insert(hash, transaccion);
            }
        }
    }
    /*
     ➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho
       monto del balance al usuario y se genera una transacción con la siguiente
       información: fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago
       o Transferencia Bancaria)
    */
    fn retirar_fiat(&mut self, usuario: &Usuario, monto: f32, medio: MedioRetiro) {
        if let Some(u) = self.usuarios.get_mut(&usuario.adress) {
            if u.fiat >= usuario.fiat {
                u.fiat -= usuario.fiat;
                let ahora = Utc::now();
                let fecha = Fecha {
                    dia: ahora.day(),
                    mes: ahora.month(),
                    anio: ahora.year() as u32,
                };
                let hash = generar_hash(&u.adress, monto, &fecha);
                let transaccion = TransaccionPlataforma {
                    fecha,
                    tipo: "retiro fiat".to_string(),
                    monto,
                    hash: hash.clone(),
                    medioDeRetiro: medio,
                    usuario: usuario.clone(),
                    criptomoneda: None,
                };
                self.transacciones.insert(hash, transaccion);
            }
        }
    }
    //		➢ Saber cual es la criptomoneda que más cantidad de ventas tiene

    fn cripto_mas_vendida(&self) -> String {
        let mut cripto_mas_vendida = "".to_string();
        let mut maximo: f32 = 0.0;
        for (_, transaccion) in self.transacciones.iter() {
            if transaccion.tipo == "venta de cripto" {
                if let Some(cripto) = &transaccion.criptomoneda {
                    let index = self
                        .balance
                        .iter()
                        .position(|x| x.prefijo == cripto.prefijo);
                    if let Some(i) = index {
                        if self.balance[i].monto > maximo {
                            maximo = self.balance[i].monto;
                            cripto_mas_vendida = self.balance[i].prefijo.clone();
                        }
                    }
                }
            }
        }
        cripto_mas_vendida
    }

    /*
    ➢ S  ➢ Saber cual es la criptomoneda que más cantidad de compras tiene

     */
    fn cripto_mas_comprada(&self) -> String {
        let mut cripto_mas_comprada = "".to_string();
        let mut maximo: f32 = 0.0;
        for (_, transaccion) in self.transacciones.iter() {
            if transaccion.tipo == "compra de cripto" {
                if let Some(cripto) = &transaccion.criptomoneda {
                    let index = self
                        .balance
                        .iter()
                        .position(|x| x.prefijo == cripto.prefijo);
                    if let Some(i) = index {
                        if self.balance[i].monto > maximo {
                            maximo = self.balance[i].monto;
                            cripto_mas_comprada = self.balance[i].prefijo.clone();
                        }
                    }
                }
            }
        }
        cripto_mas_comprada
    }

    //➢   ➢ Saber cual es la criptomoneda que más volumen de ventas tiene

    fn cripto_mas_volumen_ventas(&self) -> String {
        let mut cripto_mas_volumen_ventas = "".to_string();
        let mut maximo: f32 = 0.0;
        for (_, transaccion) in self.transacciones.iter() {
            if transaccion.tipo == "venta de cripto" {
                if let Some(cripto) = &transaccion.criptomoneda {
                    let index = self
                        .balance
                        .iter()
                        .position(|x| x.prefijo == cripto.prefijo);
                    if let Some(i) = index {
                        if transaccion.monto > maximo {
                            maximo = transaccion.monto;
                            cripto_mas_volumen_ventas = self.balance[i].prefijo.clone();
                        }
                    }
                }
            }
        }
        cripto_mas_volumen_ventas
    }

    //   ➢ Saber cual es la criptomoneda que más volumen de compras tiene
    fn cripto_mas_volumen_compras(&self) -> String {
        let mut cripto_mas_volumen_compras = "".to_string();
        let mut maximo: f32 = 0.0;
        for (_, transaccion) in self.transacciones.iter() {
            if transaccion.tipo == "compra de cripto" {
                if let Some(cripto) = &transaccion.criptomoneda {
                    let index = self
                        .balance
                        .iter()
                        .position(|x| x.prefijo == cripto.prefijo);
                    if let Some(i) = index {
                        if transaccion.monto > maximo {
                            maximo = transaccion.monto;
                            cripto_mas_volumen_compras = self.balance[i].prefijo.clone();
                        }
                    }
                }
            }
        }
        cripto_mas_volumen_compras
    }
}
fn generar_hash(adress: &str, monto: f32, fecha: &Fecha) -> String {
    let mut hasher = Sha256::new();
    hasher.update(adress);
    hasher.update(monto.to_string().as_bytes());
    hasher.update(fecha.dia.to_string().as_bytes());
    hasher.update(fecha.mes.to_string().as_bytes());
    hasher.update(fecha.anio.to_string().as_bytes());
    format!("{:x}", hasher.finalize())
}
#[derive(Clone)]
struct Usuario {
    adress: String,
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
    validado: bool,
    fiat: f32,
    balance: Vec<Criptomoneda>,
}
#[derive(Clone)]
struct Criptomoneda {
    nombre: String,
    prefijo: String,
    monto: f32,
    listado_de_blockchain: Vec<Blockchain>, //nombre de las blockchains
                                            //no me sirve que sea tipo Blockchains, porque no me interesa tener toda la estructura
                                            //solo quiero saber con cuales blockchains se puede operar, esta criptomoneda
}
impl Criptomoneda {
    fn cotizacion(&self) -> f32 {
        //por fines prácticos, devuelvo un valor random
        match self.prefijo.as_str() {
            "BTC" => 50000.0,
            "ETH" => 2000.0,
            "Luna" => 1.0,
            _ => 0.0,
        }
    }
}
#[derive(Clone)]
struct Blockchain {
    nombre: String,
    prefijo: String,
    transacciones: HashMap<String, TransaccionBlockchain>, //Hash de la transaccion, transaccion
}
/*
:fecha, tipo(ingreso de dinero), monto, usuario.
*/
struct TransaccionPlataforma {
    fecha: Fecha,
    tipo: String, //ingreso de dinero, compra de cripto, venta de cripto, retiro cripto, recepción cripto, retiro fiat
    monto: f32,
    hash: String,
    medioDeRetiro: MedioRetiro,
    usuario: Usuario,
    criptomoneda: Option<Criptomoneda>,
}
#[derive(Clone)]
struct TransaccionBlockchain {
    fecha: Fecha,
    tipo: String, //retiro cripto, recepción cripto
    monto: f32,
    blockchain: Blockchain,
    hash: String,
    criptomoneda: Criptomoneda,
    cotizacion: f32,
}
#[derive(Clone)]
struct Fecha {
    dia: u32,
    mes: u32,
    anio: u32,
}
enum MedioRetiro {
    MercadoPago,
    TransferenciaBancaria,
    None,
}

pub fn main() {}

#[test]
fn test_ingresar_dinero() {
    let mut plataforma = Plataforma {
        nombre: "Plataforma".to_string(),
        usuarios: HashMap::new(),
        transacciones: HashMap::new(),
        balance: vec![],
    };
    let usuario = Usuario {
        adress: "1".to_string(),
        nombre: "Juan".to_string(),
        apellido: "Perez".to_string(),
        email: "sdada".to_string(),
        dni: "123".to_string(),
        validado: true,
        fiat: 0.0,
        balance: vec![],
    };
    plataforma
        .usuarios
        .insert(usuario.adress.clone(), usuario.clone());
    plataforma.ingresar_dinero(100.0, usuario.clone());
    assert_eq!(
        plataforma.usuarios.get(&usuario.adress).unwrap().fiat,
        100.0
    );
}
#[test]
fn test_comprar_criptomoneda() {
    let mut plataforma = Plataforma {
        nombre: "Plataforma".to_string(),
        usuarios: HashMap::new(),
        transacciones: HashMap::new(),
        balance: vec![],
    };
    let usuario = Usuario {
        adress: "1".to_string(),
        nombre: "Juan".to_string(),
        apellido: "Perez".to_string(),
        email: "sdada".to_string(),
        dni: "123".to_string(),
        validado: true,
        fiat: 100.0,
        balance: vec![],
    };
    let criptomoneda = Criptomoneda {
        nombre: "Bitcoin".to_string(),
        prefijo: "BTC".to_string(),
        monto: 0.0,
        listado_de_blockchain: vec![],
    };
    plataforma
        .usuarios
        .insert(usuario.adress.clone(), usuario.clone());
    plataforma.comprar_criptomoneda(50.0, &mut usuario.clone(), &criptomoneda);
    assert_eq!(plataforma.usuarios.get(&usuario.adress).unwrap().fiat, 50.0);
    assert_eq!(
        plataforma.usuarios.get(&usuario.adress).unwrap().balance[0].monto,
        0.001
    );
}

#[test]
fn test_vender_criptomoneda() {
    let mut plataforma = Plataforma {
        nombre: "Plataforma".to_string(),
        usuarios: HashMap::new(),
        transacciones: HashMap::new(),
        balance: vec![],
    };
    let usuario = Usuario {
        adress: "1".to_string(),
        nombre: "Juan".to_string(),
        apellido: "Perez".to_string(),
        email: "sdada".to_string(),
        dni: "123".to_string(),
        validado: true,
        fiat: 100.0,
        balance: vec![],
    };
    let criptomoneda = Criptomoneda {
        nombre: "Bitcoin".to_string(),
        prefijo: "BTC".to_string(),
        monto: 200.0,
        listado_de_blockchain: vec![],
    };
    plataforma
        .usuarios
        .insert(usuario.adress.clone(), usuario.clone());
    plataforma.vender_criptomoneda(3.0, &mut usuario.clone(), &criptomoneda);
    assert_eq!(plataforma.usuarios.get(&usuario.adress).unwrap().fiat, 50.0);
    // assert_eq!(
    //     plataforma.usuarios.get(&usuario.adress).unwrap().balance[0].monto,
    //     0.0
    // );
}
// struct Blockchain{
// 	nombre: String,
// 	prefijo: String,
// 	usuarios: hashmap<String, Usuario>, //adress de usuario, usuario
// 	usuarios: Vec<Usuario>,
// 	trasnsacciones: hashmap<String, Transaccion>, //Hash de la transaccion, transaccion
// 	// transacciones: Vec<Transaccion>,
// 	balance: Vec<Criptomoneda>,
// }
// /*
//  ➢ Ingresar dinero: se recibe un monto en fiat de un usuario y se acredita al balance de
//   fiat de dicho usuario. Además se crea una transacción del hecho donde los datos
//   que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.
// */
// impl Blockchain{
// 	fn ingresar_dinero(&self, monto: f32, usuario: &mut Usuario){
// 		if let Some(u) = self.usuarios.get(usuario.adress){
// 			u.balance.push(Criptomoneda{nombre: "Fiat".to_string(), prefijo: "USD".to_string(), blockchains: vec!["Fiat".to_string()]});
// 			u.balance[0].monto += monto;
// 			let fecha = Fecha{dia: 1, mes: 1, anio: 2021};
// 			let transaccion = Transaccion{fecha: fecha, tipo: "ingreso de dinero".to_string(), monto: monto, adress_sender: "".to_string(), addres_receiver: u.adress, hash: "".to_string(), criptomoneda: u.balance[0], cotizacion: 1.0};
// 			self.transacciones.insert
// 	}

// }

// struct Usuario{
// 	adress: String,
// 	nombre: String,
// 	apellido: String,
// 	email: String,
// 	dni: String,
// 	validado: bool,
// 	balance : Vec<Criptomoneda>,
// }

// struct Criptomoneda{
// 	nombre: String,
// 	prefijo: String,
// 	blockchains: Vec<String>, //nombre de las blockchains
// 	//no me sirve que sea tipo Blockchains, porque no me interesa tener toda la estructura
// 	//solo quiero saber con cuales blockchains se puede operar, esta criptomoneda
// }
// /*
// :fecha, tipo(ingreso de dinero), monto, usuario.
// */
// struct Transaccion{
// 	fecha: Fecha,
// 	tipo: String,	//ingreso de dinero, compra de cripto, venta de cripto, retiro cripto, recepción cripto, retiro fiat
// 	monto: f32,
// 	adress_sender: String,
// 	addres_receiver: String,
// 	hash: String,
// 	criptomoneda: Criptomoneda,
// 	cotizacion: f32,
// }

// struct Fecha{
// 	dia: u32,
// 	mes: u32,
// 	anio: u32,
// }
