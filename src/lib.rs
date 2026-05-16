#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct FundingIntent {
    pub id: String,
    pub merchant: Address,
    pub destination: Address,
    pub asset_code: String,
    pub amount: i128,
    pub expires_at_ledger: u32,
    pub settled: bool,
}

#[contracttype]
pub enum DataKey {
    Intent(String),
}

#[contract]
pub struct OrbitPayFunding;

#[contractimpl]
impl OrbitPayFunding {
    pub fn create_intent(
        env: Env,
        id: String,
        merchant: Address,
        destination: Address,
        asset_code: String,
        amount: i128,
        expires_at_ledger: u32,
    ) -> FundingIntent {
        merchant.require_auth();

        if amount <= 0 {
            panic!("amount must be positive");
        }

        let intent = FundingIntent {
            id: id.clone(),
            merchant,
            destination,
            asset_code,
            amount,
            expires_at_ledger,
            settled: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Intent(id.clone()), &intent);
        env.events().publish((Symbol::new(&env, "intent_created"), id), ());
        intent
    }

    pub fn get_intent(env: Env, id: String) -> FundingIntent {
        env.storage()
            .persistent()
            .get(&DataKey::Intent(id))
            .expect("intent not found")
    }

    pub fn mark_settled(env: Env, id: String, merchant: Address) -> FundingIntent {
        merchant.require_auth();

        let key = DataKey::Intent(id.clone());
        let mut intent: FundingIntent = env
            .storage()
            .persistent()
            .get(&key)
            .expect("intent not found");

        if intent.merchant != merchant {
            panic!("merchant mismatch");
        }

        if intent.settled {
            panic!("intent already settled");
        }

        intent.settled = true;
        env.storage().persistent().set(&key, &intent);
        env.events().publish((Symbol::new(&env, "intent_settled"), id), ());
        intent
    }

    pub fn cancel_expired(env: Env, id: String) -> FundingIntent {
        let key = DataKey::Intent(id.clone());
        let intent: FundingIntent = env
            .storage()
            .persistent()
            .get(&key)
            .expect("intent not found");

        if env.ledger().sequence() <= intent.expires_at_ledger {
            panic!("intent still active");
        }

        env.storage().persistent().remove(&key);
        env.events().publish((Symbol::new(&env, "intent_cancelled"), id), ());
        intent
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn creates_and_reads_intent() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, OrbitPayFunding);
        let client = OrbitPayFundingClient::new(&env, &contract_id);
        let merchant = Address::generate(&env);
        let destination = Address::generate(&env);

        let intent = client.create_intent(
            &String::from_str(&env, "op-1042"),
            &merchant,
            &destination,
            &String::from_str(&env, "USDC"),
            &45_0000000,
            &5000,
        );

        assert_eq!(intent.amount, 45_0000000);
        assert_eq!(intent.settled, false);

        let stored = client.get_intent(&String::from_str(&env, "op-1042"));
        assert_eq!(stored.asset_code, String::from_str(&env, "USDC"));
    }
}

