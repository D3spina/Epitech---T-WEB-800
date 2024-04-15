use serde_json::Value;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;
extern crate common;
use std::collections::HashMap;
use common::auth::{Claims, Login, Register};

// URL pour récupérer les bar dans un périmétre donné et pour une localisation donnée
#[post("/service/auth/login",format = "json",data = "<login>")]
async fn login(login: Json<Login>) -> Json<HashMap<String, String>> {
    let mut res: HashMap<String, String> = HashMap::new();
    let email = login.email.clone().parse().unwrap();
    let password = login.password.clone().parse().unwrap();

    let token = Claims::generate_jwt_login(email, password).unwrap();
    //let token = Claims::new();

    res.insert("token".parse().unwrap(), token);

    Json(res)
}

// URL pour récupérer les bar dans un périmétre donné et pour une localisation donnée
#[post("/service/auth/register",format = "json",data = "<register>")]
async fn register(register: Json<Register>) -> Json<HashMap<String, String>> {
    let mut res: HashMap<String, String> = HashMap::new();

    let first_name = register.first_name.clone().parse().unwrap();
    let last_name = register.last_name.clone().parse().unwrap();
    let email = register.email.clone().parse().unwrap();
    let password = register.password.clone().parse().unwrap();
    let secure_password = register.secure_password.clone().parse().unwrap();


    println!("test_other");

    let token = Claims::generate_jwt_register(first_name,last_name,email, password,secure_password);

    println!("test_other");

    if token.is_ok(){
        res.insert("token".to_string(),token.unwrap());
    } else {
        res.insert("error".to_string(),token.unwrap());
    }

    println!("{:?}",res);

    Json(res)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login,register])
}