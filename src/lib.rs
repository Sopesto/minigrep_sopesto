//! # Minigrep
//! `minigrip` es una aplicación hecha siguiendo la guía del libro *The Rust Programming Lenguage*.  
//! La misma busca recrear de forma minimalista la aplicación `grep`.
use std::error::Error;
use std::{env, fs};

///Estructura para la configuración y transpaso de información hacia las funciones.
/// # Definición
///```
///pub struct Config{
///  pub consulta: String,
///  pub ruta:     String,
///  pub distinguir: bool,
///}
///```
pub struct Config{
  pub consulta: String,
  pub ruta:     String,
  pub distinguir: bool,
}

impl Config{
  ///Inicializador de la estructura Config.
  ///# Ejemplo
  ///```
  ///use std::env;
  ///use std::process;
  ///use minigrip::Config;
  /// 
  ///let config = Config::build(env::args()).unwrap_or_else(|err|{
  ///  eprintln!("Problema pasando argumentos: {err}");
  ///  process::exit(1);
  ///});
  ///``` 
  pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{
    args.next();

    let consulta = match args.next(){
      Some(arg) => arg,
      None => return Err("No se especificó el texto de búsqueda"),
    };
    let ruta = match args.next(){
      Some(arg) => arg,
      None => return Err("No se especificó la ruta del archivo"),
    };

    let distinguir = env::var("IGNORE_CASE").is_ok();
    
    Ok(Config{consulta, ruta, distinguir})
  }
}

///Ejecuta la lógica del programa.
///# Ejemplo
///```
///use std::env;
///use std::process;
///use minigrip::Config;
/// 
///let config = Config::build(env::args()).unwrap_or_else(|err|{
///  eprintln!("Problema pasando argumentos: {err}");
///  process::exit(1);
///});
///if let Err(e) = minigrep::run(config) {
///  eprintln!("Error: {e}");
///  process::exit(1)
///} 
///``` 
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
  let contenido = fs::read_to_string(config.ruta)?;
  
  let resultado = if config.distinguir{
    busqueda(&config.consulta, &contenido)
  }else{
    buscar_distinguido(&config.consulta, &contenido)
  };
  
  for linea in resultado{
    println!("{linea}");
  }
  Ok(())
}

fn busqueda<'a>(consulta: &str, contenido: &'a str) -> Vec<&'a str>{
  contenido
    .lines()
    .filter(
      |linea| 
        linea.contains(consulta)
    )
    .collect()
}

fn buscar_distinguido<'a>(consulta: &str, contenido: &'a str) -> Vec<&'a str>{
  let consulta = consulta.to_lowercase();
  contenido
    .lines()
    .filter(
      |linea| 
        linea.to_lowercase().contains(&consulta)
    )
    .collect()
}

#[cfg(test)]
mod test{
  use super::*;

  #[test]
  fn un_resultado() {
    let consulta = "duct";
    let contenido = "\
Rust:
safe, fast, productive.
Pick three.";
    
    assert_eq!(vec!["safe, fast, productive."], busqueda(consulta, contenido));
  }

  #[test]
  fn distinguido() {
    let consulta = "duct";
    let contenido = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    
    assert_eq!(vec!["safe, fast, productive."], busqueda(consulta, contenido));
  }

  #[test]
  fn no_distinguido() {
    let consulta = "rUsT";
    let contenido = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    
    assert_eq!(vec!["Rust:", "Trust me."], buscar_distinguido(consulta, contenido));
  }
}