use core::hash;
use rand::Rng;
use std::collections::{BTreeMap, HashMap};
#[derive(Debug, PartialEq, Clone)]
struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
    kyc: bool,
    balance_crypto: BTreeMap<String, f32>,
    balance_fiat: f32,
}
trait IniciarBalance {
    fn iniciar_balance() -> BTreeMap<String, f32>;
}

impl IniciarBalance for BTreeMap<String, f32> {
    fn iniciar_balance() -> BTreeMap<String, f32> {
        let mut balance = BTreeMap::new();
        balance.insert("BTC".to_string(), 0.0);
        balance.insert("ETH".to_string(), 0.0);
        balance.insert("USDT".to_string(), 0.0);
        balance
    }
}

impl Usuario {
    fn new(nombre: String, apellido: String, email: String, dni: String) -> Self {
        Usuario {
            nombre,
            apellido,
            email,
            dni,
            kyc: false,
            balance_crypto: BTreeMap::iniciar_balance(),
            balance_fiat: 0.0,
        }
    }
    fn kyc(&mut self) {
        self.kyc = true;
    }
    fn validar_usuario_compra(self, fiat: f32) -> bool {
        if self.balance_fiat >= fiat && self.kyc {
            true
        } else {
            false
        }
    }
    fn validar_usuario_venta(self, monto: f32, cripto: Criptomoneda) -> bool {
        if self.balance_crypto.contains_key(&cripto.prefijo) && self.kyc {
            if self.balance_crypto.get(&cripto.prefijo).unwrap() >= &monto {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn validar_usuario_retiro_fiat(self, monto: f32) -> bool {
        if self.balance_fiat >= monto && self.kyc {
            true
        } else {
            false
        }
    }
    fn ingresar_dinero(&mut self, ingreso: f32) {
        self.balance_fiat += ingreso;
    }
    // esto lo puse para ayudar a testear y no tener que hacer una compra para testear
    fn aumentar_balance_crypto(&mut self, cripto: Criptomoneda, monto: f32) {
        self.balance_crypto
            .entry(cripto.prefijo.clone())
            .and_modify(|c| *c += monto)
            .or_insert(monto);
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Criptomoneda {
    nombre: String,
    prefijo: String,
    listado_blockchain: Vec<Blockchain>,
}
impl Criptomoneda {
    fn new(nombre: String, prefijo: String) -> Self {
        Criptomoneda {
            nombre,
            prefijo,
            listado_blockchain: Vec::new(),
        }
    }

    fn cotizacion(&self) -> f32 {
        match self.prefijo.as_str() {
            "BTC" => 69493.85,
            "ETH" => 3747.45,
            "USDT" => 1.0,
            _ => 0.0,
        }
    }

    fn verificar_blockchain(&self, blockchain: Blockchain) -> bool {
        self.listado_blockchain.contains(&blockchain)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}
impl Blockchain {
    fn new(nombre: String, prefijo: String) -> Self {
        Blockchain { nombre, prefijo }
    }

    fn generar_hash(&self) -> Hash {
        let hash = Hash::new(self.nombre.clone());
        return hash;
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Hash {
    nombre_blockchain: String,
    hash: i32,
}
impl Hash {
    fn new(nombre_blockchain: String) -> Self {
        let mut rng = rand::thread_rng();
        Hash {
            nombre_blockchain,
            hash: rng.gen_range(0..1000),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Transaccion {
    usuario: Usuario,
    cripto: Criptomoneda,
    cotizacion: f64,
    blockchain: Blockchain,
    hash: Hash,
    monto: f64,
    tipo: String,
    fecha: String,
}
impl Transaccion {
    fn transaccion_ingreso(fecha: String, tipo: String, monto: f64, user: Usuario) -> Self {
        Transaccion {
            usuario: user,
            cripto: Criptomoneda::new("".to_string(), "".to_string()),
            cotizacion: 0.0,
            blockchain: Blockchain {
                nombre: "".to_string(),
                prefijo: "".to_string(),
            },
            hash: Hash {
                nombre_blockchain: "".to_string(),
                hash: 0,
            },
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }

    fn transaccion_compra(
        fecha: String,
        user: Usuario,
        cripto: Criptomoneda,
        tipo: String,
        monto: f64,
        cotizacion: f64,
    ) -> Self {
        Transaccion {
            usuario: user,
            cripto: cripto,
            cotizacion: cotizacion,
            blockchain: Blockchain {
                nombre: "".to_string(),
                prefijo: "".to_string(),
            },
            hash: Hash {
                nombre_blockchain: "".to_string(),
                hash: 0,
            },
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }

    fn transaccion_venta(
        fecha: String,
        user: Usuario,
        cripto: Criptomoneda,
        tipo: String,
        monto: f64,
        cotizacion: f64,
    ) -> Self {
        Transaccion {
            usuario: user,
            cripto: cripto,
            cotizacion: cotizacion,
            blockchain: Blockchain {
                nombre: "".to_string(),
                prefijo: "".to_string(),
            },
            hash: Hash {
                nombre_blockchain: "".to_string(),
                hash: 0,
            },
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }

    fn transaccion_retiro(
        fecha: String,
        user: Usuario,
        tipo: String,
        blockchain: Blockchain,
        hash: Hash,
        monto: f64,
        cotizacion: f64,
    ) -> Self {
        Transaccion {
            usuario: user,
            cripto: Criptomoneda::new("".to_string(), "".to_string()),
            cotizacion: cotizacion,
            blockchain: blockchain,
            hash: hash,
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }

    fn transaccion_recibir(
        fecha: String,
        user: Usuario,
        tipo: String,
        blockchain: Blockchain,
        cripto: Criptomoneda,
        monto: f64,
        cotizacion: f64,
    ) -> Self {
        Transaccion {
            usuario: user,
            cripto: cripto,
            cotizacion: cotizacion,
            blockchain: blockchain,
            hash: Hash {
                nombre_blockchain: "".to_string(),
                hash: 0,
            },
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }

    fn transaccion_retirar_fiat(
        fecha: String,
        user: Usuario,
        tipo: String,
        monto: f64,
        medio: MedioPago,
    ) -> Self {
        Transaccion {
            usuario: user,
            cripto: Criptomoneda::new("".to_string(), "".to_string()),
            cotizacion: 0.0,
            blockchain: Blockchain {
                nombre: "".to_string(),
                prefijo: "".to_string(),
            },
            hash: Hash {
                nombre_blockchain: "".to_string(),
                hash: 0,
            },
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }
}
enum MedioPago {
    MercadoPago,
    TransfenciaBancaria,
}
struct XYZ {
    usuarios: BTreeMap<String, Usuario>,
    transacciones: Vec<Transaccion>,
}

impl XYZ {
    fn new() -> Self {
        XYZ {
            usuarios: BTreeMap::new(),
            transacciones: Vec::new(),
        }
    }
    fn ingresar_dinero(&mut self, user: Usuario, ingreso: f32) {
        self.usuarios.get_mut(&user.dni).unwrap().balance_fiat += ingreso;
        let transaccion = Transaccion::transaccion_ingreso(
            "fecha".to_string(),
            "ingreso".to_string(),
            ingreso as f64,
            user,
        );
        self.transacciones.push(transaccion);
    }

    fn comprar_cripto(&mut self, user: Usuario, fiat: f32, cripto: Criptomoneda) {
        if user.clone().validar_usuario_compra(fiat) {
            let cant_crypto = fiat / cripto.cotizacion();
            let usuario = self.usuarios.entry(user.dni.clone());

            usuario
                .and_modify(|u| u.balance_fiat -= fiat)
                .and_modify(|u| {
                    u.balance_crypto
                        .entry(cripto.prefijo.clone())
                        .and_modify(|c| *c += cant_crypto);
                });
            let transaccion = Transaccion::transaccion_compra(
                "fecha".to_string(),
                user,
                cripto.clone(),
                "compra cripto".to_string(),
                cant_crypto as f64,
                cripto.cotizacion() as f64,
            );
            self.transacciones.push(transaccion);
        }
    }

    fn vender_cripto(&mut self, user: Usuario, monto: f32, cripto: Criptomoneda) {
        if user.clone().validar_usuario_venta(monto, cripto.clone()) {
            let cant_fiat = monto * cripto.cotizacion();
            let usuario = self.usuarios.entry(user.dni.clone());
            usuario
                .and_modify(|u| {
                    u.balance_crypto
                        .entry(cripto.prefijo.clone())
                        .and_modify(|c| *c -= monto);
                })
                .and_modify(|u| u.balance_fiat += cant_fiat);

            let transaccion = Transaccion::transaccion_venta(
                "fecha".to_string(),
                user,
                cripto.clone(),
                "venta cripto".to_string(),
                monto as f64,
                cripto.cotizacion() as f64,
            );
            self.transacciones.push(transaccion);
        }
    }

    fn retirar_cripto(
        &mut self,
        user: Usuario,
        monto: f32,
        cripto: Criptomoneda,
        blockchain: Blockchain,
    ) {
        if user.clone().validar_usuario_venta(monto, cripto.clone())
            && cripto.verificar_blockchain(blockchain.clone())
        {
            let usuario = self.usuarios.entry(user.dni.clone());
            usuario.and_modify(|u| {
                u.balance_crypto
                    .entry(cripto.prefijo.clone())
                    .and_modify(|c| *c -= monto);
            });

            let hash = blockchain.generar_hash();
            let transaccion = Transaccion::transaccion_retiro(
                "fecha".to_string(),
                user,
                "retiro".to_string(),
                blockchain,
                hash,
                monto as f64,
                cripto.cotizacion() as f64,
            );
            self.transacciones.push(transaccion);
        }
    }

    fn recibir_cripto(
        &mut self,
        user: Usuario,
        monto: f32,
        cripto: Criptomoneda,
        blockchain: Blockchain,
    ) {
        if cripto.verificar_blockchain(blockchain.clone()) {
            let usuario = self.usuarios.entry(user.dni.clone());
            usuario.and_modify(|u| {
                u.balance_crypto
                    .entry(cripto.prefijo.clone())
                    .and_modify(|c| *c += monto);
            });

            let cotizacion = cripto.cotizacion();
            let transaccion = Transaccion::transaccion_recibir(
                "".to_string(),
                user,
                "".to_string(),
                blockchain,
                cripto,
                monto as f64,
                cotizacion as f64,
            );
            self.transacciones.push(transaccion);
        }
    }

    fn retirar_fiat(&mut self, monto: f32, user: Usuario) {
        if user.clone().validar_usuario_retiro_fiat(monto) {
            let usuario = self.usuarios.entry(user.dni.clone());
            usuario.and_modify(|u| u.balance_fiat -= monto);

            let transaccion = Transaccion::transaccion_retirar_fiat(
                "".to_string(),
                user,
                "".to_string(),
                monto as f64,
                MedioPago::MercadoPago,
            );
            self.transacciones.push(transaccion);
        }
    }

    fn cripto_mas_vendida(&self) -> String {
        let mut contador_cripto: BTreeMap<String, u32> = BTreeMap::new();

        for venta in self.transacciones.clone() {
            if venta.tipo == "venta cripto" {
                let cant = contador_cripto
                    .entry(venta.cripto.prefijo.clone())
                    .or_insert(0);
                *cant += 1;
            }
        }

        let mut max_cripto = None;
        let mut max_value = 0;

        for (cripto, &value) in &contador_cripto {
            if value > max_value {
                max_value = value;
                max_cripto = Some(cripto);
            }
        }

        if let Some(cripto) = max_cripto {
            return cripto.clone();
        } else {
            return "".to_string();
        }
    }

    fn cripto_mas_comprada(&self) -> String {
        let mut contador_cripto: BTreeMap<String, u32> = BTreeMap::new();

        for compra in self.transacciones.clone() {
            if compra.tipo == "compra cripto" {
                let cant = contador_cripto
                    .entry(compra.cripto.prefijo.clone())
                    .or_insert(0);
                *cant += 1;
            }
        }

        let mut max_cripto = None;
        let mut max_value = 0;

        for (cripto, &value) in &contador_cripto {
            if value > max_value {
                max_value = value;
                max_cripto = Some(cripto);
            }
        }

        if let Some(cripto) = max_cripto {
            return cripto.clone();
        } else {
            return "".to_string();
        }
    }

    fn cripto_mas_volumen_venta(&self) -> String {
        let mut contador_cripto: BTreeMap<String, f64> = BTreeMap::new();
        for venta in self.transacciones.clone() {
            if venta.tipo == "venta cripto" {
                let cant = contador_cripto
                    .entry(venta.cripto.prefijo.clone())
                    .or_insert(0.0);
                *cant += venta.monto;
            }
        }
        let mut max_cripto = None;
        let mut max_value = 0.0;
        for (cripto, &value) in &contador_cripto {
            if value > max_value {
                max_value = value;
                max_cripto = Some(cripto);
            }
        }
        if let Some(cripto) = max_cripto {
            return cripto.clone();
        } else {
            return "".to_string();
        }
    }
    fn cripto_mas_volumen_compra(&self) -> String {
        let mut contador_cripto: BTreeMap<String, f64> = BTreeMap::new();
        for compra in self.transacciones.clone() {
            if compra.tipo == "compra cripto" {
                let cant = contador_cripto
                    .entry(compra.cripto.prefijo.clone())
                    .or_insert(0.0);
                *cant += compra.monto;
            }
        }
        let mut max_cripto = None;
        let mut max_value = 0.0;
        for (cripto, &value) in &contador_cripto {
            if value > max_value {
                max_value = value;
                max_cripto = Some(cripto);
            }
        }
        if let Some(cripto) = max_cripto {
            return cripto.clone();
        } else {
            return "".to_string();
        }
    }

    fn pushear_transaccion(&mut self, t: Transaccion) {
        self.transacciones.push(t);
    }
}

#[test]
fn test_validar_usuario() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    assert_eq!(usuario.kyc, false);
    usuario.kyc();
    assert_eq!(usuario.kyc, true);

    usuario.ingresar_dinero(100.0);
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 500.0);

    assert_eq!(usuario.clone().validar_usuario_compra(100.0), true);
    assert_eq!(usuario.clone().validar_usuario_venta(100.0, cripto), true);
    assert_eq!(usuario.validar_usuario_retiro_fiat(100.0), true);
}

#[test]
fn test_transacciones() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let blockchain = Blockchain::new("Bitcoin".to_string(), "BTC".to_string());

    let transaccion_ingreso = Transaccion::transaccion_ingreso(
        "fecha".to_string(),
        "ingreso".to_string(),
        100.0,
        usuario.clone(),
    );
    let transaccion_compra = Transaccion::transaccion_compra(
        "fecha".to_string(),
        usuario.clone(),
        cripto.clone(),
        "compra cripto".to_string(),
        100.0,
        cripto.cotizacion() as f64,
    );

    let transaccion_venta = Transaccion::transaccion_venta(
        "fecha".to_string(),
        usuario.clone(),
        cripto.clone(),
        "venta cripto".to_string(),
        100.0,
        cripto.cotizacion() as f64,
    );

    let transaccion_retiro = Transaccion::transaccion_retiro(
        "fecha".to_string(),
        usuario.clone(),
        "retiro".to_string(),
        blockchain.clone(),
        blockchain.generar_hash(),
        100.0,
        cripto.cotizacion() as f64,
    );

    let transaccion_recibir = Transaccion::transaccion_recibir(
        "fecha".to_string(),
        usuario.clone(),
        "recibir".to_string(),
        blockchain.clone(),
        cripto.clone(),
        100.0,
        cripto.cotizacion() as f64,
    );
    let transaccion_retirar_fiat = Transaccion::transaccion_retirar_fiat(
        "fecha".to_string(),
        usuario.clone(),
        "retirar".to_string(),
        100.0,
        MedioPago::MercadoPago,
    );

    plataforma.pushear_transaccion(transaccion_ingreso.clone());
    plataforma.pushear_transaccion(transaccion_compra.clone());
    plataforma.pushear_transaccion(transaccion_venta.clone());
    plataforma.pushear_transaccion(transaccion_retiro.clone());
    plataforma.pushear_transaccion(transaccion_recibir.clone());
    plataforma.pushear_transaccion(transaccion_retirar_fiat.clone());

    assert_eq!(plataforma.transacciones.len(), 6);
    assert_eq!(plataforma.transacciones[0], transaccion_ingreso);
    assert_eq!(plataforma.transacciones[1], transaccion_compra);
    assert_eq!(plataforma.transacciones[2], transaccion_venta);
    assert_eq!(plataforma.transacciones[3], transaccion_retiro);
    assert_eq!(plataforma.transacciones[4], transaccion_recibir);
    assert_eq!(plataforma.transacciones[5], transaccion_retirar_fiat);
}

#[test]
fn test_generar_hash() {
    let blockchain = Blockchain::new("Tron".to_string(), "TRC20".to_string());
    let hash = blockchain.generar_hash();
    assert_eq!(hash.nombre_blockchain, "Tron".to_string());
    assert!(hash.hash >= 0 && hash.hash < 1000);
}

#[test]
fn test_ingresar_dinero() {
    let usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    let mut plataforma = XYZ::new();
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.ingresar_dinero(usuario.clone(), 100.0);
    assert_eq!(
        plataforma.usuarios.get(&usuario.dni).unwrap().balance_fiat,
        100.0
    );
}

#[test]
fn test_comprar_cripto() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );

    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.comprar_cripto(usuario.clone(), 100.0, cripto.clone());
    let user = plataforma.usuarios.get(&usuario.dni).unwrap();
    let balance_cripto = user.balance_crypto.get(&cripto.prefijo).unwrap();
    assert_eq!(user.balance_fiat, 900.0);
    assert_eq!(*balance_cripto, 100.0 / cripto.cotizacion());
    assert_eq!(plataforma.transacciones.len(), 1);
}

#[test]
fn test_vender_cripto() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());

    let user = plataforma.usuarios.get(&usuario.dni).unwrap();
    let balance_cripto = user.balance_crypto.get(&cripto.prefijo).unwrap();

    assert_eq!(user.balance_fiat, 1000.0 + 50.0 * cripto.cotizacion());
    assert_eq!(*balance_cripto, 50.0);
    assert_eq!(plataforma.transacciones.len(), 1);
}

#[test]
fn test_retirar_cripto() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let mut cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let blockchain = Blockchain::new("BNB Smart Chain".to_string(), "BEP20".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    cripto.listado_blockchain.push(blockchain.clone());
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.retirar_cripto(usuario.clone(), 50.0, cripto.clone(), blockchain.clone());

    let user = plataforma.usuarios.get(&usuario.dni).unwrap();
    let balance_cripto = user.balance_crypto.get(&cripto.prefijo).unwrap();

    assert_eq!(user.balance_fiat, 1000.0);
    assert_eq!(*balance_cripto, 50.0);
    assert_eq!(plataforma.transacciones.len(), 1);
}

#[test]
fn test_recibir_cripto() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let mut cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let blockchain = Blockchain::new("BNB Smart Chain".to_string(), "BEP20".to_string());
    cripto.listado_blockchain.push(blockchain.clone());
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());

    let mut user = plataforma.usuarios.get(&usuario.dni).unwrap();
    let mut balance_cripto = user.balance_crypto.get(&cripto.prefijo).unwrap();

    assert_eq!(*balance_cripto, 0.0);
    plataforma.recibir_cripto(usuario.clone(), 50.0, cripto.clone(), blockchain.clone());

    user = plataforma.usuarios.get(&usuario.dni).unwrap();
    balance_cripto = user.balance_crypto.get(&cripto.prefijo).unwrap();

    assert_eq!(*balance_cripto, 50.0);
    assert_eq!(plataforma.transacciones.len(), 1);
}

#[test]
fn test_retirar_fiat() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.retirar_fiat(100.0, usuario.clone());

    let user = plataforma.usuarios.get(&usuario.dni).unwrap();
    assert_eq!(user.balance_fiat, 900.0);
    assert_eq!(plataforma.transacciones.len(), 1);
}

#[test]
fn test_cripto_mas_vendida() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let cripto2 = Criptomoneda::new("Ethereum".to_string(), "ETH".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    usuario.aumentar_balance_crypto(cripto2.clone(), 100.0);
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());

    assert_eq!(plataforma.cripto_mas_vendida(), "BTC".to_string());
}

#[test]
fn test_cripto_mas_comprada() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let cripto2 = Criptomoneda::new("Ethereum".to_string(), "ETH".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    usuario.aumentar_balance_crypto(cripto2.clone(), 100.0);
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());

    assert_eq!(plataforma.cripto_mas_comprada(), "BTC".to_string());
}

#[test]
fn test_cripto_mas_volumen_venta() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let cripto2 = Criptomoneda::new("Ethereum".to_string(), "ETH".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    usuario.aumentar_balance_crypto(cripto2.clone(), 100.0);
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());

    assert_eq!(plataforma.cripto_mas_volumen_venta(), "BTC".to_string());
}

#[test]
fn test_cripto_mas_volumen_compra() {
    let mut usuario = Usuario::new(
        "Agustin".to_string(),
        "Castillo".to_string(),
        " ".to_string(),
        "77777777".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let cripto2 = Criptomoneda::new("Ethereum".to_string(), "ETH".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    usuario.aumentar_balance_crypto(cripto2.clone(), 100.0);
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());

    assert_eq!(plataforma.cripto_mas_volumen_compra(), "ETH".to_string());
}
