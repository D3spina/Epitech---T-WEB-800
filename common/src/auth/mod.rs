use std::collections::HashMap;
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use crate::db::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email:String,
    pub password:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Register {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub secure_password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    email: String,
    password: String,
    exp: usize,
}

impl Claims {
    fn generate_jwt(email: String, password: String) -> HashMap<String, String> {
        let mut res: HashMap<String, String> = HashMap::new();
        // Définition des revendications du JWT
        let claims = Claims {
            email : email.clone(),
            password : password.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize, // Expiration dans 24 heures
        };

        // Clé secrète pour signer le JWT
        let secret = "your_secret_key";

        // Encodage du JWT
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()));

        let valid = Database::query(&format!("Select email in user where email='{email}' and password='{password}'",email = email,password = password));

        if(valid.is_err()){
            res.insert("error".to_string(),"account not found".to_string());
            return res;
        }

        res.insert("token".to_string(),token.unwrap());

        res
    }

    pub fn generate_jwt_login(email: String, password: String) -> HashMap<String, String> {
        // Définition des revendications du JWT
        return Self::generate_jwt(email,password);
    }

    pub fn generate_jwt_register(first_name:String, last_name:String, email: String, password: String, secure_password:String) -> HashMap<String, String> {
        // Définition des revendications du JWT
        let mut res: HashMap<String, String> = HashMap::new();

        let valid;
        if password == secure_password {
               valid = Database::query(&format!("INSERT INTO `user`(`first_name`, `last_name`, `email`, `password`) VALUES ('{first_name}','{last_name}','{email}','{password}')",first_name = first_name,last_name=last_name,email=email,password=password));
        }else {
            res.insert("error".to_string(),"password is not same".to_string());
        }

        if valid.is_ok() {
            return Self::generate_jwt(email,password);
        }

        {"Error":"Password is not same".to_string()}


    }

    // Fonction pour vérifier un JWT
    pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        // Clé secrète pour déchiffrer le JWT
        let secret = "your_secret_key";

        // Validation et décodage du JWT
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}