use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

#[derive(Debug)]
pub struct Db {
    host: &'static str,
    password: &'static str,
    port: i16,
    user: &'static str,
    pool: MySqlPool,
}

impl Db {
    pub async fn new() -> Result<Db, sqlx::Error> {
        let host = "db-mysql-fra1-63257-do-user-16108155-0.c.db.ondigitalocean.com";
        let password = "AVNS_Yv2uBhNARgrljcVnjR9";
        let port = 25060;
        let user = "doadmin";

        let database_url = format!(
            "mysql://{}:{}@{}:{}/db?ssl_mode=REQUIRED",
            user, password, host, port
        );

        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Db {
            host,
            password,
            port,
            user,
            pool,
        })
    }

    pub async fn create_account(
        &self,
        user_email: &str,
        user_password: &str,
        user_name: &str,
        user_last_name: &str,
    ) -> Result<bool, sqlx::Error> {
        let hashed_password = match hash(user_password, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => panic!("Failed to hash password"),
        };

        let result = sqlx::query!(
            "INSERT INTO user (email, password, first_name, last_name) VALUES (?, ?, ?, ?)",
            user_email,
            hashed_password,
            user_name,
            user_last_name
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn login(&self, user_email: &str, user_password: &str) -> Result<bool, sqlx::Error> {
        let user_record = sqlx::query!("SELECT password FROM user WHERE email = ?", user_email)
            .fetch_one(&self.pool)
            .await;

        match user_record {
            Ok(record) => {
                if let Some(stored_password) = record.password {
                    let password_matched = verify(user_password, &stored_password)
                        .map_err(|_| sqlx::Error::RowNotFound)?;

                    Ok(password_matched)
                } else {
                    Ok(false) // Le mot de passe est null
                }
            }
            Err(_) => Ok(false), // Aucun enregistrement trouvé pour l'e-mail donné
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connexion_bd() {
        let db = Db::new().await.unwrap();
        let connection_result = db.pool.acquire().await;
        assert!(
            connection_result.is_ok(),
            "Failed to establish database connection: {:?}",
            connection_result.err()
        );
    }

    #[tokio::test]
    async fn test_add_user() {
        let user_email = "testu@testu.com";
        let user_password = "password";
        let user_name = "testu";
        let user_last_name = "last_testu";
        let lets_db = Db::new().await.unwrap();
        lets_db
            .create_account(user_email, user_password, user_name, user_last_name)
            .await
            .unwrap();
        // Supprimer l'utilisateur après le test
        let delete_result = sqlx::query!("DELETE FROM user WHERE email = ?", user_email)
            .execute(&lets_db.pool)
            .await;
        assert!(
            delete_result.is_ok(),
            "Failed to delete user: {:?}",
            delete_result.err()
        );
    }

    #[tokio::test]
    async fn test_login() {
        // Test d'un utilisateur enregistré
        let user_email = "test@test.com";
        let user_password = "test_password";
        let lets_db = Db::new().await.unwrap();
        let login_result = lets_db.login(user_email, user_password).await;
        assert!(login_result.unwrap());

        // Test d'un utilisateur non enregistré
        let false_user_email = "test@fezf.com";
        let false_user_password = "dzqd";
        let false_lets_db = Db::new().await.unwrap();
        let false_login_result = false_lets_db
            .login(false_user_email, false_user_password)
            .await;
        assert!(!false_login_result.unwrap());
    }
}
