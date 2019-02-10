use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::sync::RwLock;
//use std::collections::HashSet;
//use std::hash::Hash;


#[derive(Debug)]
enum Input {
    Command(String),
    Quit,
}


#[derive(Eq, PartialEq, Debug, Hash)]
enum Record {
    Str(String),
    List(Vec<Record>),
    //Set(HashSet<Record>),
    //Map(Box<Record>, Box<Record>),
}


impl Clone for Record {
    fn clone(&self) -> Record {
        match self {
            Record::Str(s) => Record::Str(s.clone()),
            Record::List(l) => Record::List(l.clone())
        }
    }
}


fn set(db: &RwLock<HashMap<String, Record>>,
       key: String,
       value: Record) {
    let mut writter = db.write().unwrap();
    writter.insert(key, value);
}


fn get(db: &RwLock<HashMap<String, Record>>, key: &String) -> Option<Record> {
    let reader = db.read().unwrap();
    let value = reader.get(key);
    match value {
        Some(x) => Some(x.clone()),
        None => None
    }
}


fn make_record(item: &String) -> Record {
    Record::Str(item.to_string())
}


#[derive(Debug)]
enum Action {
    Get(String),
    Set(String, Record),
}


fn prompt_user(prompt: &String) -> Input {
    print!("{}", prompt);
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("failed to read stdin");
    let as_string = user_input.trim().to_string();
    if as_string == "q" {
        Input::Quit
    } else {
        Input::Command(as_string)
    }
}


fn lex(command: &String) -> Vec<String> {
    let mut vec = Vec::new();
    let mut in_string = false;
    let mut in_escape_char = false;
    let mut word: String = String::new();
    for c in command.chars() {
        if c == '"' && !in_escape_char {
            in_string = !in_string;
        } else if c == '\\' && !in_escape_char {
            in_escape_char = true;
        } else if c == ' ' && !in_string {
            vec.push(word);
            word = String::new();
        } else {
            in_escape_char = false;
            word.push(c);
        }
    }
    if word.len() > 0 {
        vec.push(word);
    }
    vec
}


fn parse(input_vec: &Vec<String>) -> Result<Action, String> {
    let length = input_vec.len();

    if length == 0 {
        return Result::Err("".to_string())
    }

    let command = &input_vec[0];
    let params = &input_vec[1..];

    match command.as_ref() {
        "get" =>
            if length == 2 {
                Result::Ok(Action::Get(params[0].to_string()))
            } else {
                Result::Err("expecting: get <key>".to_string())
            },

        "set" =>
            if length == 3 {
                Result::Ok(Action::Set(params[0].to_string(),
                           make_record(&params[1].to_string())))
            } else {
                Result::Err("expecting: set <key> <value>".to_string())
            }

        _ => {
            Result::Err(format!("invalid command {}", command))
        }
    }
}


fn main() {
    let db = RwLock::new(HashMap::new());
    let prompt = String::from("> ");

    while let Input::Command(user_input) = prompt_user(&prompt) {

        let input_vec: Vec<_> = lex(&user_input)
            .iter()
            .map(|s| s.trim().to_lowercase())
            .collect::<Vec<_>>();

        if input_vec.len() == 0 {
            continue;
        }

        // parse input
        let parsed = parse(&input_vec);

        // execute db operations
        match parsed {
            Result::Ok(action) => match action {
                Action::Get(key) => match get(&db, &key) {
                    Some(item) => match item {
                        Record::Str(s) => println!("{}", s),
                        _ => println!("Not implemented"),
                    },
                    None => continue,
                },
                Action::Set(key, value) => {
                    set(&db, key, value);
                },
            },
            Result::Err(msg) =>
                println!("{}", msg),
        };
    }
}
