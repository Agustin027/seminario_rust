trait MedioPagoTrait {
    fn from_int(&self) -> MedioPago;
}

impl MedioPagoTrait for i32 {
    fn from_int(&self) -> MedioPago {
        match self {
            0 => MedioPago::Efectivo(0.0),
            1 => MedioPago::MercadoPago(MercadoPago {
                cuenta: "".to_string(),
                forma_pago: FormaPagoMP::TarjetaCredito, // Aquí especifica una forma de pago
            }),
            2 => MedioPago::Credito(Tcredito {
                nombre: "".to_string(),
                numero: "".to_string(),
                codigo_seguridad: "".to_string(),
                vencimiento: "".to_string(),
            }),
            3 => MedioPago::TransferenciaBancaria(TransferenciaBancaria {
                banco: "".to_string(),
                cbu: "".to_string(),
                alias: "".to_string(),
                titular: "".to_string(),
                cuil: "".to_string(),
                id_transaccion: "".to_string(),
            }),
            4 => MedioPago::Cripto(Cripto {
                red: "".to_string(),
                moneda: "".to_string(),
                wallet: "".to_string(),
            }),
            _ => panic!("Invalid value for medios_pago"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum FormaPagoMP {
    TarjetaCredito,
    TarjetaDebito,
    Efectivo,
    DineroEnCuenta,
}
#[derive(Debug, PartialEq, Clone)]
struct MercadoPago {
    cuenta: String,
    forma_pago: FormaPagoMP,
}
#[derive(Debug, PartialEq, Clone)]
struct Tcredito {
    nombre: String,
    numero: String,
    codigo_seguridad: String,
    vencimiento: String,
}
#[derive(Debug, PartialEq, Clone)]
struct TransferenciaBancaria {
    banco: String,
    cbu: String,
    alias: String,
    titular: String,
    cuil: String,
    id_transaccion: String,
}

#[derive(Debug, PartialEq, Clone)]
struct Cripto {
    red: String,
    moneda: String,
    wallet: String,
}
#[derive(Debug, PartialEq, Clone)]
enum MedioPago {
    Efectivo(f64),
    MercadoPago(MercadoPago),
    Credito(Tcredito),
    TransferenciaBancaria(TransferenciaBancaria),
    Cripto(Cripto),
}
impl MedioPago {
    fn datos(&self) -> String {
        match self {
            MedioPago::Efectivo(cantidad) => format!("Efectivo: ${}", cantidad),
            MedioPago::MercadoPago(mp) => format!(
                "MercadoPago: Cuenta: {}, Forma de Pago: {:?}",
                mp.cuenta, mp.forma_pago
            ),
            MedioPago::Credito(tc) => format!(
                "Tarjeta de Credito: Nombre: {}, Numero: {}, Codigo de Seguridad: {}, Vencimiento: {}",
                tc.nombre, tc.numero, tc.codigo_seguridad, tc.vencimiento
            ),
            MedioPago::TransferenciaBancaria(tb) => format!(
                "Transferencia Bancaria: Banco: {}, CBU: {}, Alias: {}, Titular: {}, CUIL: {}, ID Transaccion: {}", tb.banco, tb.cbu, tb.alias, tb.titular, tb.cuil, tb.id_transaccion),
            MedioPago::Cripto(c) => format!( "Cripto: Red: {}, Moneda: {}, Wallet: {}", c.red, c.moneda, c.wallet),
        }
    }
    fn to_int(&self) -> i32 {
        match self {
            MedioPago::Efectivo(_) => 0,
            MedioPago::MercadoPago(_) => 1,
            MedioPago::Credito(_) => 2,
            MedioPago::TransferenciaBancaria(_) => 3,
            MedioPago::Cripto(_) => 4,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
enum TipoSubscripcion {
    Basic,
    Clasic,
    Super,
}

impl TipoSubscripcion {
    fn costo_mensual(&self) -> f64 {
        match self {
            TipoSubscripcion::Basic => 100.0,
            TipoSubscripcion::Clasic => 200.0,
            TipoSubscripcion::Super => 300.0,
        }
    }

    fn duracion_meses(&self) -> u8 {
        match self {
            TipoSubscripcion::Basic => 1,
            TipoSubscripcion::Clasic => 3,
            TipoSubscripcion::Super => 6,
        }
    }
    fn upgrade(&mut self) {
        *self = match self {
            TipoSubscripcion::Basic => TipoSubscripcion::Clasic,
            TipoSubscripcion::Clasic => TipoSubscripcion::Super,
            TipoSubscripcion::Super => TipoSubscripcion::Super,
        };
    }

    fn downgrade(&mut self) {
        *self = match self {
            TipoSubscripcion::Super => TipoSubscripcion::Clasic,
            TipoSubscripcion::Clasic => TipoSubscripcion::Basic,
            _ => TipoSubscripcion::Basic,
        };
    }
}

impl MedioPago {}

struct StreamingRust {
    subs_usuarios: Vec<Subscripcion>,
}

#[derive(Debug, PartialEq, Clone)]
struct Subscripcion {
    tipo: TipoSubscripcion,
    costo_mensual: f64,
    duracion_meses: u8,
    fecha_inicio: String,
    medios_pago: MedioPago,
}
pub fn contar_medios_de_pago(vec: Vec<Subscripcion>, cont: &mut Vec<i32>) {
    for i in 0..vec.len() {
        match vec[i].medios_pago.to_int() {
            0 => cont[0] += 1,
            1 => cont[1] += 1,
            2 => cont[2] += 1,
            3 => cont[3] += 1,
            4 => cont[4] += 1,
            _ => panic!("Invalid value for medios_pago"),
        }
    }
}

impl StreamingRust {
    fn crear_usuario(&mut self, sub: TipoSubscripcion, medio_de_pago: MedioPago) {
        let subscripcion = Subscripcion {
            tipo: sub.clone(),
            costo_mensual: sub.costo_mensual(),
            duracion_meses: sub.duracion_meses(),
            fecha_inicio: "".to_string(),
            medios_pago: medio_de_pago,
        };
        self.subs_usuarios.push(subscripcion);
    }

    fn upgradrear_usuario(&mut self, id: usize) {
        self.subs_usuarios[id].tipo.upgrade();
        self.subs_usuarios[id].costo_mensual = self.subs_usuarios[id].tipo.costo_mensual();
        self.subs_usuarios[id].duracion_meses = self.subs_usuarios[id].tipo.duracion_meses();
    }
    fn downgradear_usuario(&mut self, id: usize) {
        if self.subs_usuarios[id].tipo == TipoSubscripcion::Basic {
            self.subs_usuarios.remove(id);
        } else {
            self.subs_usuarios[id].tipo.downgrade();
            self.subs_usuarios[id].costo_mensual = self.subs_usuarios[id].tipo.costo_mensual();
            self.subs_usuarios[id].duracion_meses = self.subs_usuarios[id].tipo.duracion_meses();
        }
    }
    fn cancelar_subscripcion(&mut self, id: usize) {
        self.subs_usuarios.remove(id);
    }
    fn medio_de_pago_mas_usado(self) -> MedioPago {
        let mut contador = vec![0; 5];

        contar_medios_de_pago(self.subs_usuarios.clone(), &mut contador);

        let iter = contador.iter().enumerate();
        let max = iter.max_by(|a, b| a.1.cmp(b.1)).unwrap().0 as i32;
        println!("{:?}", contador);
        println!("{:?}", max);
        return max.from_int();
    }

    fn subscripcion_mas_contratada(self) -> TipoSubscripcion {
        let mut contador = vec![0; 3];
        for i in 0..self.subs_usuarios.len() {
            match self.subs_usuarios[i].tipo {
                TipoSubscripcion::Basic => contador[0] += 1,
                TipoSubscripcion::Clasic => contador[1] += 1,
                TipoSubscripcion::Super => contador[2] += 1,
            }
        }
        let iter = contador.iter().enumerate();
        let max = iter.max_by(|a, b| a.1.cmp(b.1)).unwrap().0 as i32;
        return match max {
            0 => TipoSubscripcion::Basic,
            1 => TipoSubscripcion::Clasic,
            2 => TipoSubscripcion::Super,
            _ => TipoSubscripcion::Basic,
        };
    }
}

#[test]
fn test_upgradear_usuario() {
    let mut streaming = StreamingRust {
        subs_usuarios: vec![],
    };
    let medio_pago = MedioPago::Efectivo(100.0);
    streaming.crear_usuario(TipoSubscripcion::Basic, medio_pago);
    streaming.upgradrear_usuario(0);
    assert_eq!(streaming.subs_usuarios[0].tipo, TipoSubscripcion::Clasic);
    streaming.upgradrear_usuario(0);
    assert_eq!(streaming.subs_usuarios[0].tipo, TipoSubscripcion::Super);
    assert_eq!(streaming.subs_usuarios[0].costo_mensual, 300.0);
}

#[test]
fn test_downgradear_usuario() {
    let mut streaming = StreamingRust {
        subs_usuarios: vec![],
    };
    let mut medio_pago = MedioPago::Efectivo(100.0);
    streaming.crear_usuario(TipoSubscripcion::Super, medio_pago.clone());
    medio_pago = MedioPago::Efectivo(150.0);
    streaming.crear_usuario(TipoSubscripcion::Clasic, medio_pago.clone());
    medio_pago = MedioPago::Efectivo(200.0);
    streaming.crear_usuario(TipoSubscripcion::Basic, medio_pago.clone());

    assert_eq!(streaming.subs_usuarios.len(), 3);
    streaming.downgradear_usuario(0);
    assert_eq!(streaming.subs_usuarios[0].tipo, TipoSubscripcion::Clasic);
    assert_eq!(streaming.subs_usuarios[0].costo_mensual, 200.0);
    assert_eq!(streaming.subs_usuarios.len(), 3);
    streaming.downgradear_usuario(0);
    assert_eq!(streaming.subs_usuarios[0].tipo, TipoSubscripcion::Basic);
    assert_eq!(streaming.subs_usuarios[0].costo_mensual, 100.0);
    assert_eq!(streaming.subs_usuarios.len(), 3);
    streaming.downgradear_usuario(0);
    assert_eq!(streaming.subs_usuarios.len(), 2);
}

#[test]
fn test_cancelar_subscripcion() {
    let mut streaming = StreamingRust {
        subs_usuarios: vec![],
    };
    let medio_pago = MedioPago::Efectivo(100.0);
    streaming.crear_usuario(TipoSubscripcion::Basic, medio_pago);
    assert_eq!(streaming.subs_usuarios.len(), 1);
    streaming.cancelar_subscripcion(0);
    assert_eq!(streaming.subs_usuarios.len(), 0);
}

#[test]
fn test_medio_de_pago_mas_usado() {
    let mut streaming = StreamingRust {
        subs_usuarios: vec![],
    };
    let mut efectivo = MedioPago::Efectivo(100.0);
    let mut mercado_pago = MedioPago::MercadoPago(MercadoPago {
        cuenta: "".to_string(),
        forma_pago: FormaPagoMP::TarjetaCredito,
    });
    let mut credito = MedioPago::Credito(Tcredito {
        nombre: "".to_string(),
        numero: "".to_string(),
        codigo_seguridad: "".to_string(),
        vencimiento: "".to_string(),
    });
    let mut transferencia = MedioPago::TransferenciaBancaria(TransferenciaBancaria {
        banco: "".to_string(),
        cbu: "".to_string(),
        alias: "".to_string(),
        titular: "".to_string(),
        cuil: "".to_string(),
        id_transaccion: "".to_string(),
    });
    let mut cripto = MedioPago::Cripto(Cripto {
        red: "".to_string(),
        moneda: "".to_string(),
        wallet: "".to_string(),
    });

    streaming.crear_usuario(TipoSubscripcion::Basic, credito.clone());
    streaming.crear_usuario(TipoSubscripcion::Basic, efectivo.clone());
    streaming.crear_usuario(TipoSubscripcion::Basic, mercado_pago.clone());
    streaming.crear_usuario(TipoSubscripcion::Basic, credito.clone());
    streaming.crear_usuario(TipoSubscripcion::Basic, transferencia.clone());
    streaming.crear_usuario(TipoSubscripcion::Basic, transferencia.clone());
    streaming.crear_usuario(TipoSubscripcion::Basic, transferencia.clone());
    streaming.crear_usuario(TipoSubscripcion::Basic, transferencia.clone());
    println!("aaaaaaaa {:?}", streaming.medio_de_pago_mas_usado());
}

#[test]
fn test_subscripcion_mas_contratada() {
    let mut streaming = StreamingRust {
        subs_usuarios: vec![],
    };
    let medio_pago = MedioPago::Efectivo(100.0);
    streaming.crear_usuario(TipoSubscripcion::Basic, medio_pago.clone());
    streaming.crear_usuario(TipoSubscripcion::Basic, medio_pago.clone());
    streaming.crear_usuario(TipoSubscripcion::Clasic, medio_pago.clone());
    streaming.crear_usuario(TipoSubscripcion::Clasic, medio_pago.clone());
    streaming.crear_usuario(TipoSubscripcion::Super, medio_pago.clone());
    streaming.crear_usuario(TipoSubscripcion::Basic, medio_pago.clone());
    assert_eq!(
        streaming.subscripcion_mas_contratada(),
        TipoSubscripcion::Basic
    );
}
