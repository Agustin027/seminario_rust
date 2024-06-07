//Nombre: Agustin Castillo, Legajo:20778/1 Alias Discord: Agustin agustin2002 Dni:44130476

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Serialize, Deserialize)]
pub struct Fecha {
    pub dia: u32,
    pub mes: u32,
    pub año: i32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, año: i32) -> Self {
        Fecha { dia, mes, año }
    }

    pub fn es_bisiesto(&self) -> bool {
        self.año % 4 == 0 && (self.año % 100 != 0 || self.año % 400 == 0)
    }

    pub fn es_fecha_valida(&self) -> bool {
        if self.mes < 1 || self.mes > 12 {
            return false;
        }
        if self.dia < 1 {
            return false;
        }
        match self.mes {
            4 | 6 | 9 | 11 => {
                if self.dia <= 30 {
                    return true;
                } else {
                    return false;
                }
            }
            2 => {
                if Fecha::es_bisiesto(&self) {
                    self.dia <= 29
                } else {
                    self.dia <= 28
                }
            }
            _ => {
                if self.dia <= 31 {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }

    pub fn sumar_dias(&mut self, mut dias: u32) {
        while dias > 0 {
            let dias_en_mes = match self.mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 if self.es_bisiesto() => 29,
                _ => 28,
            };
            let dias_restantes = dias_en_mes - self.dia + 1;
            if dias_restantes <= dias {
                dias -= dias_restantes;
                self.mes += 1;
                if self.mes > 12 {
                    self.mes = 1;
                    self.año += 1;
                }
                self.dia = 1;
            } else {
                self.dia += dias;
                dias = 0;
            }
        }
    }

    pub fn restar_dias(&mut self, mut dias: u32) {
        while dias > 0 {
            let dias_en_mes = match self.mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 if self.es_bisiesto() => 29,
                _ => 28,
            };
            if self.dia > dias {
                self.dia -= dias;
                dias = 0;
            } else {
                dias -= self.dia;
                self.mes -= 1;
                if self.mes < 1 {
                    self.mes = 12;
                    self.año -= 1;
                }
                self.dia = match self.mes {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                    4 | 6 | 9 | 11 => 30,
                    2 if self.es_bisiesto() => 29,
                    _ => 28,
                };
            }
        }
    }

    pub fn es_mayor(&self, fecha: &Fecha) -> bool {
        if self.año > fecha.año {
            return true;
        } else if self.año == fecha.año {
            if self.mes > fecha.mes {
                return true;
            } else if self.mes == fecha.mes {
                if self.dia > fecha.dia {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests_2 {
    use super::Fecha;

    #[test]
    fn test_new() {
        let fecha = Fecha::new(1, 1, 2020);
        assert_eq!(fecha.dia, 1);
        assert_eq!(fecha.mes, 1);
        assert_eq!(fecha.año, 2020);
    }

    #[test]
    fn test_es_bisiesto() {
        assert!(Fecha::new(1, 1, 2020).es_bisiesto());
        assert!(!Fecha::new(1, 1, 2019).es_bisiesto());
        assert!(!Fecha::new(1, 1, 1900).es_bisiesto());
        assert!(Fecha::new(1, 1, 2000).es_bisiesto());
    }

    #[test]
    fn test_es_fecha_valida() {
        assert!(Fecha::new(29, 2, 2020).es_fecha_valida());
        assert!(!Fecha::new(29, 2, 2019).es_fecha_valida());
        assert!(Fecha::new(31, 12, 2020).es_fecha_valida());
        assert!(!Fecha::new(31, 4, 2020).es_fecha_valida());
        assert!(!Fecha::new(0, 1, 2020).es_fecha_valida());
        assert!(!Fecha::new(1, 13, 2020).es_fecha_valida());
    }

    #[test]
    fn test_sumar_dias() {
        let mut fecha = Fecha::new(28, 2, 2020);
        fecha.sumar_dias(1);
        assert_eq!(fecha, Fecha::new(29, 2, 2020));

        let mut fecha = Fecha::new(28, 2, 2019);
        fecha.sumar_dias(1);
        assert_eq!(fecha, Fecha::new(1, 3, 2019));

        let mut fecha = Fecha::new(31, 12, 2020);
        fecha.sumar_dias(1);
        assert_eq!(fecha, Fecha::new(1, 1, 2021));

        let mut fecha = Fecha::new(30, 6, 2020);
        fecha.sumar_dias(1);
        assert_eq!(fecha, Fecha::new(1, 7, 2020));

        let mut fecha = Fecha::new(1, 1, 2020);
        fecha.sumar_dias(365);
        assert_eq!(fecha, Fecha::new(31, 12, 2020));
    }

    #[test]
    fn test_restar_dias() {
        let mut fecha = Fecha::new(1, 3, 2020);
        fecha.restar_dias(1);
        assert_eq!(fecha, Fecha::new(29, 2, 2020));

        let mut fecha = Fecha::new(1, 3, 2019);
        fecha.restar_dias(1);
        assert_eq!(fecha, Fecha::new(28, 2, 2019));

        let mut fecha = Fecha::new(1, 1, 2021);
        fecha.restar_dias(1);
        assert_eq!(fecha, Fecha::new(31, 12, 2020));

        let mut fecha = Fecha::new(1, 7, 2020);
        fecha.restar_dias(1);
        assert_eq!(fecha, Fecha::new(30, 6, 2020));

        let mut fecha = Fecha::new(1, 1, 2021);
        fecha.restar_dias(365);
        assert_eq!(fecha, Fecha::new(2, 1, 2020));
    }

    #[test]
    fn test_es_mayor() {
        let fecha1 = Fecha::new(1, 1, 2020);
        let fecha2 = Fecha::new(1, 1, 2019);
        assert!(fecha1.es_mayor(&fecha2));
        assert!(!fecha2.es_mayor(&fecha1));

        let fecha1 = Fecha::new(1, 2, 2020);
        let fecha2 = Fecha::new(1, 1, 2020);
        assert!(fecha1.es_mayor(&fecha2));
        assert!(!fecha2.es_mayor(&fecha1));

        let fecha1 = Fecha::new(2, 1, 2020);
        let fecha2 = Fecha::new(1, 1, 2020);
        assert!(fecha1.es_mayor(&fecha2));
        assert!(!fecha2.es_mayor(&fecha1));
    }
}

//-----------------------------------------------------------
use std::{collections::BTreeMap, fmt::Display, vec};

#[derive(Debug, Clone, PartialEq)]
struct StreamingRust {
    subs: BTreeMap<i32, Usuario>,
}
#[derive(Debug, Clone, PartialEq)]
struct Usuario {
    id: i32,
    sub: (bool, Suscripcion),
    mediopago: MedioPago,
}
#[derive(Debug, Clone, PartialEq)]
struct Suscripcion {
    tipo: TipoSuscripcion,
    fecha_inicio: Fecha,
    paquete_exclusivo: Option<Paquete>,
}
impl Suscripcion {
    fn new(tipo: TipoSuscripcion, fecha_inicio: Fecha) -> Suscripcion {
        Suscripcion {
            tipo,
            fecha_inicio,
            paquete_exclusivo: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Mp {
    usuario: String,
    mediopago: String,
}
#[derive(Debug, Clone, PartialEq)]
struct Tarjeta {
    numero: i32,
    fecha_vencimiento: Fecha,
    cvv: i32,
}
#[derive(Debug, Clone, PartialEq)]
struct Transferencia {
    cbu: i32,
    alias: String,
}
#[derive(Debug, Clone, PartialEq)]
struct Cripto {
    direccion: String,
    cadena: String,
    cripto: String,
}
#[derive(Debug, Clone, PartialEq)]
enum MedioPago {
    Efectivo(f32),
    MercadoPago(Mp),
    Tarjeta(Tarjeta),
    Transferencia(Transferencia),
    Cripto(Cripto),
}
#[derive(Debug, Clone, PartialEq)]
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
        StreamingRust {
            subs: BTreeMap::new(),
        }
    }
    fn crear_usuario(
        &mut self,
        id: i32,
        mediopago: MedioPago,
        tipo: TipoSuscripcion,
    ) -> Result<(), StreamingError> {
        if self.subs.contains_key(&id) {
            return Err(StreamingError::UsuarioExiste);
        } else {
            let mut user = Usuario {
                id,
                sub: (
                    true,
                    Suscripcion {
                        tipo,
                        fecha_inicio: Fecha::new(1, 1, 2024),
                        paquete_exclusivo: None,
                    },
                ),
                mediopago,
            };

            self.subs.insert(id, user);
        }
        Ok(())
    }

    fn upgrade_subscripcion(&mut self, user: Usuario) -> Result<(), StreamingError> {
        // Verificar si el usuario existe en el HashMap de suscripciones
        if let Some(aux) = self.subs.get_mut(&user.id) {
            if aux.sub.0 {
                let nuevo_tipo = match aux.sub.1.tipo {
                    TipoSuscripcion::Basic => TipoSuscripcion::Clasic,
                    TipoSuscripcion::Clasic => TipoSuscripcion::Super,
                    TipoSuscripcion::Super => Err(StreamingError::SubscripcionNoActiva)?,
                };
                // Actualizar el tipo de suscripción
                aux.sub.1.tipo = nuevo_tipo;
            } else {
                return Err(StreamingError::SubscripcionNoActiva);
            }
        } else {
            return Err(StreamingError::UsuarioNoExiste);
        }
        Ok(())
    }

    fn downgrade_subscripcion(&mut self, user: Usuario) -> Result<(), StreamingError> {
        // Verificar si el usuario existe en el HashMap de suscripciones
        if let Some(aux) = self.subs.get_mut(&user.id) {
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
            } else {
                return Err(StreamingError::SubscripcionNoActiva);
            }
        } else {
            return Err(StreamingError::UsuarioNoExiste);
        }
        Ok(())
    }

    fn cancelar_subscripcion(&mut self, user: Usuario) -> Result<(), StreamingError> {
        // Verificar si el usuario existe en el HashMap de suscripciones
        if let Some(aux) = self.subs.get_mut(&user.id) {
            if aux.sub.0 == true {
                aux.sub.0 = false;
            } else {
                return Err(StreamingError::SinSubscripcion);
            }
        } else {
            return Err(StreamingError::UsuarioNoExiste);
        }
        Ok(())
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
    fn contratar_paquete_exclusivo(
        &mut self,
        user_id: i32,
        tipo: TipoPaquete,
    ) -> Result<(), StreamingError> {
        if let Some(user) = self.subs.get_mut(&user_id) {
            if user.sub.0 {
                let paquete = Paquete::new(tipo, user.mediopago.clone());
                user.sub.1.paquete_exclusivo = Some(paquete);
            } else {
                return Err(StreamingError::SubscripcionNoActiva);
            }
        } else {
            return Err(StreamingError::UsuarioNoExiste);
        }
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
//-----------------------------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
enum canales {
    A,
    B,
    C,
    D,
}
#[derive(Debug, Clone, PartialEq)]
struct PeliculasySeries {
    nombre: String,
    año: i32,
    genero: String,
    duracion: i32,
}
#[derive(Debug, Clone, PartialEq)]
struct PerfilUsuario {
    nombre: String,
    es_mayor: bool,
}
impl PerfilUsuario {
    fn new(nombre: String, es_mayor: bool) -> PerfilUsuario {
        PerfilUsuario { nombre, es_mayor }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TipoPaquete {
    PackFutbol(Vec<canales>),
    PackEstrenos(Vec<PeliculasySeries>),
    PackFamiliar([PerfilUsuario; 5]),
}

impl TipoPaquete {
    fn new_pack_futbol() -> TipoPaquete {
        TipoPaquete::PackFutbol(vec![canales::A, canales::B, canales::C, canales::D])
    }
    fn new_pack_estrenos() -> TipoPaquete {
        TipoPaquete::PackEstrenos(vec![
            PeliculasySeries {
                nombre: "Pelicula1".to_string(),
                año: 2021,
                genero: "Accion".to_string(),
                duracion: 120,
            },
            PeliculasySeries {
                nombre: "Pelicula2".to_string(),
                año: 2021,
                genero: "Accion".to_string(),
                duracion: 120,
            },
            PeliculasySeries {
                nombre: "Pelicula3".to_string(),
                año: 2021,
                genero: "Accion".to_string(),
                duracion: 120,
            },
        ])
    }
    fn new_pack_familiar() -> TipoPaquete {
        TipoPaquete::PackFamiliar([
            PerfilUsuario::new("".to_string(), false),
            PerfilUsuario::new("".to_string(), false),
            PerfilUsuario::new("".to_string(), false),
            PerfilUsuario::new("".to_string(), false),
            PerfilUsuario::new("".to_string(), false),
        ])
    }
    fn agregar_nuevo_perfil(
        &mut self,
        nombre: String,
        esmayor: bool,
    ) -> Result<(), StreamingError> {
        let perfil = PerfilUsuario::new(nombre, esmayor);
        match self {
            TipoPaquete::PackFamiliar(perfiles) => {
                for i in 0..perfiles.len() {
                    if perfiles[i].nombre == "" {
                        perfiles[i] = perfil;
                        return Ok(());
                    }
                }
                return Err(StreamingError::UsuarioExiste);
            }
            _ => return Err(StreamingError::Erorrgeneral),
        }
    }
    fn precio(&self) -> f64 {
        match self {
            TipoPaquete::PackFutbol(_) => 500.0,
            TipoPaquete::PackEstrenos(_) => 700.0,
            TipoPaquete::PackFamiliar(_) => 1000.0,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Paquete {
    tipo: TipoPaquete,
    costo: f64,
    medio_de_pago: MedioPago,
}
impl Paquete {
    fn new(tipo: TipoPaquete, medio_de_pago: MedioPago) -> Paquete {
        Paquete {
            tipo: tipo.clone(),
            costo: tipo.precio(),
            medio_de_pago,
        }
    }
}

//-----------------------------------------------------------------------------------------------
#[derive(Debug)]
enum StreamingError {
    UsuarioExiste,
    UsuarioNoExiste,
    SubscripcionNoActiva,
    SinSubscripcion,
    Erorrgeneral,
}
impl Display for StreamingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            StreamingError::UsuarioExiste => write!(f, "El usuario ya existe"),
            StreamingError::UsuarioNoExiste => write!(f, "El usuario no existe"),
            StreamingError::SubscripcionNoActiva => {
                write!(f, "El usuario no tiene suscripción activa")
            }
            StreamingError::SinSubscripcion => write!(f, "El usuario no tiene suscripción"),
            Self::Erorrgeneral => write!(f, "Error general"),
        }
    }
}
//-----------------------------------------------------------------------------------------------
#[test]
fn test_precio() {
    assert_eq!(TipoSuscripcion::Basic.precio(), 10.0);
    assert_eq!(TipoSuscripcion::Clasic.precio(), 20.0);
    assert_eq!(TipoSuscripcion::Super.precio(), 30.0);
    assert_eq!(TipoPaquete::new_pack_futbol().precio(), 500.0);
    assert_eq!(TipoPaquete::new_pack_estrenos().precio(), 700.0);
    assert_eq!(TipoPaquete::new_pack_familiar().precio(), 1000.0);
}

#[test]
fn test_duracion() {
    assert_eq!(TipoSuscripcion::Basic.duracion(), 1);
    assert_eq!(TipoSuscripcion::Clasic.duracion(), 2);
    assert_eq!(TipoSuscripcion::Super.duracion(), 3);
}
#[test]
fn test_constructor_subscripcion() {
    let suscripcion = Suscripcion::new(TipoSuscripcion::Basic, Fecha::new(1, 1, 2024));
    assert_eq!(suscripcion.tipo, TipoSuscripcion::Basic);
    assert_eq!(suscripcion.fecha_inicio, Fecha::new(1, 1, 2024));
    assert_eq!(suscripcion.paquete_exclusivo, None);
}
#[test]
fn test_subscripciones() {
    let mut streaming = StreamingRust::new();
    let tarjeta = Tarjeta {
        numero: 1234,
        fecha_vencimiento: Fecha::new(1, 1, 2024),
        cvv: 123,
    };
    let mut sub = Suscripcion::new(TipoSuscripcion::Basic, Fecha::new(1, 1, 2024));
    let mut user = Usuario {
        id: 1,
        sub: (true, sub),
        mediopago: MedioPago::Tarjeta(tarjeta.clone()),
    };
    streaming.crear_usuario(
        1,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Basic,
    );
    //-----------------------------------------------------------------------
    // aca testeo la creacion de un usuario
    assert_eq!(streaming.subs.get(&1), Some(&user));
    assert!(streaming
        .crear_usuario(1, MedioPago::Tarjeta(tarjeta), TipoSuscripcion::Basic)
        .is_err());
    //-----------------------------------------------------------------------
    // aca testeo el upgrade de una subscripcion
    streaming.upgrade_subscripcion(user.clone());
    user.sub.1.tipo = TipoSuscripcion::Clasic;
    assert_eq!(streaming.subs.get(&1), Some(&user));
    streaming.upgrade_subscripcion(user.clone());
    user.sub.1.tipo = TipoSuscripcion::Super;
    assert_eq!(streaming.subs.get(&1), Some(&user));
    streaming.upgrade_subscripcion(user.clone());
    assert!(streaming.upgrade_subscripcion(user.clone()).is_err());
    user.id = 2;
    assert!(streaming.upgrade_subscripcion(user.clone()).is_err());
    assert!(streaming.cancelar_subscripcion(user.clone()).is_err());
    //-----------------------------------------------------------------------
    // aca testeo el downgrade de una subscripcion
    streaming.downgrade_subscripcion(user.clone());
    user.id = 1;
    streaming.downgrade_subscripcion(user.clone());
    user.sub.1.tipo = TipoSuscripcion::Clasic;
    assert_eq!(streaming.subs.get(&1), Some(&user));
    streaming.downgrade_subscripcion(user.clone());
    user.sub.1.tipo = TipoSuscripcion::Basic;
    assert_eq!(streaming.subs.get(&1), Some(&user));
    streaming.downgrade_subscripcion(user.clone());
    assert!(streaming.downgrade_subscripcion(user.clone()).is_err());
    //-----------------------------------------------------------------------
    //aca testeo cuando el usuario no tiene subscripcion activa
    streaming.cancelar_subscripcion(user.clone());
    assert!(streaming.upgrade_subscripcion(user.clone()).is_err());
    assert!(streaming.downgrade_subscripcion(user.clone()).is_err());
    assert!(streaming.cancelar_subscripcion(user.clone()).is_err());
    //-----------------------------------------------------------------------
}
#[test]
fn test_medios_de_pago() {
    let mut streaming = StreamingRust::new();
    let efectivo = MedioPago::Efectivo(0.0);
    let Transferencia = Transferencia {
        cbu: 1234,
        alias: "alias".to_string(),
    };
    let Cripto = Cripto {
        direccion: "direccion".to_string(),
        cadena: "cadena".to_string(),
        cripto: "cripto".to_string(),
    };
    let tarjeta = Tarjeta {
        numero: 1234,
        fecha_vencimiento: Fecha::new(1, 1, 2024),
        cvv: 123,
    };

    let mercado_pago = Mp {
        usuario: "usuario".to_string(),
        mediopago: "Dinero en Cuenta".to_string(),
    };
    let mut user = Usuario {
        id: 1,
        sub: (
            true,
            Suscripcion {
                tipo: TipoSuscripcion::Basic,
                fecha_inicio: Fecha::new(1, 1, 2024),
                paquete_exclusivo: None,
            },
        ),
        mediopago: MedioPago::Tarjeta(tarjeta.clone()),
    };
    streaming.crear_usuario(
        1,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Basic,
    );
    streaming.crear_usuario(2, efectivo.clone(), TipoSuscripcion::Basic);
    streaming.crear_usuario(
        3,
        MedioPago::MercadoPago(mercado_pago),
        TipoSuscripcion::Basic,
    );

    streaming.crear_usuario(
        4,
        MedioPago::Transferencia(Transferencia.clone()),
        TipoSuscripcion::Basic,
    );

    //-----------------------------------------------------------------------
    streaming.crear_usuario(5, MedioPago::Cripto(Cripto.clone()), TipoSuscripcion::Basic);
    streaming.crear_usuario(7, MedioPago::Cripto(Cripto.clone()), TipoSuscripcion::Basic);
    streaming.crear_usuario(8, MedioPago::Cripto(Cripto.clone()), TipoSuscripcion::Basic);
    streaming.crear_usuario(9, MedioPago::Cripto(Cripto.clone()), TipoSuscripcion::Basic);

    //-----------------------------------------------------------------------
    streaming.crear_usuario(10, efectivo.clone(), TipoSuscripcion::Basic);
    streaming.crear_usuario(11, efectivo.clone(), TipoSuscripcion::Basic);
    streaming.crear_usuario(12, efectivo.clone(), TipoSuscripcion::Basic);
    streaming.crear_usuario(13, efectivo.clone(), TipoSuscripcion::Basic);
    user.id = 10;
    streaming.cancelar_subscripcion(user.clone());
    user.id = 11;
    streaming.cancelar_subscripcion(user.clone());
    user.id = 12;
    streaming.cancelar_subscripcion(user.clone());
    user.id = 13;
    streaming.cancelar_subscripcion(user.clone());

    //-----------------------------------------------------------------------

    // aca testeo el medio de pago mas usado
    assert_eq!(
        streaming.medio_de_pago_mas_usado(),
        MedioPago::Efectivo(0.0)
    );
    //-----------------------------------------------------------------------
    // aca testeo el medio de pago mas usado con subs activas
    assert_eq!(
        streaming.medio_de_pago_mas_usado_subs_activas(),
        MedioPago::Cripto(Cripto.clone())
    );
    //-----------------------------------------------------------------------
}
#[test]
fn test_trait_contador_medio_pago() {
    let mut vec = vec![1, 20, 3, 4, 5];
    assert_eq!(
        vec.retornar_maximo_medio_de_pago(),
        MedioPago::MercadoPago(Mp {
            usuario: "usuario".to_string(),
            mediopago: "mercadopago".to_string()
        })
    );
    vec[2] = 30;
    assert_eq!(
        vec.retornar_maximo_medio_de_pago(),
        MedioPago::Tarjeta(Tarjeta {
            numero: 1234,
            fecha_vencimiento: Fecha::new(1, 1, 2024),
            cvv: 123
        })
    );
    vec[3] = 40;
    assert_eq!(
        vec.retornar_maximo_medio_de_pago(),
        MedioPago::Transferencia(Transferencia {
            cbu: 1234,
            alias: "alias".to_string()
        })
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
                tipo: TipoSuscripcion::Basic,
                fecha_inicio: Fecha::new(1, 1, 2024),
                paquete_exclusivo: None,
            },
        ),
        mediopago: MedioPago::Tarjeta(tarjeta.clone()),
    };
    streaming.crear_usuario(
        1,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Basic,
    );
    streaming.crear_usuario(
        2,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Basic,
    );
    streaming.crear_usuario(
        3,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Basic,
    );
    //-----------------------------------------------------------------------
    streaming.crear_usuario(
        4,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Clasic,
    );
    streaming.crear_usuario(
        5,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Clasic,
    );
    //-----------------------------------------------------------------------
    streaming.crear_usuario(
        6,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Super,
    );
    streaming.crear_usuario(
        7,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Super,
    );
    streaming.crear_usuario(
        8,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Super,
    );
    //-----------------------------------------------------------------------
    // aca testeo la subscripcion mas contratada
    assert_eq!(
        streaming.subscripcion_mas_contratada_subs_activas(),
        TipoSuscripcion::Super
    );
    //-----------------------------------------------------------------------
    streaming.cancelar_subscripcion(user.clone());
    user.id = 2;
    streaming.cancelar_subscripcion(user.clone());
    user.id = 3;
    streaming.cancelar_subscripcion(user.clone());
    streaming.crear_usuario(
        9,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Basic,
    );
    assert_eq!(
        streaming.subscripcion_mas_contratada(),
        TipoSuscripcion::Basic
    );
    //-----------------------------------------------------------------------
}

#[test]
fn test_trait_contador_subscripcion() {
    let mut vec = vec![1, 20, 3];
    assert_eq!(vec.retornar_maximo_subscripcion(), TipoSuscripcion::Clasic);
    vec[2] = 30;
    assert_eq!(vec.retornar_maximo_subscripcion(), TipoSuscripcion::Super);
    vec[0] = 50;
    assert_eq!(vec.retornar_maximo_subscripcion(), TipoSuscripcion::Basic);
}
#[test]
fn test_constructor_paquete() {
    let paquete = Paquete::new(TipoPaquete::new_pack_futbol(), MedioPago::Efectivo(0.0));
    assert_eq!(paquete.costo, 500.0);
    assert_eq!(
        paquete.tipo,
        TipoPaquete::PackFutbol(vec![canales::A, canales::B, canales::C, canales::D])
    );
    assert_eq!(paquete.medio_de_pago, MedioPago::Efectivo(0.0));
}

#[test]
fn test_agregar_nuevo_perfil() {
    let mut pack_familiar = TipoPaquete::new_pack_familiar();
    pack_familiar.agregar_nuevo_perfil("Agustin".to_string(), true);
    pack_familiar.agregar_nuevo_perfil("Tomas".to_string(), false);
    pack_familiar.agregar_nuevo_perfil("Fer".to_string(), true);
    pack_familiar.agregar_nuevo_perfil("Mile".to_string(), false);
    pack_familiar.agregar_nuevo_perfil("Carpincho".to_string(), true);
    assert_eq!(
        pack_familiar,
        TipoPaquete::PackFamiliar([
            PerfilUsuario::new("Agustin".to_string(), true),
            PerfilUsuario::new("Tomas".to_string(), false),
            PerfilUsuario::new("Fer".to_string(), true),
            PerfilUsuario::new("Mile".to_string(), false),
            PerfilUsuario::new("Carpincho".to_string(), true),
        ])
    );
    let mut pack = TipoPaquete::new_pack_futbol();
    assert!(pack_familiar
        .agregar_nuevo_perfil("Agustin".to_string(), true)
        .is_err());
    assert!(pack
        .agregar_nuevo_perfil("Agustin".to_string(), true)
        .is_err());
}
#[test]
fn test_contratar_paquete_exlcusivo() {
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
                tipo: TipoSuscripcion::Basic,
                fecha_inicio: Fecha::new(1, 1, 2024),
                paquete_exclusivo: None,
            },
        ),
        mediopago: MedioPago::Tarjeta(tarjeta.clone()),
    };
    streaming.crear_usuario(
        1,
        MedioPago::Tarjeta(tarjeta.clone()),
        TipoSuscripcion::Basic,
    );
    assert_eq!(streaming.subs.get(&1), Some(&user));
    assert_eq!(streaming.subs.len(), 1);
    streaming.contratar_paquete_exclusivo(1, TipoPaquete::new_pack_futbol());
    let paquete = Paquete::new(
        TipoPaquete::new_pack_futbol(),
        MedioPago::Tarjeta(tarjeta.clone()),
    );
    user.sub.1.paquete_exclusivo = Some(paquete);
    assert_eq!(streaming.subs.get(&1), Some(&user));

    streaming.cancelar_subscripcion(user.clone());
    assert!(streaming
        .contratar_paquete_exclusivo(1, TipoPaquete::new_pack_futbol())
        .is_err());
    assert!(streaming
        .contratar_paquete_exclusivo(2, TipoPaquete::new_pack_futbol())
        .is_err());
}
