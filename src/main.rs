use std::{env::current_exe, path::PathBuf};

use reqwest::Client;
use rutas::_router::gestionar_rutas;
use sqlx::{Pool, Sqlite};

mod bbdd;
mod cliente_http;
mod cuestionarios;
mod rutas;
mod utilidades;

#[derive(Clone)]
pub struct App {
    conexion_bbdd: Pool<Sqlite>,
    ruta_online_tests: PathBuf,
    cliente_http: Client,
}

#[tokio::main]
async fn main() {
    let ruta_ejecutable =
        current_exe().expect("ERROR FATAL: no se ha podido encontrar la ruta del ejecutable");
    let ruta_raiz = ruta_ejecutable
        .parent()
        .expect("ERROR FATAL: no ha podido encontrarse el directorio del ejecutable");

    let conexion_bbdd = bbdd::conectar::conectar_bbdd(ruta_raiz).await;
    bbdd::create_table::crear_tablas_base(&conexion_bbdd).await;

    let cliente_http = cliente_http::crear_cliente_http().await;

    let ruta_online_tests = ruta_raiz.join("mis_tests");

    let app = App {
        conexion_bbdd,
        ruta_online_tests,
        cliente_http,
    };

    let router = gestionar_rutas(app);

    for puerto in 3000..8000 {
        let direccion = format!("0.0.0.0:{puerto}");
        match tokio::net::TcpListener::bind(direccion).await {
            Err(_) => continue,
            Ok(ok) => {
                println!("Aplicación web iniciada en http://localhost:{puerto}");
                axum::serve(ok, router.clone()).await.unwrap();
            }
        }
    }
}
