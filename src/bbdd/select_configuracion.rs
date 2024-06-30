use serde::Serialize;
use sqlx::{Pool, Sqlite};

#[derive(sqlx::FromRow, Serialize)]
pub struct Configuracion {
    pub chatpdf_api: String,
    pub chatpdf_prompt: String,
    pub chatpdf_preguntas: i64,
    pub neocities_api: String,
}

pub async fn recuperar_configuracion(conexion: &Pool<Sqlite>) -> Result<Configuracion, String> {
    let sql_select_configuracion = r###"
        SELECT chatpdf_api, chatpdf_prompt, chatpdf_preguntas, neocities_api
        FROM configuracion    
    "###;

    let select_configuracion_resultado = sqlx::query_as(sql_select_configuracion)
        .fetch_one(conexion)
        .await;

    match select_configuracion_resultado {
        Err(error) => {
            let mensaje_error = format!("no se ha podido recuperar la configuración {}", error);
            return Err(mensaje_error);
        }
        Ok(ok) => {
            return Ok(ok);
        }
    }
}
