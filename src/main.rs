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
        positions: None,
    };

    let mut savings_b = Account {
        account_type: accounts::AccountType::Savings,
        account_holder: "Nathalia Martins".to_string(),
        routing_number: 87_491_858,
        account_number: 299_283_561_701_187_767,
        cash_balance: 57_652.66,
        positions: None,
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
    println!("Nathalia's final balance: ${}", savings_b.cash_balance);
}
