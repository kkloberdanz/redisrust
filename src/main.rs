/*
 * Author: Kyle Kloberdanz
 * Date: 4 Feb 2019
 * File: distrust.rs
 *
 * Discription:
 *  This software acts as an in-memory key value store and database
 *
 * License: GNU GPLv3
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */


use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::sync::RwLock;
use std::net::TcpListener;
use std::thread;
use std::time;
use std::io::Read;
use std::slice::SliceIndex;
use std::sync::Arc;


//use std::collections::HashSet;
//use std::hash::Hash;


extern crate threadpool;


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

    let command = &input_vec[0].to_lowercase();
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


fn evaluate(user_input: &String,
            db: &RwLock<HashMap<String, Record>>) -> String {
    let input_vec: Vec<_> = lex(&user_input)
        .iter()
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    if input_vec.len() == 0 {
        return String::from("");
    }

    let parsed = parse(&input_vec);

    // execute db operations
    match parsed {
        Result::Ok(action) => match action {
            Action::Get(key) => match get(&db, &key) {
                Some(item) => match item {
                    Record::Str(s) => format!("{}", s),
                    _ => format!("Not implemented"),
                },
                None => String::from(""),
            },
            Action::Set(key, value) => {
                set(&db, key, value);
                String::from("Ok")
            },
        },
        Result::Err(msg) =>
            format!("{}", msg),
    }
}


fn drop_header(http_msg: &String) -> String {
    let vec: Vec<&str> = http_msg.split('\n').collect();
    let last_item = vec.last();
    match last_item {
        Some(s) => s.trim().to_string(),
        None => "".to_string()
    }
}


fn handle_connection(mut stream: std::net::TcpStream,
                     db: &RwLock<HashMap<String, Record>>) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer);
    let buf_as_vec: Vec<u8> = buffer.to_vec();
    let index_of_null = buf_as_vec.iter().position(|&i| i == 0u8).unwrap();
    let buf_as_vec = buf_as_vec[..index_of_null].to_vec();
    let stream_data = String::from_utf8(buf_as_vec);
    match stream_data {
        Ok(s) => {
            let user_input = drop_header(&s);
            println!("User input: {}", user_input);
            let result = evaluate(&user_input, &db);
            println!("{}", result);
            let response = format!("HTTP/1.1 200 OK \r\n\r\n{}\r\n", result);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        Err(msg) => println!("error: {}", msg),
    }
}


fn main() {
    let db = Arc::new(RwLock::new(HashMap::new()));
    //let prompt = String::from("> ");

    //while let Input::Command(user_input) = prompt_user(&prompt) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        println!("Got connection");
        let db = Arc::clone(&db);
        let stream = stream.unwrap();

        thread::spawn(move || {
            handle_connection(stream, &db);
        });
    }

}
