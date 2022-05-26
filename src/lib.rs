use std::{env, collections::HashMap};
use serde::Deserialize;
mod csv_process;
mod operations;
mod error_handling;
use crate::csv_process::process_csv;


pub fn lib_main(){
    let args: Vec<String> = env::args().collect();
    let csv_file = args[1].to_string();
    match process_csv(&csv_file){
        Ok(accounts) => println!("{}", format_output(accounts)),
        Err(_e) => panic!("Error reading csv"),
    }
}

/*
    Print Output
*/
pub fn format_output(accounts:HashMap<u16,AccountDetails>)->String{
    let mut output = "client, available, held, total, locked".to_string();
    for (key, value) in accounts.into_iter() {
        output = format!("{}\n{},{},{},{},{}",output,key,value.total,value.held,value.total,value.locked.to_string());
    }
    output
}

/*
 Allow invalid_option for fields
 Invalid Fields can be treated by the application
 */
#[derive(Deserialize)]
pub struct Input{
    #[serde(rename = "type",deserialize_with = "csv::invalid_option")]
    op_type: Option<Operations>,
    #[serde(deserialize_with = "csv::invalid_option")]
    client: Option<u16>,
    #[serde(deserialize_with = "csv::invalid_option")]
    tx: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    amount: Option<f32>
}

pub struct Transactions{
    details: Input,
    dispute: bool
}

#[derive(Debug, Deserialize,PartialEq,Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Operations{
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback
}

#[derive(Debug, Deserialize)]
pub struct Amount(f64);

trait AddAmount{
    fn get_amount(amount: f32) -> f32;
}

impl AddAmount for Amount {

    fn get_amount(amount: f32) -> f32 {
        (amount * 10000.0).round() / 10000.0
    }
}

pub struct AccountDetails{
    available: f32,
    held: f32,
    total: f32,
    locked: bool
}


/*
    Default input csv test
 */
#[test]
fn default_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0";

    let mut expected: HashMap<u16,AccountDetails> = HashMap::new();
    expected.insert(1, AccountDetails { available: 1.5, held: 0.0, total: 1.5, locked: false });
    expected.insert(2, AccountDetails { available: 2.0, held: 0.0, total: 2.0, locked: false });
    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, 1, &mut transactions){
            Ok(()) => {}
            Err(error) => panic!("{}", error),
        }
    }
    for (key, value) in expected.iter() {
        let expected_value = accounts.get(key);
        match expected_value{
            Some(acc) => {
                assert_eq!(acc.total,value.total);
                assert_eq!(acc.held,value.held);
                assert_eq!(acc.locked,value.locked);
                assert_eq!(acc.available,value.available);
            }
            None => assert!(false),
        }
    }
}

/*
    Test withdrawal without previous client ID
 */
#[test]
fn withdrawal_non_existent_clientid_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0";

    let mut expected: HashMap<u16,AccountDetails> = HashMap::new();
    expected.insert(1, AccountDetails { available: 1.5, held: 0.0, total: 1.5, locked: false });
    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, 1, &mut transactions){
            Ok(()) => {}
            Err(error) => panic!("{}", error),
        }
    }
    for (key, value) in expected.iter() {
        let expected_value = accounts.get(key);
        match expected_value{
            Some(acc) => {
                assert_eq!(acc.total,value.total);
                assert_eq!(acc.held,value.held);
                assert_eq!(acc.locked,value.locked);
                assert_eq!(acc.available,value.available);
            }
            None => assert!(false),
        }
    }
}

/*
    Test dispute without previous client ID
 */
#[test]
fn dispute_non_existent_clientid_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.223298
    deposit, 1, 3, 2.3455987
    withdrawal, 1, 4, 1.5123987
    dispute, 2, 5, ";

    let mut expected: HashMap<u16,AccountDetails> = HashMap::new();
    expected.insert(1, AccountDetails { available: 2.0565, held: 0.0, total: 2.0565, locked: false });
    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, 1, &mut transactions){
            Ok(()) => {}
            Err(error) => panic!("{}", error),
        }
    }
    for (key, value) in expected.iter() {
        let expected_value = accounts.get(key);
        match expected_value{
            Some(acc) => {
                assert_eq!(acc.total,value.total);
                assert_eq!(acc.held,value.held);
                assert_eq!(acc.locked,value.locked);
                assert_eq!(acc.available,value.available);
            }
            None => assert!(false),
        }
    }
}

/*
    Test four decimal precision
 */
#[test]
fn four_decimal_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.223298
    deposit, 2, 2, 2.122398
    deposit, 1, 3, 2.3455987
    withdrawal, 1, 4, 1.5123987
    withdrawal, 2, 5, 1.2332889";

    let mut expected: HashMap<u16,AccountDetails> = HashMap::new();
    expected.insert(1, AccountDetails { available: 2.0565, held: 0.0, total: 2.0565, locked: false });
    expected.insert(2, AccountDetails { available: 0.8891, held: 0.0, total: 0.8891, locked: false });
    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, 1, &mut transactions){
            Ok(()) => {}
            Err(error) => panic!("{}", error),
        }
    }
    for (key, value) in expected.iter() {
        let expected_value = accounts.get(key);
        match expected_value{
            Some(acc) => {
                assert_eq!(acc.total,value.total);
                assert_eq!(acc.held,value.held);
                assert_eq!(acc.locked,value.locked);
                assert_eq!(acc.available,value.available);
            }
            None => assert!(false),
        }
    }
}

/*
    Test panic message for invalid operation
 */
#[test]
#[should_panic(expected = "Invalid Operation at line: 3")]
fn invalid_type_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    invalid, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0";

    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    let mut line = 1; //skip headers
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, line, &mut transactions){
            Ok(()) => {line = line +1}
            Err(error) => panic!("{}", error),
        }
    }
}

/*
    Invalid value in client column
 */
#[test]
#[should_panic(expected = "Invalid Client at line: 1")]
fn invalid_client_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, abc, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0";

    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    let mut line = 1; //skip headers
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, line, &mut transactions){
            Ok(()) => {line = line +1}
            Err(error) => panic!("{}", error),
        }
    }
}

/*
    Transaction with invalid format
 */
#[test]
#[should_panic(expected = "Invalid Tx at line: 1")]
fn invalid_tx_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, abc, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0";

    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    let mut line = 1; //skip headers
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, line, &mut transactions){
            Ok(()) => {line = line +1}
            Err(error) => panic!("{}", error),
        }
    }
}

/*
    Invalid value in amount column
 */
#[test]
#[should_panic(expected = "Invalid Amount at line: 1")]
fn invalid_amount_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, abc
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0";

    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    let mut line = 1; //skip headers
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, line, &mut transactions){
            Ok(()) => {line = line +1}
            Err(error) => panic!("{}", error),
        }
    }
}

/*
    Conflicting transactions: same ID in multiple rows
 */
#[test]
#[should_panic(expected = "Conflicting Transaction at line: 2")]
fn conflicting_transaction() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 1, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0";

    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    let mut line = 1; //skip headers
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, line, &mut transactions){
            Ok(()) => {line = line +1}
            Err(error) => panic!("{}", error),
        }
    }
}

/*
    Client ID and transaction do not match on dispute
 */
#[test]
#[should_panic(expected = "Divergent Transaction and Client ID at line: 6")]
fn divergent_transaction_id() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0
    dispute, 1, 2, ";

    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    let mut line = 1; //skip headers
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, line, &mut transactions){
            Ok(()) => {line = line +1}
            Err(error) => panic!("{}", error),
        }
    }
    
}


/*
    Test successful dispute
 */
#[test]
fn dispute_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0
    dispute, 1, 1, ";

    let mut expected: HashMap<u16,AccountDetails> = HashMap::new();
    expected.insert(1, AccountDetails { available: 0.5, held: 1.0, total: 1.5, locked: false });
    expected.insert(2, AccountDetails { available: 2.0, held: 0.0, total: 2.0, locked: false });
    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, 1, &mut transactions){
            Ok(()) => {}
            Err(error) => panic!("{}", error),
        }
    }
    for (key, value) in expected.iter() {
        let expected_value = accounts.get(key);
        match expected_value{
            Some(acc) => {
                assert_eq!(acc.total,value.total);
                assert_eq!(acc.held,value.held);
                assert_eq!(acc.locked,value.locked);
                assert_eq!(acc.available,value.available);
            }
            None => assert!(false),
        }
    }
}

/*
    Test successful resolve
 */
#[test]
fn resolve_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0
    dispute, 1, 1, 
    resolve, 1, 1, ";

    let mut expected: HashMap<u16,AccountDetails> = HashMap::new();
    expected.insert(1, AccountDetails { available: 1.5, held: 0.0, total: 1.5, locked: false });
    expected.insert(2, AccountDetails { available: 2.0, held: 0.0, total: 2.0, locked: false });
    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, 1, &mut transactions){
            Ok(()) => {}
            Err(error) => panic!("{}", error),
        }
    }
    for (key, value) in expected.iter() {
        let expected_value = accounts.get(key);
        match expected_value{
            Some(acc) => {
                assert_eq!(acc.total,value.total);
                assert_eq!(acc.held,value.held);
                assert_eq!(acc.locked,value.locked);
                assert_eq!(acc.available,value.available);
            }
            None => assert!(false),
        }
    }
}

/*
    Test successful chargeback
 */
#[test]
fn chargeback_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0
    dispute, 1, 1, 
    chargeback, 1, 1, ";

    let mut expected: HashMap<u16,AccountDetails> = HashMap::new();
    expected.insert(1, AccountDetails { available: 0.5, held: 0.0, total: 0.5, locked: true });
    expected.insert(2, AccountDetails { available: 2.0, held: 0.0, total: 2.0, locked: false });
    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, 1, &mut transactions){
            Ok(()) => {}
            Err(error) => panic!("{}", error),
        }
    }
    for (key, value) in expected.iter() {
        let expected_value = accounts.get(key);
        match expected_value{
            Some(acc) => {
                assert_eq!(acc.total,value.total);
                assert_eq!(acc.held,value.held);
                assert_eq!(acc.locked,value.locked);
                assert_eq!(acc.available,value.available);
            }
            None => assert!(false),
        }
    }
}

/*
    Test frozen account
 */
#[test]
fn frozen_account_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0
    dispute, 1, 1, 
    chargeback, 1, 1, 
    deposit, 1, 6, 1.0
    withdrawal, 1, 7, 1.5
    dispute, 1, 3, 
    resolve, 1, 3, 
    dispute, 1, 4, 
    chargeback, 1, 4, ";

    let mut expected: HashMap<u16,AccountDetails> = HashMap::new();
    expected.insert(1, AccountDetails { available: 0.5, held: 0.0, total: 0.5, locked: true });
    expected.insert(2, AccountDetails { available: 2.0, held: 0.0, total: 2.0, locked: false });
    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, 1, &mut transactions){
            Ok(()) => {}
            Err(error) => panic!("{}", error),
        }
    }
    for (key, value) in expected.iter() {
        let expected_value = accounts.get(key);
        match expected_value{
            Some(acc) => {
                assert_eq!(acc.total,value.total);
                assert_eq!(acc.held,value.held);
                assert_eq!(acc.locked,value.locked);
                assert_eq!(acc.available,value.available);
            }
            None => assert!(false),
        }
    }
}

/*
    Test issuing a resolve without previous dispute
 */
#[test]
fn resolve_without_dispute_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0
    resolve, 1, 1, 
    deposit, 1, 6, 1.0";

    let mut expected: HashMap<u16,AccountDetails> = HashMap::new();
    expected.insert(1, AccountDetails { available: 2.5, held: 0.0, total: 2.5, locked: false });
    expected.insert(2, AccountDetails { available: 2.0, held: 0.0, total: 2.0, locked: false });
    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, 1, &mut transactions){
            Ok(()) => {}
            Err(error) => panic!("{}", error),
        }
    }
    for (key, value) in expected.iter() {
        let expected_value = accounts.get(key);
        match expected_value{
            Some(acc) => {
                assert_eq!(acc.total,value.total);
                assert_eq!(acc.held,value.held);
                assert_eq!(acc.locked,value.locked);
                assert_eq!(acc.available,value.available);
            }
            None => assert!(false),
        }
    }
}

/*
    Test issuing a chargeback without previous dispute
 */
#[test]
fn chargeback_without_dispute_test() {
    use ::csv::{ReaderBuilder, Trim};
    use crate::operations::process_operation;
    let str_ = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0
    chargeback, 1, 1, 
    deposit, 1, 6, 1.0";

    let mut expected: HashMap<u16,AccountDetails> = HashMap::new();
    expected.insert(1, AccountDetails { available: 2.5, held: 0.0, total: 2.5, locked: false });
    expected.insert(2, AccountDetails { available: 2.0, held: 0.0, total: 2.0, locked: false });
    let mut accounts: HashMap<u16,AccountDetails> = HashMap::new();
    let mut transactions: HashMap<u32,Transactions> = HashMap::new();
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(str_.as_bytes());
    for result in rdr.deserialize::<Input>() {
        let tr: Input = result.unwrap();
        match process_operation(tr, &mut accounts, 1, &mut transactions){
            Ok(()) => {}
            Err(error) => panic!("{}", error),
        }
    }
    for (key, value) in expected.iter() {
        let expected_value = accounts.get(key);
        match expected_value{
            Some(acc) => {
                assert_eq!(acc.total,value.total);
                assert_eq!(acc.held,value.held);
                assert_eq!(acc.locked,value.locked);
                assert_eq!(acc.available,value.available);
            }
            None => assert!(false),
        }
    }
}
