use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

use super::modelos::{TestOnline, TestOnlineOpcion, TestOnlinePreguntas};

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatPDFSolicitud {
    pub source_id: String,
    pub messages: Vec<ChatPDFMensaje>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatPDFMensaje {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatPDFContent {
    pub content: String,
}

pub async fn preguntar_chat_pdf(
    cliente_http: &Client,
    chatpdf_api: &str,
    chatpdf_source_id: &str,
    prompt: &str,
    nombre_archivo: &str,
) -> Result<TestOnline, String> {
    let chatpdf_mensaje = ChatPDFMensaje {
        role: String::from("user"),
        content: String::from(prompt),
    };

    let chatpdf_solicitud = ChatPDFSolicitud {
        source_id: String::from(chatpdf_source_id),
        messages: vec![chatpdf_mensaje],
    };

    let chatpdf_solicitud_json;
    match serde_json::to_string(&chatpdf_solicitud) {
        Err(error) => {
            let mensaje_error = format!(
                "ha fallado la serializacion en json de la solicitud de conversacion a chatpdf {}",
                error
            );
            return Err(mensaje_error);
        }
        Ok(ok) => chatpdf_solicitud_json = ok,
    }

    let mut cabeceras = header::HeaderMap::new();
    cabeceras.insert("x-api-key", chatpdf_api.parse().unwrap());
    cabeceras.insert("Content-Type", "application/json".parse().unwrap());

    let respuesta;
    match cliente_http
        .post("https://api.chatpdf.com/v1/chats/message")
        .headers(cabeceras)
        .body(chatpdf_solicitud_json)
        .send()
        .await
    {
        Err(error) => {
            let mensaje_error = format!(
                "ha fallado la respuesta de la solicitud de conversacion a chatpdf {}",
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

    let chatpdf_contenido: ChatPDFContent;
    match serde_json::from_str(&texto) {
        Err(error) => {
            let mensaje_error = format!("no se ha podido deserializar el texto de la respuesta de la subida del pdf a chatpdf {}", error);
            return Err(mensaje_error);
        }
        Ok(ok) => chatpdf_contenido = ok,
    }

    let chatpdf_contenido_test: Vec<Vec<String>>;
    match serde_json::from_str(&chatpdf_contenido.content) {
        Err(error) => {
            let mensaje_error = format!(
                "no se ha podido deserializar el json devuelto en content por chatpdf {}",
                error
            );
            return Err(mensaje_error);
        }
        Ok(ok) => chatpdf_contenido_test = ok,
    }

    let mut test_preguntas = Vec::new();

    for test_data in &chatpdf_contenido_test {
        if test_data.len() != 5 {
            continue;
        }
        let mut opciones = vec![
            TestOnlineOpcion {
                texto: String::from(&test_data[1]),
                puntuacion: 100,
            },
            TestOnlineOpcion {
                texto: String::from(&test_data[2]),
                puntuacion: 0,
            },
            TestOnlineOpcion {
                texto: String::from(&test_data[3]),
                puntuacion: 0,
            },
            TestOnlineOpcion {
                texto: String::from(&test_data[4]),
                puntuacion: 0,
            },
        ];
        let mut rng = thread_rng();
        opciones.shuffle(&mut rng);

        let test_pregunta = TestOnlinePreguntas {
            enunciado: String::from(&test_data[0]),
            opcion: opciones,
        };

        test_preguntas.push(test_pregunta);
    }

    Ok(TestOnline {
        nombre: String::from(nombre_archivo),
        preguntas: test_preguntas,
    })
}
