use mysql::*;
//use dotenv::dotenv;
use mysql::prelude::Queryable;

pub struct Database {
    pool: Pool
}

impl Database {

    fn new() -> Self {

        //database
        let url = "mysql://root@localhost:3307/rust";

        //create connection
        let pool = Pool::new(url).unwrap();


        Self {
            pool
        }


        //pool.get_conn().unwrap()
    }

    pub fn query(query: &str) -> Result<(), anyhow::Error> {
        let db = Database::new();
        let conn = db.pool.get_conn();
        let query = query;
        let result = conn?.query_map(query, |(column1, column2): (i32, String)| {
            (column1, column2)
        })?;

        for (column1, column2) in result {
            println!("column1: {}, column2: {}", column1, column2);
        }

        Ok(())
    }
}
