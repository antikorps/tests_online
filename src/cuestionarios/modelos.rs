use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestOnline {
    pub nombre: String,
    pub preguntas: Vec<TestOnlinePreguntas>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TestOnlinePreguntas {
    pub enunciado: String,
    pub opcion: Vec<TestOnlineOpcion>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TestOnlineOpcion {
    pub texto: String,
    pub puntuacion: i64,
}
