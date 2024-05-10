use soroban_sdk::{vec, Bytes, BytesN, Env, Symbol, IntoVal, BigInt};

#[derive(Clone)]
struct AccountBalance {
    pub account_id: BytesN,
    pub balance: BigInt, 
}

pub struct PaymentContract;

impl PaymentContract {
    
    pub fn initialize(env: Env, initial_balances: Vec<(BytesN, BigInt)>) -> Vec<AccountBalance> {
        let mut balances = Vec::new();
        for (account, balance) in initial_balances {
            balances.push(AccountBalance { account_id: account, balance });
        }
        balances
    }


    pub fn payment(&self, env: Env, from: BytesN, to: BytesN, amount: BigInt) {
        
        if amount <= BigInt::zero(&env) {
            panic!("Payment amount must be greater than zero");
        }

        let sender_balance = env.contract_data().get_unchecked(from.clone()).unwrap(); 
        if sender_balance.balance < amount {
            panic!("Insufficient balance in sender's account");
        }

         
        let mut from_balance: AccountBalance = sender_balance.unwrap_or(AccountBalance { 
            account_id: from.clone(), 
            balance: BigInt::zero(&env) 
        });
        let mut to_balance: AccountBalance = env.contract_data().get_unchecked(to.clone()).unwrap_or(AccountBalance { 
            account_id: to, 
            balance: BigInt::zero(&env) 
        });

        
        from_balance.balance -= amount;
        to_balance.balance += amount;

        
        env.contract_data().set(from, from_balance);
        env.contract_data().set(to, to_balance);
    }
}
