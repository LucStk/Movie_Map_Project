#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]  // Ajout des autres attributs d'autorisation

use crate::utils::tmdb_api::*;
use std::sync::mpsc;
use serde_json::Value;
use dioxus::{html::pre, logger::tracing::{debug, Level}};
use tokio::runtime::Builder;  // Ajouter tokio runtime pour exécuter les tests
// Fonction de test standalone
#[test]
fn debug_tmdb_request() {
    // Initialize logger, ignore if already initialized
    let _ = dioxus::logger::init(Level::DEBUG);
    
    let (tx, rx) = mpsc::channel();

    // Créer un runtime tokio pour exécuter les fonctions asynchrones
    let rt = Builder::new_current_thread().enable_time().enable_io().build()
    .unwrap();;
    
    // Test avec une URL valide
    let test_url = "https://api.themoviedb.org/3/movie/popular?language=en-US&page=1";
    debug!("Testing invalid URL: {}", test_url);
    
    rt.block_on(async {
        ask_tmdb(test_url, tx.clone()).await;
    });
    
    // Attendre et afficher la réponse
    match rx.recv_timeout(std::time::Duration::from_secs(100)) {
        Ok(val) => debug!("Received valid response: {}", val),
        Err(e) => debug!("Error with valid URL: {:?}", e),
    }
    // Test avec une URL invalide
     let test_url = "https://api.themoviedb.org/3/movie/popuasenonase";
    debug!("Testing valid URL: {}", test_url);
    
    rt.block_on(async {
        ask_tmdb(test_url, tx.clone()).await;
    });
    
    // Attendre et afficher la réponse
    match rx.recv_timeout(std::time::Duration::from_secs(100)) {
        Ok(val) => debug!("Received valid response: {}", val),
        Err(e) => debug!("Error with valid URL: {:?}", e),
    }

}

#[test]
fn test_parse_movies(){
    let _ = dioxus::logger::init(Level::DEBUG);
    let data = std::fs::read_to_string("src/debug/res_request.txt")
        .expect("Should have been able to read the file");
    let v = match serde_json::from_str::<Value>(&data) {
        Err(e) => {
            debug!("Impossible to create the instance {:?}", e);
            serde_json::Value::Null
        },
        Ok(val) => {
            //debug!("value created {:?}", val);
            // list des clés du retour à chaud
            if let Some(obj) = val.as_object() {
                debug!("\n\nValue keys {:?}", obj.keys().collect::<Vec<_>>());
            }
            // Récupérer la liste des films comme une liste de struct Movie
            let movies_iter = &val["results"];
            let mut movies_list: Vec<Movie> = Vec::new();
            for mv in movies_iter.as_array().unwrap_or(&vec![]) {
                movies_list.push(Movie { d: mv.clone() });
            }

            // Sauvegarder la liste des films dans le format géré par javascript
            let stringif = serde_json::to_string(&movies_list).unwrap();


            // Sauvegarder la liste des films dans un fichier
            std::fs::write("src/debug/res_movies.txt", stringif).expect("Unable to write file");


            debug!("\n\nOne movie : {:?}", movies_list[0].d);
            // Afficher les clés du premier film
            let mut s = String::from("\n\nMovie keys :");
            let mut output = String::from("\n\nMovie keys :");
            for (key,value) in movies_list[0].d.as_object().unwrap(){
                output.push_str(&format!("\n {:?} : {:?}",key, value ));
            }
            debug!("{}", output);
            debug!(stringify!(s));
            val
        }
    };
}

#[test]
fn test_debug(){
    // Initialize logger, ignore if already initialized
    let _ = dioxus::logger::init(Level::DEBUG);
    
    let a = 10;
    println!("coucou");
    debug!("Testing valid URL: {}", a);
}

pub fn debug_test(){
    let a = 10;
    debug!("Testing valid URL: {}", a);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_debug(){
        debug_test();
    }

    #[test]
    fn run_all_tests() {
        test_debug();
        debug_tmdb_request();
        test_parse_movies();
    
    }
}
