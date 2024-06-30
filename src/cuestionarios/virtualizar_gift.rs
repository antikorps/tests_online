use std::{fs::File, io::Write, path::PathBuf};

use crate::utilidades::crear_carpeta_si_no_existe;

use super::modelos::TestOnline;

pub fn preparar_archivo_gift(ruta: &PathBuf, test: &TestOnline) -> Result<(), String> {
    match crear_carpeta_si_no_existe(ruta) {
        Err(error) => {
            return Err(error);
        }
        Ok(_) => (),
    }

    let ruta_archivo_test = ruta.join("test_gift.txt");
    let mut archivo;
    match File::create(&ruta_archivo_test) {
        Err(error) => {
            let mensaje_error = format!("no se ha podido crear el archivo test_gift.txt {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => archivo = ok,
    }

    // Escribir el nombre del test como un comentario

    match writeln!(
        archivo,
        "// question: 0  name: Switch category to $course$/top/{}",
        test.nombre
    ) {
        Err(error) => {
            let mensaje_error = format!("error escribiendo en el archivo test_gift.txt {}", error);
            return Err(mensaje_error);
        }
        Ok(_) => (),
    }

    // Para Moodle
    match writeln!(archivo, "$CATEGORY: $course$/top/{}", test.nombre) {
        Err(error) => {
            let mensaje_error = format!("error escribiendo en el archivo test_gift.txt {}", error);
            return Err(mensaje_error);
        }
        Ok(_) => (),
    }

    for (index, pregunta) in test.preguntas.iter().enumerate() {
        // Escribir el encabezado de la pregunta
        match writeln!(
            archivo,
            "\n::Pregunta {} - {}::[html]{}{{",
            index, pregunta.enunciado, pregunta.enunciado
        ) {
            Err(error) => {
                let mensaje_error =
                    format!("error escribiendo en el archivo test_gift.txt {}", error);
                return Err(mensaje_error);
            }
            Ok(_) => (),
        }

        for opcion in &pregunta.opcion {
            if opcion.puntuacion == 100 {
                // Opción correcta
                match writeln!(archivo, "    ={}", opcion.texto) {
                    Err(error) => {
                        let mensaje_error =
                            format!("error escribiendo en el archivo test_gift.txt {}", error);
                        return Err(mensaje_error);
                    }
                    Ok(_) => (),
                }
            } else {
                // Opción incorrecta
                match writeln!(archivo, "    ~{}", opcion.texto) {
                    Err(error) => {
                        let mensaje_error =
                            format!("error escribiendo en el archivo test_gift.txt {}", error);
                        return Err(mensaje_error);
                    }
                    Ok(_) => (),
                }
            }
        }

        match writeln!(archivo, "}}") {
            Err(error) => {
                let mensaje_error =
                    format!("error escribiendo en el archivo test_gift.txt {}", error);
                return Err(mensaje_error);
            }
            Ok(_) => (),
        }
    }

    Ok(())
}
