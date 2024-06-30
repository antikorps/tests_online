use axum::{
    body::Bytes,
    extract::{Multipart, State},
    http::Response,
};
use reqwest::{header, multipart, Client};
use serde::Deserialize;

use crate::{bbdd, App};

async fn enviar_html_neocities(
    nombre: &str,
    api_token: &str,
    cliente_http: &Client,
    html_bytes: Bytes,
) -> Result<(), String> {
    let bearer = format!("Bearer {}", api_token);
    let mut cabeceras = header::HeaderMap::new();
    cabeceras.insert(
        "Authorization",
        header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let html_vec: Vec<u8> = html_bytes.to_vec();

    // Crear el campo de multipart
    let parte;
    match multipart::Part::bytes(html_vec)
        .file_name(String::from(nombre))
        .mime_str("text/html")
    {
        Err(error) => {
            let mensaje_error =
                format!("no se ha podido crear el campo para el multipart {}", error);
            return Err(mensaje_error);
        }
        Ok(ok) => parte = ok,
    }

    // Crear la forma de multipart
    let form = multipart::Form::new().part(String::from(nombre), parte);

    let respuesta;
    match cliente_http
        .post("https://neocities.org/api/upload")
        .headers(cabeceras)
        .multipart(form)
        .send()
        .await
    {
        Err(error) => {
            let mensaje_error = format!(
                "ha fallado la peticion de subida del html a Neocities {}",
                error
            );
            return Err(mensaje_error);
        }
        Ok(ok) => respuesta = ok,
    }

    if respuesta.status() != 200 {
        let mensaje_error = format!(
            "la peticion de subida del html a neocities ha devuelto un status code incorrecto {}",
            respuesta.status()
        );
        return Err(mensaje_error);
    }

    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeocitiesApiInfo {
    pub result: String,
    pub info: Info,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub sitename: String,
}

pub async fn recuperar_url_neocities(api_token: &str, cliente_http: &Client) -> String {
    let bearer = format!("Bearer {}", api_token);
    let mut cabeceras = header::HeaderMap::new();
    cabeceras.insert(
        "Authorization",
        header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let respuesta;
    match cliente_http
        .get("https://neocities.org/api/info")
        .headers(cabeceras)
        .send()
        .await
    {
        Err(_) => return String::new(),
        Ok(ok) => respuesta = ok,
    }

    if respuesta.status() != 200 {
        return String::new();
    }

    let respuesta_texto;
    match respuesta.text().await {
        Err(_) => return String::new(),
        Ok(ok) => respuesta_texto = ok,
    }

    let neocities_api_info: NeocitiesApiInfo;
    match serde_json::from_str(&respuesta_texto) {
        Err(_) => return String::new(),
        Ok(ok) => neocities_api_info = ok,
    }

    let neocities_api_url = format!("https://{}.neocities.org", neocities_api_info.info.sitename);
    return neocities_api_url;
}

pub async fn subir_html_neocities(
    State(app): State<App>,
    mut multipart: Multipart,
) -> Response<String> {
    let mut nombre_html = String::new();
    let mut bytes_html: Option<Bytes> = None;

    while let Some(campo) = multipart.next_field().await.unwrap() {
        let campo_nombre = campo.name().unwrap().to_string();
        if campo_nombre == "nombre" {
            nombre_html = campo.text().await.unwrap();
            continue;
        }
        if campo_nombre == "archivo" {
            bytes_html = Some(campo.bytes().await.unwrap());
        }
    }

    nombre_html.push_str(".html");

    let configuracion: bbdd::select_configuracion::Configuracion;
    match bbdd::select_configuracion::recuperar_configuracion(&app.conexion_bbdd).await {
        Err(error) => {
            return Response::builder().status(400).body(error).unwrap();
        }
        Ok(ok) => configuracion = ok,
    }

    if configuracion.neocities_api == "" {
        let mensaje_error =
            String::from("para la subida del test debe incorporarse el API token de Neocities");
        return Response::builder().status(400).body(mensaje_error).unwrap();
    }

    match enviar_html_neocities(
        &nombre_html,
        &configuracion.neocities_api,
        &app.cliente_http,
        bytes_html.unwrap(),
    )
    .await
    {
        Err(error) => {
            return Response::builder().status(400).body(error).unwrap();
        }
        Ok(_) => {
            let neocities_api_url =
                recuperar_url_neocities(&configuracion.neocities_api, &app.cliente_http).await;

            let neocities_api_url_completa = format!("{neocities_api_url}/{nombre_html}");
            let mensaje_exito = format!(
                r###"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test autocorregible</title>
</head>
<body>
    <h1>¡Archivo subido correctamente!</h1>
    <p>puedes visitarlo en tu página web a través en <a href="{neocities_api_url_completa}" target="_blank">{neocities_api_url_completa}</a></p>
    <p><a href="/">Volver</a></p>
</body>
</html>"###,
            );
            return Response::builder().status(200).body(mensaje_exito).unwrap();
        }
    }
}
