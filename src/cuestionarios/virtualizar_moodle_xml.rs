use std::{fs::File, io::Write, path::PathBuf};

use handlebars::Handlebars;

use crate::utilidades::crear_carpeta_si_no_existe;

use super::modelos::TestOnline;

pub fn preparar_archivo_moodle_xml(ruta: &PathBuf, test: &TestOnline) -> Result<(), String> {
    match crear_carpeta_si_no_existe(ruta) {
        Err(error) => {
            return Err(error);
        }
        Ok(_) => (),
    }

    let test_online_plantilla = include_str!("../plantillas/moodle_xml.xml");

    let mut hb = Handlebars::new();
    match hb.register_template_string("moodle_xml", test_online_plantilla) {
        Err(error) => {
            let mensaje_error =
                format!("no se ha registrar la plantilla para el archivo moodle_xml {error}");
            return Err(mensaje_error);
        }
        Ok(_) => (),
    }
    let test_generado;
    match hb.render("moodle_xml", &test) {
        Err(error) => {
            let mensaje_error =
                format!("no se ha renderizar la plantilla para el archivo moodle_xml {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => test_generado = ok,
    }

    let ruta_archivo_test = ruta.join("moodle_xml.xml");
    let mut archivo;
    match File::create(&ruta_archivo_test) {
        Err(error) => {
            let mensaje_error = format!("no se ha podido crear el archivo moodle_xml.xml {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => archivo = ok,
    }

    match archivo.write(test_generado.as_bytes()) {
        Err(error) => {
            let mensaje_error = format!(
                "no se ha podido incorporar el contenido al archivo moodle_xml.xml {error}"
            );
            return Err(mensaje_error);
        }
        Ok(_) => (),
    }
    Ok(())
}
