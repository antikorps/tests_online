use std::{fs::File, path::Path};

use sqlx::{Pool, Sqlite};

pub async fn conectar_bbdd(ruta_raiz: &Path) -> Pool<Sqlite> {
    let ruta_base_datos = ruta_raiz.join("tests_online.sqlite");
    if !&ruta_base_datos.is_file() {
        File::create(&ruta_base_datos)
            .expect("ERROR FATAL: no se ha podido crear el archivo tests_online.sqlite");
    }

    let conexion = format!("sqlite:{}", ruta_base_datos.as_path().to_str().unwrap());
    return sqlx::sqlite::SqlitePool::connect(&conexion)
        .await
        .expect("ERROR FATAL: no ha podido establecerse una conexión sqlite");
}
