use std::collections::HashMap;

pub struct Database {
    pub map: HashMap<String, String>,
}

impl Database {
    // A constructor function for Database
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

    pub fn insert(&mut self, key: String, value: String) {
        // Insert the key value pair into the map
        self.map.insert(key, value);
    }

    pub fn flush(self) -> std::io::Result<()> {
        // Declare a new string for the full contents of the DB
        let mut dbcontents = String::new();
        // Declare a new string for the line by line iteration
        let mut kvpair;

        // Iterate over the contents of the db map
        for (key, value) in self.map {
            // Format the key value pair into a string
            kvpair = format!("{}\t{}\n", key, value);
            // Push the string into the db contents
            dbcontents.push_str(&kvpair);
        }

        // Write the full db contents to file
        return std::fs::write("kv.db", dbcontents);
    }
}