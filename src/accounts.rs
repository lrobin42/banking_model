use rust_decimal::Decimal;
//use rust_decimal::dec;
use std::collections::HashMap;
#[derive(PartialEq)]
pub enum AccountType {
    Savings,
    Brokerage,
}
//change Positions to an option, enum doesn't work here
//pub enum Positions {    HashMap, //<String,f64>,None,}

pub struct Account {
    pub(crate) account_type: AccountType,
    pub(crate) account_holder: String,
    pub(crate) routing_number: i64, //10 digit
    pub(crate) account_number: i64, //18 digit
    pub(crate) cash_balance: f64,
    pub(crate) positions: Option<HashMap<String, f64>>,
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
            && sending_account.check_sufficient_balance(funds)
        {
            self.cash_balance += funds;
            sending_account.cash_balance -= funds;
        } else {
            println!("Transfer unsuccessful. Please check account information.")
        }
    }
    pub fn check_sufficient_balance(&self, funds: f64) -> bool {
        if self.cash_balance >= funds {
            true
        } else {
            false
        }
    }

    pub fn open_position(&mut self, ticker: &str, cash_amount: f64, share_price: f64) {
        //first make sure the account given is a Brokerage account.
        if self.account_type == AccountType::Savings {
            println!("Trade unsuccessful.Trades can only be made in Brokerage accounts.");
            return ();
        } else {
            //next check the balance in the account.
            let check_balance: bool = self.check_sufficient_balance(cash_amount);

            //if sufficient funds, calculate shares bought
            if check_balance {
                let shares_bought = cash_amount / share_price;
                self.cash_balance -= cash_amount;

                //if the account already has positions, check if the positions already include this
                //security
                if self.positions.is_some() {
                    let mut unpacked_positions: HashMap<String, f64> =
                        self.positions.as_mut().unwrap().clone();
                    if unpacked_positions.contains_key(ticker) {
                        let new_position_shares = unpacked_positions[ticker] + shares_bought;
                        unpacked_positions.insert(ticker.to_string(), new_position_shares);
                        self.positions = Some(unpacked_positions.clone());
                    } else {
                        unpacked_positions.insert(ticker.to_string(), shares_bought);
                        selt.positions = Some(unpacked_positions)
                    }
                    //if account_holder doesn't hold any positions, start a new one
                else if self.positions.is_none() || !unpacked_positions.contains_key(ticker) {
                        let mut investments: HashMap<String, f64> = HashMap::new();
                        investments.insert("ticker".to_string(), shares_bought);
                        self.positions = Some(investments);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_open_positions() {
        let savings_a = Account {
            account_type: AccountType::Savings,
            account_holder: "Marcus Mitchell".to_string(),
            routing_number: 50_498_023,
            account_number: 530_671_356_593_614_818,
            cash_balance: 38_346.45,
            positions: None,
        };
        let mut test_positions = HashMap::new();
        test_positions.insert("VTI".to_string(), 23.0443);

        let mut savings_b = Account {
            account_type: AccountType::Brokerage,
            account_holder: "Nathalia Martins".to_string(),
            routing_number: 87_491_858,
            account_number: 299_283_561_701_187_767,
            cash_balance: 57_652.66,
            positions: Some(test_positions),
        };

        savings_b.open_position("VOO", 3760.22, 435.07);
        assert_eq!(savings_b.positions.unwrap()["VOO"], 8.642793114);
    }
    //works if you are updating an existing VTI position.
    // doesn't work for unheld securities since it panics when key not found

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        // assert_eq!(bad_add(1, 2), 3);
    }
}
