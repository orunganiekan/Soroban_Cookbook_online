#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
}

#[derive(Debug, PartialEq)]
#[contracttype]
pub enum Error {
    InsufficientBalance = 1,
    InvalidAmount = 2,
    SelfTransfer = 3,
}

#[contract]
pub struct TokenTransfer;

#[contractimpl]
impl TokenTransfer {
    /// Mint tokens to an address (for testing purposes).
    pub fn mint(env: Env, to: Address, amount: i128) {
        let key = DataKey::Balance(to.clone());
        let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(current + amount));
    }

    /// Return the balance of an address.
    pub fn balance(env: Env, of: Address) -> i128 {
        let key = DataKey::Balance(of);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Transfer tokens from one address to another.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        if from == to {
            return Err(Error::SelfTransfer);
        }

        let from_key = DataKey::Balance(from.clone());
        let from_balance: i128 = env.storage().persistent().get(&from_key).unwrap_or(0);

        if from_balance < amount {
            return Err(Error::InsufficientBalance);
        }

        let to_key = DataKey::Balance(to.clone());
        let to_balance: i128 = env.storage().persistent().get(&to_key).unwrap_or(0);

        env.storage()
            .persistent()
            .set(&from_key, &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&to_key, &(to_balance + amount));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, soroban_sdk::Address, TokenTransferClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(TokenTransfer, ());
        let client = TokenTransferClient::new(&env, &contract_id);
        (env, contract_id, client)
    }

    #[test]
    fn test_mint_increases_balance() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);

        client.mint(&alice, &500);
        assert_eq!(client.balance(&alice), 500);
    }

    #[test]
    fn test_transfer_moves_tokens() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        client.mint(&alice, &1000);
        client.transfer(&alice, &bob, &400);

        assert_eq!(client.balance(&alice), 600);
        assert_eq!(client.balance(&bob), 400);
    }

    #[test]
    fn test_transfer_fails_on_insufficient_balance() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        client.mint(&alice, &100);
        let result = client.try_transfer(&alice, &bob, &200);

        assert_eq!(result, Err(Ok(Error::InsufficientBalance)));
        // Verify state rolled back
        assert_eq!(client.balance(&alice), 100);
        assert_eq!(client.balance(&bob), 0);
    }

    #[test]
    fn test_transfer_fails_on_invalid_amount() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);

        client.mint(&alice, &100);
        let result = client.try_transfer(&alice, &bob, &0);
        assert_eq!(result, Err(Ok(Error::InvalidAmount)));

        let result = client.try_transfer(&alice, &bob, &-50);
        assert_eq!(result, Err(Ok(Error::InvalidAmount)));
    }

    #[test]
    fn test_self_transfer_is_rejected() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);

        client.mint(&alice, &100);
        let result = client.try_transfer(&alice, &alice, &50);
        assert_eq!(result, Err(Ok(Error::SelfTransfer)));
    }

    #[test]
    fn test_initial_balance_is_zero() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);

        assert_eq!(client.balance(&alice), 0);
    }
}
