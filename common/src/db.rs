use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Serialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::mysql::MySqlQueryResult;
use sqlx::{Error as SqlxError, Error, MySqlPool, Row, Transaction};
use sqlx::{self, MySql, Pool, query, query_scalar};
use sqlx_macros::FromRow;

#[derive(Debug)]
pub struct Db {
    host: &'static str,
    password: &'static str,
    port: i16,
    user: &'static str,
    pool: MySqlPool,
}

#[derive(Debug)]
struct Activity {
    name: String,
    address: String,
    city: String,
}
#[derive(Debug, FromRow, Serialize)]
pub struct ActivityDetails {
    name: String,
    address: String,
    city: String,
    depart: String,
    arrive: String,
}

impl Db {
    pub async fn new() -> Result<Db, Error> {
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
    pub async fn fetch_activity_details(self, user_email: &str, activity_description: &str) -> Result<Vec<ActivityDetails>, sqlx::Error> {
        let user_id: i32 = sqlx::query("SELECT id FROM user WHERE email = ?")
            .bind(user_email)
            .fetch_one(&self.pool)
            .await?
            .try_get(0)?;

        let result = sqlx::query_as::<_, ActivityDetails>(
            "SELECT activity.name, activity.address, activity.city, travel.depart, travel.arrive
         FROM activity
         JOIN travel ON activity.id = travel.activity_id
         WHERE travel.id_user = ? AND travel.description = ? ")
            .bind(user_id)
            .bind(activity_description.to_string())
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn add_activity_and_travel(
        &self,
        email: &str,
        activity_name: String,
        address: String,
        city: String,
        description: &str,
        transport: &str,
        depart: &str,
        arrive: &str,
    ) -> Result<(), sqlx::Error> {

        let mut tx: Transaction<'_, MySql> = self.pool.begin().await?;
        let address_clone= address.clone();
        // Check for existing activity
        let activity_id: Option<i32> =
            sqlx::query_scalar("SELECT id FROM activity WHERE address = ?")
                .bind(address)
                .fetch_optional(&mut tx)
                .await?;

        let activity_id = match activity_id {
            Some(id) => id,
            None => {
                // Insert new activity
                let result: MySqlQueryResult = sqlx::query(
                    "INSERT INTO activity (name, address, city) VALUES (?, ?, ?)",
                )
                    .bind(activity_name)
                    .bind(address_clone)
                    .bind(city)
                    .execute(&mut tx)
                    .await?;
                result.last_insert_id() as i32
            }
        };

        // Find user ID by email
        let user_id: Option<i32> = sqlx::query_scalar("SELECT id FROM user WHERE email = ?")
            .bind(email)
            .fetch_optional(&mut tx)
            .await?;
        if let Some(user_id) = user_id {
            // Insert into travel
            sqlx::query(
                "INSERT INTO travel (id_user, description, activity_id, transport, depart, arrive) VALUES (?, ?, ?, ?, ?, ?)"
            )
                .bind(user_id)
                .bind(description)
                .bind(activity_id)
                .bind(transport)
                .bind(depart)
                .bind(arrive)
                .execute(&mut tx)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn create_account(
        &self,
        user_email: &str,
        user_password: &str,
        user_name: &str,
        user_last_name: &str,
    ) -> Result<bool, Error> {
        let hashed_password = match hash(user_password, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => panic!("Failed to hash password"),
        };

        let result = sqlx::query(
            "INSERT INTO user (email, password, first_name, last_name) VALUES (?, ?, ?, ?)",)
            .bind(user_email)
            .bind(hashed_password)
            .bind(user_name)
            .bind(user_last_name)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn login(&self, user_email: &str, user_password: &str) -> Result<bool, Error> {
        let result = sqlx::query("SELECT password FROM user WHERE email = ?")
            .bind(user_email)
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(row) => {
                let stored_password: Option<String> = row.try_get("password")?;
                if let Some(password) = stored_password {
                    // Verify the password against the hash stored in the database
                    let password_matched = verify(user_password, &password)
                        .map_err(|_| Error::RowNotFound)?; // Adjust error handling as necessary

                    Ok(password_matched)
                } else {
                    Ok(false) // Password is null
                }
            },
            Err(e) => Err(e), // Pass along any SQL error
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[tokio::test]
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
        let delete_result = sqlx::query("DELETE FROM user WHERE email = ?", user_email)
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
    }*/
}
