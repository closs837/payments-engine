# Payments engine

A small transaction-processing engine in Rust, written for a take-home exercise: it reads
a CSV of payment operations and prints the resulting account balances.

```
cargo run -- input_test.csv > accounts.csv
```

The input file is the only argument; output goes to stdout as
`client, available, held, total, locked`.

## What it handles

- `deposit` and `withdrawal` against a client account, with held funds kept separate from
  available funds.
- `dispute`, `resolve` and `chargeback` against an earlier transaction id, including the
  freeze that a dispute puts on the disputed amount.
- Bad input is rejected instead of guessed at: unknown operation types, non-numeric
  amounts, duplicate or unknown transaction ids, and operations on locked accounts are
  skipped or reported as errors.

## Layout

| Path | Purpose |
| --- | --- |
| `src/csv_process.rs` | CSV reader and row model |
| `src/operations.rs` | Operation handlers and account bookkeeping |
| `src/error_handling.rs` | Error types |
| `src/lib.rs` | Entry point and output formatting |

`serde_derive` is vendored under `serde/` so the exercise builds without network access;
`input_test.csv` is a small sample to run against.

`cargo test` runs the unit tests.
