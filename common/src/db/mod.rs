use std::env;
use mysql::*;
use std::error::Error;
use mysql::prelude::Queryable;

pub struct Database {
    pool: Pool
}

impl Database {
    pub fn connect() -> Result<Database, Box<dyn Error>> {
        let mut opts = OptsBuilder::new();
        let username = env::var("USERNAME").expect("La USERNAME variable n'a pas été définie");
        let password = env::var("PASSWORD").expect("La PASSWORD variable n'a pas été définie");
        let hostname = env::var("HOSTNAME").expect("La HOSTNAME variable n'a pas été définie");
        let port = env::var("PORT").expect("La PORT variable n'a pas été définie");
        let db_name = env::var("DB_NAME").expect("La DB_NAME variable n'a pas été définie");

        opts.user(username)
            .pass(password)
            .ip_or_hostname(hostname)
            .port(port)
            .db_name(db_name);

        let pool = Pool::new(opts)?;

        Ok(Database { pool })
    }

    pub fn query_example(&mut self) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        let query = "SELECT * FROM example_table";
        let result = conn.query_map(query, |(column1, column2): (i32, String)| {
            (column1, column2)
        })?;

        for (column1, column2) in result {
            println!("column1: {}, column2: {}", column1, column2);
        }

        Ok(())
    }
}
