use std::io::{Read, stdin};
use std::process::exit;
use serde_json;
use hugr::{hugr::ValidationError, Hugr, extension::PRELUDE_REGISTRY};

fn parse_and_validate () -> Result<(), ValidationError> {
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut buffer).unwrap();
    let hugr: Hugr = serde_json::from_str(&buffer).unwrap();
    hugr.validate(&PRELUDE_REGISTRY)?;
    println!("hugr parsed & validated successfully!");
    Ok(())
}

fn main() {
    match parse_and_validate() {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            exit(1);
        },
    }
}
