mod accounts;
//use rust_decimal_macros::dec;
use crate::accounts::Account;
//use rust_decimal::Decimal
use crate::accounts::*;

fn main() {
    let mut savings_a = Account {
        account_type: accounts::AccountType::Savings,
        account_holder: "Marcus Mitchell".to_string(),
        routing_number: 50_498_023,
        account_number: 530_671_356_593_614_818,
        cash_balance: 38_346.45,
        positions: accounts::Positions::None,
    };

    let mut savings_b = Account {
        account_type: accounts::AccountType::Savings,
        account_holder: "Nathalia Martins".to_string(),
        routing_number: 87_491_858,
        account_number: 299_283_561_701_187_767,
        cash_balance: 57_652.66,
        positions: accounts::Positions::None,
    };

    //savings_a withdrawal
    let previous_balance = savings_a.cash_balance;
    savings_a.withdraw_funds(10_000.0);
    println!(
        "Previous balance was {}. New balance is {}",
        previous_balance, savings_a.cash_balance
    );
    savings_a.deposit_funds(10000.0);
    println!("Post-deposit balance is {}", savings_a.cash_balance);
    //
    //Marcus transfers to Nathalia
    println!(
        "Pre-transfer: Marcus - {:.2}, Nathalia {:.2}.",
        savings_a.cash_balance, savings_b.cash_balance
    );
    savings_a.transfer_funds_out(&mut savings_b, 20_000.0);
    println!(
        "Post-transfer Marcus - {:.2}, Nathalia - {:.2}",
        savings_a.cash_balance, savings_b.cash_balance
    );
    savings_a.transfer_funds_in(&mut savings_b, 20_000.0);
    println!(
        "Post-transfer Marcus - {:.2}, Nathalia - {:.2}",
        savings_a.cash_balance, savings_b.cash_balance
    );
    savings_b.check_balance();
}
//TODO
// creating new accounts
// implement rust crate for decimal math
// Build out enum for stock prices, shares, security type, and security type
//implement  functions for security_trades
// implement error handling for insufficient funds, ore incorrect routing numbers

//Bank Account Simulator: Model accounts with structs,
// and use enums for transaction types (Deposit, Withdraw, Transfer).
// This helps you define methods (impl blocks) and match on transaction requests.
