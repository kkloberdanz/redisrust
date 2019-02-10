use std::io;
use std::io::Write;
use std::collections::HashMap;
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


fn split_command(command: &String) -> Vec<String> {
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
    let mut db: HashMap<String, Record> = HashMap::new();
    let prompt = String::from("> ");

    while let Input::Command(user_input) = prompt_user(&prompt) {

        let input_vec: Vec<_> = split_command(&user_input)
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
                Action::Get(key) => match get(&db, key) {
                    Some(item) => match item {
                        Record::Str(s) => println!("{}", s),
                        _ => println!("Not implemented"),
                    },
                    None => continue,
                },
                Action::Set(key, value) => {
                    db = set(db, key, value);
                },
            },
            Result::Err(msg) =>
                println!("{}", msg),
        };
    }
}
