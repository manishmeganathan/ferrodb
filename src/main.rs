pub(crate) mod database;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    println!("Input -> key='{}' and value='{}'", key, value);

    let mut db = database::Database::new().expect("database creation crashed!");

    db.insert(key.clone(), value.clone());

    //db.flush().unwrap();
}
