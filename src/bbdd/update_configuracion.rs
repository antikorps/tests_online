use sqlx::{Pool, Sqlite};

pub async fn actualizar_configuracion(
    conexion: &Pool<Sqlite>,
    chatpdf_api: &str,
    neocities_api: &str,
) -> Result<(), String> {
    let sql_update_configuracion = r###"
    UPDATE configuracion 
    SET 
        chatpdf_api = $1, 
        neocities_api = $2
    WHERE id = 1
"###;

    match sqlx::query(&sql_update_configuracion)
        .bind(chatpdf_api)
        .bind(neocities_api)
        .fetch_all(conexion)
        .await
    {
        Err(error) => {
            let mensaje_error = format!("no ha podido modificarse la configuracion: {}", error);
            return Err(mensaje_error);
        }
        Ok(_) => {
            return Ok(());
        }
    }
}
