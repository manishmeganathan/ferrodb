pub(crate) mod database;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    println!("Input -> key='{}' and value='{}'", key, value);

    let data = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", data).unwrap();

    let _db = database::Database::new().expect("database creation crashed!");

    //println!("{}", db::map);
}
