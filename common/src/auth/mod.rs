struct AuthToken(String);

#[rocket::async_trait]
impl<'r> FromDataSimple for AuthToken {
    type Error = String;

    async fn from_data(_: &'r Request<'_>, data: Data) -> data::Outcome<Self, Self::Error> {
        let token = match data.open(1024 * 16).into_string().await {
            Ok(token) => token,
            Err(_) => return Outcome::Failure((Status::BadRequest, "Invalid token format".to_string())),
        };
        Outcome::Success(AuthToken(token))
    }
}

fn generate_jwt() -> Result<String, jsonwebtoken::errors::Error> {
    // Vos données à inclure dans le JWT
    let claims = jsonwebtoken::Claims::default();

    // Clé secrète pour signer le JWT
    let secret = "your_secret_key";

    // Encodage du JWT
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;

    Ok(token)
}

fn verify_token(token: &str) -> Result<(), String> {
    let token_data = decode::<String>(
        &token,
        &DecodingKey::from_secret("your_secret_key".as_ref()),
        &Validation::default(),
    );

    match token_data {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid token".to_string()),
    }
}
