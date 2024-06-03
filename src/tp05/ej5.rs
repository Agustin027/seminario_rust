use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fmt::{self, Display},
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    vec,
};

#[derive(Debug)]
struct MiError {
    msg: String,
}
impl std::error::Error for MiError {}

impl std::fmt::Display for MiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<std::io::Error> for MiError {
    fn from(error: std::io::Error) -> Self {
        MiError {
            msg: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for MiError {
    fn from(error: serde_json::Error) -> Self {
        MiError {
            msg: error.to_string(),
        }
    }
}
use super::fecha::Fecha;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct StreamingRust {
    subs: BTreeMap<i32, Usuario>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Usuario {
    id: i32,
    sub: (bool, Suscripcion),
    mediopago: MedioPago,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Suscripcion {
    tipo: TipoSuscripcion,
    fecha_inicio: Fecha,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Mp {
    usuario: String,
    mediopago: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Tarjeta {
    numero: i32,
    fecha_vencimiento: Fecha,
    cvv: i32,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Transferencia {
    cbu: i32,
    alias: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Cripto {
    direccion: String,
    cadena: String,
    cripto: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum MedioPago {
    Efectivo(f32),
    MercadoPago(Mp),
    Tarjeta(Tarjeta),
    Transferencia(Transferencia),
    Cripto(Cripto),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}

impl TipoSuscripcion {
    fn precio(&self) -> f64 {
        match self {
            TipoSuscripcion::Basic => 10.0,
            TipoSuscripcion::Clasic => 20.0,
            TipoSuscripcion::Super => 30.0,
        }
    }
    fn duracion(&self) -> i32 {
        match self {
            TipoSuscripcion::Basic => 1,
            TipoSuscripcion::Clasic => 2,
            TipoSuscripcion::Super => 3,
        }
    }
}

impl StreamingRust {
    fn new() -> StreamingRust {
        let subs = match OpenOptions::new().read(true).open("subs.json") {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                serde_json::from_str(&buf).unwrap()
            }
            Err(_) => BTreeMap::new(),
        };
        StreamingRust { subs }
    }
    fn crear_usuario(
        &mut self,
        id: i32,
        mediopago: MedioPago,
        tipo: TipoSuscripcion,
    ) -> Result<(), StreamingError> {
        if self.subs.contains_key(&id) {
            Err(StreamingError::UsuarioYaExiste)
        } else {
            let user = Usuario {
                id,
                sub: (
                    true,
                    Suscripcion {
                        tipo,
                        fecha_inicio: Fecha::new(1, 1, 2024),
                    },
                ),
                mediopago,
            };
            self.subs.insert(id, user);
            self.guardar_subs();
            Ok(())
        }
    }

    fn upgrade_subscripcion(&mut self, user_id: i32) -> Result<(), StreamingError> {
        if let Some(aux) = self.subs.get_mut(&user_id) {
            if aux.sub.0 {
                let nuevo_tipo = match aux.sub.1.tipo {
                    TipoSuscripcion::Basic => TipoSuscripcion::Clasic,
                    TipoSuscripcion::Clasic => TipoSuscripcion::Super,
                    TipoSuscripcion::Super => {
                        return Err(StreamingError::YaEsSuper);
                    }
                };
                aux.sub.1.tipo = nuevo_tipo;
                self.guardar_subs();
                Ok(())
            } else {
                Err(StreamingError::SubscripcionInactiva)
            }
        } else {
            Err(StreamingError::UsuarioNoExiste)
        }
    }

    fn downgrade_subscripcion(&mut self, user_id: i32) -> Result<(), StreamingError> {
        if let Some(aux) = self.subs.get_mut(&user_id) {
            if aux.sub.0 {
                let nuevo_tipo = match aux.sub.1.tipo {
                    TipoSuscripcion::Basic => {
                        aux.sub.0 = false;
                        TipoSuscripcion::Basic
                    }
                    TipoSuscripcion::Clasic => TipoSuscripcion::Basic,
                    TipoSuscripcion::Super => TipoSuscripcion::Clasic,
                };
                if aux.sub.0 {
                    aux.sub.1.tipo = nuevo_tipo;
                }
                self.guardar_subs();
                Ok(())
            } else {
                Err(StreamingError::SubscripcionInactiva)
            }
        } else {
            Err(StreamingError::UsuarioNoExiste)
        }
    }

    fn cancelar_subscripcion(&mut self, user_id: i32) -> Result<(), StreamingError> {
        if let Some(aux) = self.subs.get_mut(&user_id) {
            if aux.sub.0 {
                aux.sub.0 = false;
                self.guardar_subs();
                Ok(())
            } else {
                Err(StreamingError::SubscripcionInactiva)
            }
        } else {
            Err(StreamingError::UsuarioNoExiste)
        }
    }

    fn medio_de_pago_mas_usado_subs_activas(&self) -> MedioPago {
        let mut contador = vec![0, 0, 0, 0, 0];
        for (user_id, user) in self.subs.iter() {
            if user.sub.0 == true {
                match user.mediopago {
                    MedioPago::Efectivo(_) => contador[0] += 1,
                    MedioPago::MercadoPago(_) => contador[1] += 1,
                    MedioPago::Tarjeta(_) => contador[2] += 1,
                    MedioPago::Transferencia(_) => contador[3] += 1,
                    MedioPago::Cripto(_) => contador[4] += 1,
                }
            }
        }
        println!("aaaaaa{:?}", contador);
        return contador.retornar_maximo_medio_de_pago();
    }
    fn subscripcion_mas_contratada_subs_activas(&self) -> TipoSuscripcion {
        let mut contador = vec![0, 0, 0];
        for user in self.subs.values() {
            if user.sub.0 == true {
                match &user.sub.1.tipo {
                    TipoSuscripcion::Basic => contador[0] += 1,
                    TipoSuscripcion::Clasic => contador[1] += 1,
                    TipoSuscripcion::Super => contador[2] += 1,
                }
            }
        }

        return contador.retornar_maximo_subscripcion();
    }
    fn medio_de_pago_mas_usado(&self) -> MedioPago {
        let mut contador = vec![0, 0, 0, 0, 0];
        for (user_id, user) in self.subs.iter() {
            match user.mediopago {
                MedioPago::Efectivo(_) => contador[0] += 1,
                MedioPago::MercadoPago(_) => contador[1] += 1,
                MedioPago::Tarjeta(_) => contador[2] += 1,
                MedioPago::Transferencia(_) => contador[3] += 1,
                MedioPago::Cripto(_) => contador[4] += 1,
            }
        }

        return contador.retornar_maximo_medio_de_pago();
    }
    fn subscripcion_mas_contratada(&self) -> TipoSuscripcion {
        let mut contador = vec![0, 0, 0];
        for user in self.subs.values() {
            match &user.sub.1.tipo {
                TipoSuscripcion::Basic => contador[0] += 1,
                TipoSuscripcion::Clasic => contador[1] += 1,
                TipoSuscripcion::Super => contador[2] += 1,
            }
        }
        return contador.retornar_maximo_subscripcion();
    }
    fn guardar_subs(&self) -> Result<(), MiError> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("subs.json")?;
        serde_json::to_writer(file, &self.subs)?;
        Ok(())
    }
}

trait ContadorVec {
    fn retornar_maximo_medio_de_pago(&self) -> MedioPago;
    fn retornar_maximo_subscripcion(&self) -> TipoSuscripcion;
}

impl ContadorVec for Vec<i32> {
    fn retornar_maximo_medio_de_pago(&self) -> MedioPago {
        if let Some((indice, _)) = self.iter().enumerate().max_by_key(|&(_, value)| value) {
            let indice_maximo = indice;
            match indice_maximo {
                0 => return MedioPago::Efectivo(0.0),
                1 => {
                    return MedioPago::MercadoPago(Mp {
                        usuario: "usuario".to_string(),
                        mediopago: "mercadopago".to_string(),
                    })
                }
                2 => {
                    return MedioPago::Tarjeta(Tarjeta {
                        numero: 1234,
                        fecha_vencimiento: Fecha::new(1, 1, 2024),
                        cvv: 123,
                    })
                }
                3 => {
                    return MedioPago::Transferencia(Transferencia {
                        cbu: 1234,
                        alias: "alias".to_string(),
                    })
                }
                4 => {
                    return MedioPago::Cripto(Cripto {
                        direccion: "direccion".to_string(),
                        cadena: "cadena".to_string(),
                        cripto: "cripto".to_string(),
                    })
                }
                _ => return MedioPago::Efectivo(0.0),
            }
        }
        return MedioPago::Efectivo(0.0);
    }

    fn retornar_maximo_subscripcion(&self) -> TipoSuscripcion {
        if let Some((indice, _)) = self.iter().enumerate().max_by_key(|&(_, value)| value) {
            let indice_maximo = indice;
            match indice_maximo {
                0 => return TipoSuscripcion::Basic,
                1 => return TipoSuscripcion::Clasic,
                2 => return TipoSuscripcion::Super,
                _ => return TipoSuscripcion::Basic,
            }
        }
        return TipoSuscripcion::Basic;
    }
}

#[derive(Debug)]
enum StreamingError {
    UsuarioYaExiste,
    UsuarioNoExiste,
    SubscripcionInactiva,
    YaEsSuper,
}

impl fmt::Display for StreamingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StreamingError::UsuarioYaExiste => write!(f, "El usuario ya existe"),
            StreamingError::UsuarioNoExiste => write!(f, "El usuario no existe"),
            StreamingError::SubscripcionInactiva => {
                write!(f, "El usuario no tiene suscripción activa")
            }
            StreamingError::YaEsSuper => write!(f, "El usuario ya tiene la subscripción Super"),
        }
    }
}

impl std::error::Error for StreamingError {}

#[test]
fn test_precio() {
    assert_eq!(TipoSuscripcion::Basic.precio(), 10.0);
    assert_eq!(TipoSuscripcion::Clasic.precio(), 20.0);
    assert_eq!(TipoSuscripcion::Super.precio(), 30.0);
}

#[test]
fn test_duracion() {
    assert_eq!(TipoSuscripcion::Basic.duracion(), 1);
    assert_eq!(TipoSuscripcion::Clasic.duracion(), 2);
    assert_eq!(TipoSuscripcion::Super.duracion(), 3);
}

#[test]
fn test_general() {
    let mut streaming = StreamingRust::new();
    let tarjeta = Tarjeta {
        numero: 1234,
        fecha_vencimiento: Fecha::new(1, 1, 2024),
        cvv: 123,
    };

    assert!(streaming
        .crear_usuario(
            1,
            MedioPago::Tarjeta(tarjeta.clone()),
            TipoSuscripcion::Basic
        )
        .is_ok());

    let mut user = Usuario {
        id: 1,
        sub: (
            true,
            Suscripcion {
                tipo: TipoSuscripcion::Basic,
                fecha_inicio: Fecha::new(1, 1, 2024),
            },
        ),
        mediopago: MedioPago::Tarjeta(tarjeta.clone()),
    };

    assert_eq!(streaming.subs.get(&1), Some(&user));
    assert_eq!(streaming.subs.len(), 1);

    assert!(streaming.upgrade_subscripcion(1).is_ok());
    user.sub.1.tipo = TipoSuscripcion::Clasic;
    assert_eq!(streaming.subs.get(&1), Some(&user));

    assert!(streaming.downgrade_subscripcion(1).is_ok());
    user.sub.1.tipo = TipoSuscripcion::Basic;
    assert_eq!(streaming.subs.get(&1), Some(&user));

    assert_eq!(
        streaming.medio_de_pago_mas_usado_subs_activas(),
        MedioPago::Tarjeta(tarjeta.clone())
    );
    assert_eq!(
        streaming.subscripcion_mas_contratada_subs_activas(),
        TipoSuscripcion::Basic
    );

    assert!(streaming.cancelar_subscripcion(1).is_ok());
    user.sub.0 = false;
    assert_eq!(streaming.subs.get(&1), Some(&user));
    assert_eq!(
        streaming.medio_de_pago_mas_usado(),
        MedioPago::Tarjeta(tarjeta.clone())
    );
    assert_eq!(
        streaming.subscripcion_mas_contratada(),
        TipoSuscripcion::Basic
    );
    assert!(streaming
        .crear_usuario(
            1,
            MedioPago::Tarjeta(tarjeta.clone()),
            TipoSuscripcion::Basic
        )
        .is_err());
}
#[test]
fn test_contador_vec() {
    let vec = vec![1, 2, 13, 4, 5];
    assert_eq!(
        vec.retornar_maximo_medio_de_pago(),
        MedioPago::Tarjeta(Tarjeta {
            numero: 1234,
            fecha_vencimiento: Fecha::new(1, 1, 2024),
            cvv: 123,
        })
    );
    assert_eq!(vec.retornar_maximo_subscripcion(), TipoSuscripcion::Super);
}

#[test]
fn test_upgrade_sub() {
    let mut streaming = StreamingRust::new();
    let tarjeta = Tarjeta {
        numero: 1234,
        fecha_vencimiento: Fecha::new(1, 1, 2024),
        cvv: 123,
    };
    let mut user = Usuario {
        id: 1,
        sub: (
            true,
            Suscripcion {
                tipo: TipoSuscripcion::Clasic,
                fecha_inicio: Fecha::new(1, 1, 2024),
            },
        ),
        mediopago: MedioPago::Tarjeta(tarjeta.clone()),
    };
    streaming.crear_usuario(
        1,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Clasic,
    );
    assert_eq!(streaming.subs.get(&1), Some(&user));
    assert_eq!(streaming.subs.len(), 1);

    streaming.upgrade_subscripcion(user.id.clone());

    user.sub.1.tipo = TipoSuscripcion::Super;
    assert_eq!(streaming.subs.get(&1), Some(&user));

    assert!(streaming.upgrade_subscripcion(user.id.clone()).is_err());
    streaming.cancelar_subscripcion(user.id.clone());
    assert!(streaming.upgrade_subscripcion(user.id.clone()).is_err());
    user.id = 2;
    assert!(streaming.upgrade_subscripcion(user.id.clone()).is_err());
}
#[test]
fn test_downgrade_sub() {
    let mut streaming = StreamingRust::new();
    let tarjeta = Tarjeta {
        numero: 1234,
        fecha_vencimiento: Fecha::new(1, 1, 2024),
        cvv: 123,
    };
    let mut user = Usuario {
        id: 1,
        sub: (
            true,
            Suscripcion {
                tipo: TipoSuscripcion::Super,
                fecha_inicio: Fecha::new(1, 1, 2024),
            },
        ),
        mediopago: MedioPago::Tarjeta(tarjeta.clone()),
    };
    streaming.crear_usuario(
        1,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Super,
    );
    assert_eq!(streaming.subs.get(&1), Some(&user));
    assert_eq!(streaming.subs.len(), 1);

    streaming.downgrade_subscripcion(user.id.clone());
    user.sub.1.tipo = TipoSuscripcion::Clasic;
    assert_eq!(streaming.subs.get(&1), Some(&user));

    user.sub.1.tipo = TipoSuscripcion::Basic;
    streaming.downgrade_subscripcion(user.id.clone());
    assert_eq!(streaming.subs.get(&1), Some(&user));

    streaming.downgrade_subscripcion(user.id);
    assert!(streaming.downgrade_subscripcion(user.id.clone()).is_err());
    user.id = 2;
    assert!(streaming.downgrade_subscripcion(user.id.clone()).is_err());
}
#[test]
fn test_cancelar_sub() {
    let mut streaming = StreamingRust::new();
    let tarjeta = Tarjeta {
        numero: 1234,
        fecha_vencimiento: Fecha::new(1, 1, 2024),
        cvv: 123,
    };
    let mut user = Usuario {
        id: 1,
        sub: (
            true,
            Suscripcion {
                tipo: TipoSuscripcion::Clasic,
                fecha_inicio: Fecha::new(1, 1, 2024),
            },
        ),
        mediopago: MedioPago::Tarjeta(tarjeta.clone()),
    };
    streaming.crear_usuario(
        1,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Clasic,
    );
    assert_eq!(streaming.subs.get(&1), Some(&user));
    assert_eq!(streaming.subs.len(), 1);

    streaming.cancelar_subscripcion(user.id.clone());

    user.sub.0 = false;
    assert_eq!(streaming.subs.get(&1), Some(&user));
    user.sub.0 = false;
    assert!(streaming.cancelar_subscripcion(user.id.clone()).is_err());
    user.id = 2;
    assert!(streaming.cancelar_subscripcion(user.id.clone()).is_err());
}
#[test]
fn test_medio_de_pago_mas_usado_subs_activas() {
    let mut streaming = StreamingRust::new();
    let tarjeta = Tarjeta {
        numero: 1234,
        fecha_vencimiento: Fecha::new(1, 1, 2024),
        cvv: 123,
    };
    streaming.crear_usuario(
        1,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Clasic,
    );
    streaming.crear_usuario(2, MedioPago::Efectivo(0.0), TipoSuscripcion::Clasic);
    streaming.crear_usuario(
        3,
        MedioPago::MercadoPago(Mp {
            usuario: "usuario".to_string(),
            mediopago: "mercadopago".to_string(),
        }),
        TipoSuscripcion::Clasic,
    );
    streaming.crear_usuario(
        4,
        MedioPago::Transferencia(Transferencia {
            cbu: 1234,
            alias: "alias".to_string(),
        }),
        TipoSuscripcion::Clasic,
    );
    streaming.crear_usuario(
        5,
        MedioPago::Cripto(Cripto {
            direccion: "direccion".to_string(),
            cadena: "cadena".to_string(),
            cripto: "cripto".to_string(),
        }),
        TipoSuscripcion::Clasic,
    );
    streaming.crear_usuario(
        6,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Clasic,
    );
    assert_eq!(streaming.subs.len(), 6);

    assert_eq!(
        streaming.medio_de_pago_mas_usado_subs_activas(),
        MedioPago::Tarjeta(tarjeta.clone())
    );
}
#[test]
fn test_subscripcion_mas_contratada_subs_activas() {
    let mut streaming = StreamingRust::new();

    streaming.crear_usuario(1, MedioPago::Efectivo(0.0), TipoSuscripcion::Basic);
    streaming.crear_usuario(2, MedioPago::Efectivo(0.0), TipoSuscripcion::Clasic);
    streaming.crear_usuario(3, MedioPago::Efectivo(0.0), TipoSuscripcion::Super);
    streaming.crear_usuario(4, MedioPago::Efectivo(0.0), TipoSuscripcion::Basic);
    assert_eq!(streaming.subs.len(), 4);

    assert_eq!(
        streaming.subscripcion_mas_contratada_subs_activas(),
        TipoSuscripcion::Basic
    );
}
#[test]
fn test_medio_de_pago_mas_usado() {
    let mut streaming = StreamingRust::new();
    let tarjeta = Tarjeta {
        numero: 1234,
        fecha_vencimiento: Fecha::new(1, 1, 2024),
        cvv: 123,
    };
    streaming.crear_usuario(
        1,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Clasic,
    );
    streaming.crear_usuario(2, MedioPago::Efectivo(0.0), TipoSuscripcion::Clasic);
    streaming.crear_usuario(
        3,
        MedioPago::MercadoPago(Mp {
            usuario: "usuario".to_string(),
            mediopago: "mercadopago".to_string(),
        }),
        TipoSuscripcion::Clasic,
    );
    streaming.crear_usuario(
        4,
        MedioPago::Transferencia(Transferencia {
            cbu: 1234,
            alias: "alias".to_string(),
        }),
        TipoSuscripcion::Clasic,
    );
    streaming.crear_usuario(
        5,
        MedioPago::Cripto(Cripto {
            direccion: "direccion".to_string(),
            cadena: "cadena".to_string(),
            cripto: "cripto".to_string(),
        }),
        TipoSuscripcion::Clasic,
    );
    streaming.crear_usuario(
        6,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Clasic,
    );
    assert_eq!(streaming.subs.len(), 6);
    assert_eq!(
        streaming.medio_de_pago_mas_usado(),
        MedioPago::Tarjeta(tarjeta.clone())
    );
}
#[test]
fn test_subscripcion_mas_contratada() {
    let mut streaming = StreamingRust::new();
    let tarjeta = Tarjeta {
        numero: 1234,
        fecha_vencimiento: Fecha::new(1, 1, 2024),
        cvv: 123,
    };
    let mut user = Usuario {
        id: 1,
        sub: (
            true,
            Suscripcion {
                tipo: TipoSuscripcion::Clasic,
                fecha_inicio: Fecha::new(1, 1, 2024),
            },
        ),
        mediopago: MedioPago::Tarjeta(tarjeta.clone()),
    };
    streaming.crear_usuario(
        1,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Clasic,
    );
    assert_eq!(streaming.subs.get(&1), Some(&user));
    assert_eq!(streaming.subs.len(), 1);

    assert_eq!(
        streaming.subscripcion_mas_contratada(),
        TipoSuscripcion::Clasic
    );
    user.sub.0 = false;
    streaming.cancelar_subscripcion(user.id.clone());
    assert_eq!(
        streaming.subscripcion_mas_contratada(),
        TipoSuscripcion::Clasic
    );
    user.id = 2;
    streaming.cancelar_subscripcion(user.id.clone());
    assert_eq!(
        streaming.subscripcion_mas_contratada(),
        TipoSuscripcion::Clasic
    );
}
