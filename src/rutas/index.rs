use axum::{extract::State, response::Html};
use handlebars::Handlebars;

use crate::{bbdd, App};

pub async fn index(State(app): State<App>) -> Html<String> {
    let configuracion;
    match bbdd::select_configuracion::recuperar_configuracion(&app.conexion_bbdd).await {
        Err(error) => return Html(error),
        Ok(ok) => configuracion = ok,
    }

    let index_html = include_str!("../plantillas/index.html");

    let mut hb = Handlebars::new();
    hb.register_template_string("index", index_html).unwrap();
    let respuesta = hb.render("index", &configuracion).unwrap();

    Html(respuesta)
}
