use std::io;
use std::collections::HashMap;
//use std::collections::HashSet;
//use std::hash::Hash;


#[derive(Eq, PartialEq, Debug, Hash)]
enum Record {
    Str(String),
    List(Vec<Record>),
    //Set(HashSet<Record>),
    //Map(Box<Record>, Box<Record>),
}


fn set(mut data: HashMap<String, Record>,
       key: String,
       value: Record) -> HashMap<String, Record> {
    data.insert(key, value);
    data
}


fn get(data: &HashMap<String, Record>, key: String) -> Option<&Record> {
    data.get(&key)
}


fn make_record(item: &String) -> Record {
    Record::Str(item.to_string())
}


#[derive(Debug)]
enum Action {
    Get(String),
    Set(String, Record),
}


fn main() {
    let mut db: HashMap<String, Record> = HashMap::new();

    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("failed to read stdin");
        let input_vec: Vec<_> = user_input
            .split_whitespace()
            .map(|s| s.trim().to_lowercase())
            .collect::<Vec<_>>();

        let command = &input_vec[0];
        let params = &input_vec[1..];

        // parse action
        let action = match command.as_ref() {
            "get" => Action::Get(params[0].to_string()),
            "set" => Action::Set(params[0].to_string(),
                                 make_record(&params[1].to_string())),
            _ => {
                println!("invalid command {}", command);
                continue;
            }
        };

        // execute action
        let result = match action {
            Action::Get(key) => get(&db, key),
            Action::Set(key, value) => {
                db = set(db, key, value);
                None
            },
        };

        // print result
        match result {
            None => println!("None"),
            Some(item) => match item {
                Record::Str(s) => println!("{}", s),
                _ => println!("Not implemented"),
            }
        }
    }
}
