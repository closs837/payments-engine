# Rust Coding Test

Given a CSV representing a series of transactions, implement a simple toy transactions engine
that processes the payments crediting and debiting accounts. After processing the complete set
of payments output the client account balances
You should be able to run your payments engine like
$ cargo run -- transactions.csv > accounts.csv
The input file is the first and only argument to the binary. Output should be written to std out

# Instructions

To compile the program run the following:

```
cargo build
```
To run the test cases:

```
cargo test -- --show-output
```

To run the input file sample:

```
cargo run input_csv.csv
```

# Considerations:

```
    - CSV is read through BufReader, which "can improve the speed of programs that make small and repeated read calls to the same file or network socket".
    - Operations are processed line by line, which matches BufReader implementation.
    - All operations are functional.
    - Some errors are considered critical and interrupt execution (panic), due to possible invalid values or security issues. This should be changed in product to logging error:
        1) CSV Line with invalid operation (not a dispute, withdrawal, dispute, resolve, chargeback).
        2) Invalid Transaction ID (e.g. String)
        3) Invalid Amount value (e.g. String)
        4) Invalid Client ID (e.g. String)
        5) Conflicting transactions: one or more transactions with same ID
        6) Divergent dispute/chargeback/resolve and transaction id: client ID in transaction ID of dispute is different than client ID in stored transaction.
    - Some errors result in skipping the operation:
        1) Operating a locked account
        2) Dispute/Chargeback/Resolve with invalid Transaction ID (previous transaction ID not found)
        3) Resolve or Chargeback transaction that is not under dispute
        4) Account without funds for withdrawal
```
    
    


