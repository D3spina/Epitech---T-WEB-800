use common::db::Database;
fn main(){
    println!("{:?}",Database::query("SELECT * FROM user"));
}