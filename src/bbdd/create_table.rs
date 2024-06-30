use sqlx::{Executor, Pool, Sqlite};

pub async fn crear_tablas_base(conexion_bbdd: &Pool<Sqlite>) {
    let sql_create = r###"
    CREATE TABLE IF NOT EXISTS "configuracion" (
        "id"	INTEGER NOT NULL,
        "chatpdf_api"	TEXT NOT NULL,
        "chatpdf_prompt" TEXT NOT NULL,
        "chatpdf_preguntas" INT NOT NULL,
        "neocities_api"	TEXT NOT NULL,
        PRIMARY KEY("id" AUTOINCREMENT)
    );
    "###;

    conexion_bbdd
        .execute(sqlx::query(sql_create))
        .await
        .expect("ERROR FATAL: no se ha podido crear la tabla configuración");

    let sql_insert = r###"
    INSERT OR IGNORE INTO 
        configuracion (id, chatpdf_api, chatpdf_prompt, chatpdf_preguntas, neocities_api) 
    VALUES (1, $1, $2, $3, $4);
    "###;

    sqlx::query(&sql_insert)
        .bind("")
        .bind("Necesito el máximo de preguntas de opción múltiple con 4 opciones posibles que me ayuden a interiorizar el grueso teórico del contenido del documento. Estas preguntas deben hacer referencia al contenido teórico, a los conceptos, a la información relevante, no quiero preguntas acerca de los títulos, el formato, la página, etc. Es imprescindible que devueltas la respuesta en formato JSON como un array de arrays, sin ningún tipo de clave. Cada array debe corresponder a una pregunta donde el primer elemento es el enunciado, el segundo la opción correcta, el tercero la segunda opción, el cuarto la tercera opción y el quinto la cuarta opción. Tu respuesta posteriormente se serializará en JSON, por lo tanto, es fundamental que cumplas a rajatabla las condiciones del JSON y que esté completa, no se puede cortar la respuesta, el contenido del JSON debe estar completo. Revisa estas cuestiones antes de enviar la respuesta.")
        .bind(5)
        .bind("")
        .fetch_all(conexion_bbdd)
        .await
        .expect("ERROR FATAL: no se ha podido insertar o ignorar los valores por defecto en la tabla configuracion");
}
