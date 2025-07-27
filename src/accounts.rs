use rust_decimal::Decimal;
//use rust_decimal::dec;
use std::collections::HashMap;

pub enum AccountType {
    Savings,
    Brokerage,
}

pub enum Positions {
    HashMap, //<String,f64>,
    None,
}

pub struct Account {
    pub(crate) account_type: AccountType,
    pub(crate) account_holder: String,
    pub(crate) routing_number: i64, //10 digit
    pub(crate) account_number: i64, //18 digit
    pub(crate) cash_balance: f64,
    pub(crate) positions: Positions,
}
//Deposit, withdrawal, transfer, check balance, trade positions

impl Account {
    pub fn deposit_funds(&mut self, funds: f64) {
        self.cash_balance += funds;
    }

    pub fn withdraw_funds(&mut self, funds: f64) {
        let new_balance = self.cash_balance - funds;

        if new_balance >= 0.0 {
            self.cash_balance -= funds;
        } else {
        }
    }
    pub fn transfer_funds_out(&mut self, receiving_account: &mut Account, funds: f64) {
        if self.account_number != receiving_account.account_number && self.cash_balance >= funds {
            self.cash_balance -= funds;
            receiving_account.cash_balance += funds;
            //println!("Transfer successful!")
        } else {
            println!("Transfer unsuccessful. Please check account information.")
        }
    }
    pub fn transfer_funds_in(&mut self, sending_account: &mut Account, funds: f64) {
        if self.account_number != sending_account.account_number
            && sending_account.cash_balance >= funds
        {
            self.cash_balance += funds;
            sending_account.cash_balance -= funds;
        } else {
            println!("Transfer unsuccessful. Please check account information.")
        }
    }
    pub fn check_balance(&self) {
        println!("Current balance = {}", self.cash_balance)
    }
}

//use rust decimal crate for the cents math
