#![allow(dead_code)]  // Permet le code non utilisé dans ce fichier

use dioxus::prelude::*;
use dioxus::logger::tracing::debug;
use dioxus_elements::g;
use serde_json::Value;
use serde::Serialize; // added import
use std::collections::HashMap;
use std::sync::mpsc;
use reqwest::Client;

use std::env;

pub fn get_api_key() -> Option<String> {
    env::var("TMDB_REQ_KEY").ok()
}

fn main() {
    match get_api_key() {
        Some(api_key) => println!("API Key: {}", api_key),
        None => println!("API Key not found. Please set the API_KEY environment variable."),
    }
}


pub struct GenreMap {
    pub name_to_id: HashMap<String, i32>,
    pub id_to_name: HashMap<i32, String>,
}

impl GenreMap{
    pub fn new(genres: HashMap<i32, String>) -> Self {
        let mut name_to_id: HashMap<String, i32>= HashMap::new();
        let id_to_name: HashMap<i32, String>= genres;
        for (id, genre) in &id_to_name {
            name_to_id.insert(genre.clone(), *id);
        }
        Self { name_to_id, id_to_name }
    }

    pub fn get_id(&self, genre: &str) -> Option<i32> {
        self.name_to_id.get(genre).copied()
    }

    pub fn get_genre(&self, id: i32) -> Option<&String> {
        self.id_to_name.get(&id)
    }
}

#[derive(Serialize)] // added Serialize derive
pub struct Movie {
    pub d: Value,
}

impl Movie{
    pub fn new(data:Value) -> Self {
        Self { d: data}
    }
}

pub fn test_channel(tx : mpsc::SyncSender<i32>){
    debug!("into the fonction");
    tx.send(32).unwrap();
}

pub async fn ask_tmdb(requrl: &str, tx: mpsc::Sender<String>) {  // Rendre la fonction asynchrone
    let requrl = requrl.to_string();

    let client = Client::new();  // Utiliser le client asynchrone

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", API_KEY.parse().unwrap());
    headers.insert("accept", "application/json".parse().unwrap());

    match client
        .get(&requrl)
        .headers(headers)
        .send()
        .await  // Attendre la réponse asynchrone
    {
        Ok(response) => {
            let status = response.status();
            match response.text().await {  // Attendre le texte de la réponse
                Ok(text) => {
                    debug!("Request successful: Status {:?}", status);
                    if tx.send(text).is_err() {
                        debug!("Failed to send response");
                    }
                },
                Err(e) => {
                    debug!("Failed to read response text: {:?}", e);
                    if tx.send(String::from("Error reading response")).is_err() {
                        debug!("Failed to send error message");
                    }
                }
            }
        },
        Err(e) => {
            debug!("Request error: {:?}", e);
            if tx.send(String::from("Request failed")).is_err() {
                debug!("Failed to send error message");
            }
        }
    };
}
/* 
fn create_random_data(size : i32, w :i32, h :i32) -> String{

    let mut b1 = [0u8; 20];
    let mut b2 = [0u8; 20];
    
    // Remplir les vecteurs avec des octets aléatoires
    getrandom::getrandom(&mut b1).expect("Échec de la génération des octets aléatoires pour b1");
    getrandom::getrandom(&mut b2).expect("Échec de la génération des octets aléatoires pour b2");
    
    // Zipper b1 et b2 en un vecteur de tuples (f64, f64) avec valeurs entre 0.0 et 1.0
    let data: Vec<(f64, f64)> = b1.iter()
        .zip(b2.iter())
        .map(|(&a, &b)| ((a as f64 / 255.0)*(w as f64), (b as f64 / 255.0)*(h as f64)))
        .collect();

    json!(data).to_string()
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GenreRes {
    id: i32,
    name: String,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct GenreList {
    genres: Vec<GenreRes>,
}
pub fn get_genre(){
    spawn(async move {
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", API_KEY.parse().unwrap());
        headers.insert("accept", "application/json".parse().unwrap());
        let resp = client
            .get("https://api.themoviedb.org/3/genre/movie/list?language")
            .headers(headers)
            .send()
            .await;

        match resp {
            Ok(genres) => {
                let genres = genres.json::<GenreList>().await.unwrap();
                debug!("Request successful");
                let genre_map = GenreMap::new(genres);
                match result {
                    Ok(_) => debug!("File saved"),
                    Err(e) => {
                        debug!("File not saved");
                        debug!("{:?}",e)
                    }
                }
            },
            Err(_) => debug!("Request failed"),
        }
    });
}
*/
