extern crate csv;

use std::{fs::File, collections::HashMap};
use std::io::{self, BufReader};
use csv::{ReaderBuilder, Trim};
use crate::{AccountDetails, Transactions};
use crate::{Input, operations::process_operation};

/*
    Read CSV line by line and deserialize
 */
pub fn process_csv(path: &str) -> Result<HashMap<u16,AccountDetails>, io::Error> {
    let mut accounts:HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions:HashMap<u32,Transactions> = HashMap::new();
    let file = File::open(path).unwrap();
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(true)
        .from_reader(BufReader::new(file));
    let mut line = 1;
    for result in reader.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, line, &mut transactions){
            Ok(()) => {line = line + 1},
            Err(error) => panic!("{}", error),
        }
    }
    Ok(accounts)
}