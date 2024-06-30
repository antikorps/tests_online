use std::{fs::create_dir, path::PathBuf};

pub fn crear_carpeta_si_no_existe(ruta: &PathBuf) -> Result<(), String> {
    if ruta.is_dir() {
        return Ok(());
    }

    match create_dir(ruta) {
        Err(error) => {
            let mensaje_error = format!(
                "no se ha podido crear la carpeta en {} {}",
                ruta.display(),
                error
            );
            return Err(mensaje_error);
        }
        Ok(_) => {
            return Ok(());
        }
    }
}
