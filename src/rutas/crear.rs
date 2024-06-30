use axum::{
    body::Bytes,
    extract::{Multipart, State},
    response::Response,
};

use crate::{
    bbdd,
    cuestionarios::{
        chatpdf_preguntar::preguntar_chat_pdf, chatpdf_subir::subir_pdf_chatpdf,
        virtualizar_gift::preparar_archivo_gift,
        virtualizar_moodle_xml::preparar_archivo_moodle_xml,
        vitualizar_html::preparar_archivo_html,
    },
    utilidades::crear_carpeta_si_no_existe,
    App,
};

pub async fn crear_test(State(app): State<App>, mut multipart: Multipart) -> Response<String> {
    let mut nombre_archivo = String::new();
    let mut bytes_pdf: Option<Bytes> = None;

    while let Some(campo) = multipart.next_field().await.unwrap() {
        let campo_nombre = campo.name().unwrap().to_string();
        if campo_nombre == "nombre" {
            nombre_archivo = campo.text().await.unwrap();
            continue;
        }
        if campo_nombre == "pdf" {
            bytes_pdf = Some(campo.bytes().await.unwrap());
        }
    }

    let configuracion: bbdd::select_configuracion::Configuracion;
    match bbdd::select_configuracion::recuperar_configuracion(&app.conexion_bbdd).await {
        Err(error) => {
            return Response::builder().status(400).body(error).unwrap();
        }
        Ok(ok) => configuracion = ok,
    }

    if configuracion.chatpdf_api == "" || configuracion.chatpdf_prompt == "" {
        let mensaje_error = String::from(
            "Sin el API token de ChatPDF o el prompt no es posible realizar el test online",
        );
        return Response::builder().status(400).body(mensaje_error).unwrap();
    }

    let chatpdf_source_id;
    match subir_pdf_chatpdf(
        &app.cliente_http,
        &configuracion.chatpdf_api,
        bytes_pdf.unwrap(),
    )
    .await
    {
        Err(error) => {
            return Response::builder().status(400).body(error).unwrap();
        }
        Ok(ok) => chatpdf_source_id = ok,
    }

    let mut test;
    match preguntar_chat_pdf(
        &app.cliente_http,
        &configuracion.chatpdf_api,
        &chatpdf_source_id,
        &configuracion.chatpdf_prompt,
        &nombre_archivo,
    )
    .await
    {
        Err(error) => {
            return Response::builder().status(400).body(error).unwrap();
        }
        Ok(ok) => test = ok,
    }

    for _ in 1..3 {
        if test.preguntas.len() >= 10 {
            break;
        }
        let mas_preguntas_prompt = format!("Genérame más preguntas diferentes. {}. Si aparecen preguntas repetidas no las incluyas en el JSON.", &configuracion.chatpdf_prompt);

        let mas_preguntas = preguntar_chat_pdf(
            &app.cliente_http,
            &configuracion.chatpdf_api,
            &chatpdf_source_id,
            &mas_preguntas_prompt,
            &nombre_archivo,
        )
        .await;

        if mas_preguntas.is_err() {
            /* El error más cómun es que haya fallado la deserialización
            porque el JSON lo hayan devuelto mal
            En realidad poco se puede hacer y hay un límite de pasadas
            así que sin pasando */
            continue;
        }

        for p in mas_preguntas.unwrap().preguntas {
            test.preguntas.push(p)
        }
    }

    // CREAR ARCHIVOS
    // ASEGURAR LA CARPETA TEMPORAL
    match crear_carpeta_si_no_existe(&app.ruta_online_tests) {
        Err(error) => {
            return Response::builder().status(400).body(error).unwrap();
        }
        Ok(_) => (),
    }
    let ruta_test_online_nombre = app.ruta_online_tests.join(&nombre_archivo);

    match preparar_archivo_html(&ruta_test_online_nombre, &test) {
        Err(error) => {
            return Response::builder().status(400).body(error).unwrap();
        }
        Ok(_) => (),
    }

    match preparar_archivo_moodle_xml(&ruta_test_online_nombre, &test) {
        Err(error) => {
            return Response::builder().status(400).body(error).unwrap();
        }
        Ok(_) => (),
    }

    match preparar_archivo_gift(&ruta_test_online_nombre, &test) {
        Err(error) => {
            return Response::builder().status(400).body(error).unwrap();
        }
        Ok(_) => (),
    }

    let mensaje_exito = format!(
        r###"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test autocorregible</title>
</head>
<body>
    <h1>¡Virtualización satisfactoria!</h1>
    <p>Tests disponibles en {}</p>
    <p><a href="/">Volver</a></p>
</body>
</html>"###,
        ruta_test_online_nombre.display()
    );
    return Response::builder().status(200).body(mensaje_exito).unwrap();
}
