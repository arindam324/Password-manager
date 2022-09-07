use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::path::Path;


#[derive(Debug, Deserialize, Serialize)]
struct PasswordType {
    name: String,
    value: String,
}


fn main() -> Result<(), std::io::Error> {
    let json_file_path = Path::new("./secret.json");
    let file = File::open(json_file_path).expect("there is no file called secret.json");

    let passwords: Vec<PasswordType> = serde_json::from_reader(file).expect("error while parsing the file");

    for i in passwords.iter() {
        println!("{}", i.name);
        println!("{}", i.value);
    }

    Ok(())
}