use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};

use crate::App;

use super::{
    configuracion::configuracion, crear::crear_test, index::index, subir::subir_html_neocities,
};

pub fn gestionar_rutas(app: App) -> Router {
    let router = Router::new()
        .route("/", get(index))
        .route("/configuracion", post(configuracion))
        .route("/crear", post(crear_test))
        .layer(DefaultBodyLimit::max(8000 * 1024))
        .route("/subir", post(subir_html_neocities))
        .with_state(app);
    return router;
}
