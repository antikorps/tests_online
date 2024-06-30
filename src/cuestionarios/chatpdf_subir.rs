use axum::body::Bytes;
use reqwest::{header, multipart, Client};
use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatPDFSubida {
    pub source_id: String,
}

pub async fn subir_pdf_chatpdf(
    cliente_http: &Client,
    chatpdf_api: &str,
    pdf: Bytes,
) -> Result<String, String> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "x-api-key",
        header::HeaderValue::from_str(&chatpdf_api).unwrap(),
    );

    let pdf_vec: Vec<u8> = pdf.to_vec();

    // Crear el campo de multipart
    let parte;
    match multipart::Part::bytes(pdf_vec)
        .file_name("document.pdf")
        .mime_str("application/pdf")
    {
        Err(error) => {
            let mensaje_error =
                format!("no se ha podido crear el campo para el multipart {}", error);
            return Err(mensaje_error);
        }
        Ok(ok) => parte = ok,
    }

    // Crear la forma de multipart
    let form = multipart::Form::new().part("file", parte);

    let respuesta;
    match cliente_http
        .post("https://api.chatpdf.com/v1/sources/add-file")
        .headers(headers)
        .multipart(form)
        .send()
        .await
    {
        Err(error) => {
            let mensaje_error = format!(
                "ha fallado la peticion de subida del pdf a chatpdf {}",
                error
            );
            return Err(mensaje_error);
        }
        Ok(ok) => respuesta = ok,
    }

    if respuesta.status() != 200 {
        let mensaje_error = format!(
            "la peticion de subida del pdf a chatpdf ha devuelto un status code incorrecto {}",
            respuesta.status()
        );
        return Err(mensaje_error);
    }

    let texto;
    match respuesta.text().await {
        Err(error) => {
            let mensaje_error = format!("no se ha podido obtener el texto de la respuesta de la subida del pdf a chatpdf {}", error);
            return Err(mensaje_error);
        }
        Ok(ok) => texto = ok,
    }

    let chatpdf_subida: ChatPDFSubida;
    match serde_json::from_str(&texto) {
        Err(error) => {
            let mensaje_error = format!(
                "ha fallado la deserializacion de la respuesta de la subida del pdf a chatpdf {}",
                error
            );
            return Err(mensaje_error);
        }
        Ok(ok) => chatpdf_subida = ok,
    }

    Ok(chatpdf_subida.source_id)
}
