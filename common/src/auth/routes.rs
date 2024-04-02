//routes.rs
use chrono::{DateTime, Utc};
use std::sync::{Arc, RwLock};
use tokio::time::{interval, Duration};

lazy_static! {
    pub static ref GLOBAL_TOKEN_MANAGER: TokenManager = TokenManager::new(5, 1);
}

struct AuthToken {
    token: String,
    exp_time: DateTime<Utc>,
    expire_seconds: i64,
}

impl AuthToken {
    pub fn new(expire_seconds: i64) -> Self {
        AuthToken {
            token: String::new(),
            exp_time: Utc::now() + chrono::Duration::seconds(expire_seconds),
            expire_seconds,
        }
    }

    pub async fn update_token(&mut self) {
        println!("Generating new token!");
        self.token = AuthToken::generate_secure_token().await;
        self.exp_time = Utc::now() + chrono::Duration::seconds(self.expire_seconds);
    }

    pub fn refresh_token(&mut self, token: &str) {
        println!("Generating new token!");
        self.token = token.to_owned();
        self.exp_time = Utc::now() + chrono::Duration::seconds(self.expire_seconds);
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.exp_time
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub async fn generate_secure_token() -> String {
        let mut interval = interval(Duration::from_secs(1));
        interval.tick().await;
        Utc::now().to_string()
    }
}

pub struct TokenManager {
    auth_token: Arc<RwLock<AuthToken>>,
    tick_interval_secs: u64,
}

impl TokenManager {
    pub fn new(expire_seconds: i64, tick_interval_secs: u64) -> Self {
        Self {
            auth_token: Arc::new(RwLock::new(AuthToken::new(expire_seconds))),
            tick_interval_secs,
        }
    }

    pub fn fetch_token(&self) -> Option<String> {
        let token = self.auth_token.clone();
        let token_string = match token.read() {
            Ok(t) => Some(t.get_token()),
            Err(e) => {
                eprintln!("Failed to acquire lock: {}", e);
                None
            }
        };
        token_string
    }

    pub async fn refresh_token(&self) {
        let token_ref = self.auth_token.clone();
        let new_token = AuthToken::generate_secure_token().await;
        match token_ref.write() {
            Ok(mut t) => t.refresh_token(&new_token),
            Err(e) => {
                eprintln!("Failed to acquire lock: {}", e);
            }
        };
    }

    pub async fn generate_tokens(&self) -> tokio::task::JoinHandle<()> {
        let token_ref = self.auth_token.clone();
        let interval_secs = self.tick_interval_secs;
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(interval_secs));
            loop {
                interval.tick().await;

                // Check if the token needs to be refreshed
                let needs_refresh = {
                    let token = match token_ref.read() {
                        Ok(t) => t,
                        Err(e) => {
                            eprintln!("Failed to acquire lock: {}", e);
                            continue;
                        }
                    };
                    token.is_expired()
                };

                // Refresh the token if necessary
                if needs_refresh {
                    let new_token = AuthToken::generate_secure_token().await;
                    let mut token = match token_ref.write() {
                        Ok(t) => t,
                        Err(e) => {
                            eprintln!("Failed to acquire lock: {}", e);
                            continue;
                        }
                    };
                    token.refresh_token(&new_token);
                }
            }
        })
    }
}