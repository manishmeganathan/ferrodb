use std::collections::HashMap;

pub struct Database {
    pub map: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Result<Database, std::io::Error> {
        // Create a new mutable hashmap
        let mut map = HashMap::new();

        // Read the kv.db file
        let dbcontents = std::fs::read_to_string("kv.db")?;

        // Iterate over the contents of the db lines
        for line in dbcontents.lines() {
            // Parse each line into a key-value pair
            let (key, value) = line.split_once("\t").expect("db is corrupt");
            // Insert the key-value pair into the hash map
            map.insert(key.to_string(), value.to_string());
        }

        // Return the constructed database
        Ok(Database{map})
    }
}