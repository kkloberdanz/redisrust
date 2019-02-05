use std::collections::HashMap;
//use std::collections::HashSet;
//use std::hash::Hash;


#[derive(Eq, PartialEq, Debug, Hash)]
enum Record {
    Int(i32),
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


fn main() {
    let v = Record::List(vec![Record::Int(3),
                              Record::Str("Hello world!".to_string()),
                              Record::List(vec![Record::Int(1),
                                                Record::Int(2)]),
                             ]);
    let r1 = Record::Int(32);
    let r2 = Record::Int(32);
    let r3 = Record::Int(5);
    let r4 = Record::Str("is this right?".to_string());
    println!("Hello, world! {:?}", v);

    println!("r1 == r2 {}", r1 == r2);
    println!("r1 == r3 {}", r1 == r3);
    println!("r1 == r4 {}", r1 == v);
}
