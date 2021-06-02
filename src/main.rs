use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("key: {}, value: {}", key, value);
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();
    // on the stack
    let database = Database::new().expect("Creating database: failed.");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;        

        for line in contents.lines() {
            let mut chunks = line.splitn(1, '\t');       
            let key = chunks.next().expect("No key.");
            let value = chunks.next().expect("No value.");
            map.insert(key.to_owned(), value.to_owned());
        }
        Result::Ok(Database { map: map, })
    }
}