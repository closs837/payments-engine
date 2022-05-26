use std::{collections::{HashMap, hash_map::Entry}};

use crate::{Input, Operations, AccountDetails, error_handling::Errors, Amount, AddAmount, Transactions};

pub fn process_operation(row: Input, accounts: &mut HashMap<u16,AccountDetails>, line: i32,transactions: &mut HashMap<u32,Transactions>)  -> Result<(), String> {
    let op_type = match row.op_type{
        Some(op) => op,
        None => return Err(Errors::InvalidOperation(line.to_string()).to_string()),
    };
    match row.client{
        Some(_) => {},
        None => return Err(Errors::InvalidClient(line.to_string()).to_string()),
    };
    match row.tx{
        Some(_) => {},
        None => return Err(Errors::InvalidTx(line.to_string()).to_string()),
    };
    match op_type {
        Operations::Deposit=> {
            match row.amount{
                Some(amount) => amount,
                None => return Err(Errors::InvalidAmount(line.to_string()).to_string()),
            };
            return process_deposit(row, accounts,transactions,line);
        },
        Operations::Withdrawal=> {
            match row.amount{
                Some(amount) => amount,
                None => return Err(Errors::InvalidAmount(line.to_string()).to_string()),
            };
            return process_withdrawal(row,accounts,transactions,line)
        },
        Operations::Dispute=> {
            process_dispute(row,accounts,transactions,line)
        },
        Operations::Resolve => {
            process_resolve(row,accounts,transactions,line)
        },
        Operations::Chargeback => {
            process_chargeback(row,accounts,transactions,line)
        }
    }
    
}

pub fn process_deposit(row: Input, accounts: &mut HashMap<u16,AccountDetails>,transactions: &mut HashMap<u32,Transactions>,line:i32)->Result<(), String> {
    let amount = Amount::get_amount(row.amount.unwrap());
    let client = row.client;
    match transactions.entry(row.tx.unwrap()) {
        Entry::Occupied(_e) => {
            return Err(Errors::ConflictTransaction(line.to_string()).to_string());
        },
        Entry::Vacant(e) => {
            //Add new transaction entry
            e.insert(Transactions { details: row, dispute: false });
        }
    }
    match accounts.entry(client.unwrap()) {
        Entry::Vacant(e) => {
            //Account not found
            //Adding new account
            e.insert(AccountDetails { available: amount, held: 0.0, total: amount, locked: false });
        },
        Entry::Occupied(mut e) => {
            //Account exists
            //Updating values
            if !e.get_mut().locked{
                e.get_mut().available = Amount::get_amount(e.get().available + amount);
                e.get_mut().total = Amount::get_amount(e.get().total + amount);
            }
        }
    }
    Ok(())
}

pub fn process_withdrawal(row: Input, accounts: &mut HashMap<u16,AccountDetails>,transactions: &mut HashMap<u32,Transactions>,line:i32)->Result<(), String> {
    let amount = Amount::get_amount(row.amount.unwrap());
    let client = row.client;
        match transactions.entry(row.tx.unwrap()) {
        Entry::Occupied(mut _e) => {
            return Err(Errors::ConflictTransaction(line.to_string()).to_string());
        },
        Entry::Vacant(e) => {
            //Add new transaction entry
            e.insert(Transactions { details: row, dispute: false });
        }
    }
    match accounts.entry(client.unwrap()) {
        Entry::Vacant(_e) => {
            //Account does not exist: withdrawal disconsidered
            return Ok(())
        },
        Entry::Occupied(mut e) => {
            //Account exists
            //Updating values
            if !e.get_mut().locked{
                //Check if account has funds
                if e.get_mut().available > amount{
                    e.get_mut().available = Amount::get_amount(e.get().available - amount);
                    e.get_mut().total = Amount::get_amount(e.get().total - amount);
                }
            }
            //else account locked
        }
    }
    Ok(())
}

pub fn process_dispute(row: Input, accounts: &mut HashMap<u16,AccountDetails>,transactions: &mut HashMap<u32,Transactions>,line:i32)->Result<(), String> {
    match transactions.entry(row.tx.unwrap()) {
        //Check if transaction exists
        Entry::Occupied(mut e) => {
            if e.get().details.client == row.client{
                //Check if clientId and tx in row == clientId and tx at HashMap 
                if !accounts.get(&row.client.unwrap()).unwrap().locked{
                    //Updating values: Dispute process
                    e.get_mut().dispute = true;
                    let new_account_value = accounts.get_mut(&row.client.unwrap()).unwrap();
                    new_account_value.available = Amount::get_amount(new_account_value.available - e.get().details.amount.unwrap());
                    new_account_value.held = Amount::get_amount(new_account_value.held + e.get().details.amount.unwrap());
                }
                //else account locked
            }else{
                return Err(Errors::SecurityErrDivergentClientId(line.to_string()).to_string())
            }
        },
        Entry::Vacant(_) => {
            //transaction does not exist, skip
        },
    }
    Ok(())
}

pub fn process_resolve(row: Input, accounts: &mut HashMap<u16,AccountDetails>,transactions: &mut HashMap<u32,Transactions>,line:i32)->Result<(), String> {
    match transactions.entry(row.tx.unwrap()) {
        //Check if transaction exists
        Entry::Occupied(mut e) => {
            match e.get().dispute{
                true => {
                    if e.get().details.client == row.client {
                        if !accounts.get(&row.client.unwrap()).unwrap().locked{
                            //Check if clientId and tx in row == clientId and tx at HashMap 
                            e.get_mut().dispute = false;
                            let new_account_value = accounts.get_mut(&row.client.unwrap()).unwrap();
                            new_account_value.available = Amount::get_amount(new_account_value.available + e.get().details.amount.unwrap());
                            new_account_value.held = Amount::get_amount(new_account_value.held - e.get().details.amount.unwrap());
                        }
                        //else account locked
                    }else{
                        return Err(Errors::SecurityErrDivergentClientId(line.to_string()).to_string())
                    }
                }
                false => {
                    //Transaction not under dispute yet, disconsider resolve
                },
            }
        },
        Entry::Vacant(_) => {
            //transaction does not exist, skip
        },
    }
    Ok(())
}

pub fn process_chargeback(row: Input, accounts: &mut HashMap<u16,AccountDetails>,transactions: &mut HashMap<u32,Transactions>,line:i32)->Result<(), String> {
    match transactions.entry(row.tx.unwrap()) {
        //check if transaction exists
        Entry::Occupied(mut e) => {
            match e.get().dispute{
                true => {
                    if e.get().details.client == row.client {
                        if !accounts.get(&row.client.unwrap()).unwrap().locked{
                        //check if clientId in row == clientId at HashMap 
                            e.get_mut().dispute = false;
                            let new_account_value = accounts.get_mut(&row.client.unwrap()).unwrap();
                            new_account_value.held = Amount::get_amount(new_account_value.held - e.get().details.amount.unwrap());
                            new_account_value.total = Amount::get_amount(new_account_value.total - e.get().details.amount.unwrap());
                            new_account_value.locked = true;
                        }
                        //else account locked
                    }else{
                        return Err(Errors::SecurityErrDivergentClientId(line.to_string()).to_string())
                    }
                }
                false => {
                    //Transaction not under dispute yet, disconsider resolve
                },
            }
        },
        Entry::Vacant(_) => {
            //transaction does not exist, skip
        },
    }
    Ok(())
}

