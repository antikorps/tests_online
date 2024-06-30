use axum::{
    extract::State,
    http::{Response, StatusCode},
    Json,
};
use serde::Deserialize;

use crate::{bbdd, App};

#[derive(Deserialize)]
pub struct Configuracion {
    chatpdf_api: String,
    neocities_api: String,
}

pub async fn configuracion(
    State(app): State<App>,
    Json(solicitud): Json<Configuracion>,
) -> Response<String> {
    match bbdd::update_configuracion::actualizar_configuracion(
        &app.conexion_bbdd,
        &solicitud.chatpdf_api,
        &solicitud.neocities_api,
    )
    .await
    {
        Err(error) => {
            let mensaje_error = format!(
                "se ha producido un error al actualizar la configuracion: {}",
                error
            );
            let respuesta = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(mensaje_error)
                .unwrap();
            return respuesta;
        }
        Ok(_) => {
            let respuesta = Response::builder()
                .status(200)
                .body("".to_string())
                .unwrap();
            return respuesta;
        }
    }
}
