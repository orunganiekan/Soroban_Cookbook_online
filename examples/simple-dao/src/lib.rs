#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env};

/// Proposal lifecycle states.
///
/// Valid transitions:
/// ```text
/// Pending → Active → Succeeded → Executed
///                  ↘ Defeated
/// ```
#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum ProposalState {
    Pending,
    Active,
    Succeeded,
    Defeated,
    Executed,
}

#[contracttype]
pub enum DataKey {
    State,
    VotesFor,
    VotesAgainst,
}

#[contract]
pub struct SimpleDao;

#[contractimpl]
impl SimpleDao {
    /// Initialize a new proposal in the Pending state.
    pub fn create(env: Env) {
        env.storage().instance().set(&DataKey::State, &ProposalState::Pending);
        env.storage().instance().set(&DataKey::VotesFor, &0_u32);
        env.storage().instance().set(&DataKey::VotesAgainst, &0_u32);
    }

    /// Move the proposal from Pending → Active (voting opens).
    pub fn activate(env: Env) {
        let state: ProposalState = env.storage().instance().get(&DataKey::State).unwrap();
        assert!(state == ProposalState::Pending, "must be Pending to activate");
        env.storage().instance().set(&DataKey::State, &ProposalState::Active);
    }

    /// Cast a vote while the proposal is Active.
    pub fn vote(env: Env, in_favor: bool) {
        let state: ProposalState = env.storage().instance().get(&DataKey::State).unwrap();
        assert!(state == ProposalState::Active, "voting is not open");
        if in_favor {
            let v: u32 = env.storage().instance().get(&DataKey::VotesFor).unwrap_or(0);
            env.storage().instance().set(&DataKey::VotesFor, &(v + 1));
        } else {
            let v: u32 = env.storage().instance().get(&DataKey::VotesAgainst).unwrap_or(0);
            env.storage().instance().set(&DataKey::VotesAgainst, &(v + 1));
        }
    }

    /// Close voting: Active → Succeeded or Defeated based on vote counts.
    pub fn finalize(env: Env) {
        let state: ProposalState = env.storage().instance().get(&DataKey::State).unwrap();
        assert!(state == ProposalState::Active, "must be Active to finalize");
        let for_votes: u32 = env.storage().instance().get(&DataKey::VotesFor).unwrap_or(0);
        let against_votes: u32 = env.storage().instance().get(&DataKey::VotesAgainst).unwrap_or(0);
        let next = if for_votes > against_votes {
            ProposalState::Succeeded
        } else {
            ProposalState::Defeated
        };
        env.storage().instance().set(&DataKey::State, &next);
    }

    /// Execute a Succeeded proposal → Executed.
    pub fn execute(env: Env) {
        let state: ProposalState = env.storage().instance().get(&DataKey::State).unwrap();
        assert!(state == ProposalState::Succeeded, "must be Succeeded to execute");
        env.storage().instance().set(&DataKey::State, &ProposalState::Executed);
    }

    /// Return the current proposal state.
    pub fn get_state(env: Env) -> ProposalState {
        env.storage().instance().get(&DataKey::State).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    fn setup() -> (Env, soroban_sdk::Address) {
        let env = Env::default();
        let id = env.register(SimpleDao, ());
        (env, id)
    }

    #[test]
    fn test_create_sets_pending() {
        let (env, id) = setup();
        let client = SimpleDaoClient::new(&env, &id);
        client.create();
        assert_eq!(client.get_state(), ProposalState::Pending);
    }

    #[test]
    fn test_pending_to_active() {
        let (env, id) = setup();
        let client = SimpleDaoClient::new(&env, &id);
        client.create();
        client.activate();
        assert_eq!(client.get_state(), ProposalState::Active);
    }

    #[test]
    fn test_active_to_succeeded() {
        let (env, id) = setup();
        let client = SimpleDaoClient::new(&env, &id);
        client.create();
        client.activate();
        client.vote(&true);
        client.vote(&true);
        client.vote(&false);
        client.finalize();
        assert_eq!(client.get_state(), ProposalState::Succeeded);
    }

    #[test]
    fn test_active_to_defeated() {
        let (env, id) = setup();
        let client = SimpleDaoClient::new(&env, &id);
        client.create();
        client.activate();
        client.vote(&false);
        client.vote(&false);
        client.vote(&true);
        client.finalize();
        assert_eq!(client.get_state(), ProposalState::Defeated);
    }

    #[test]
    fn test_succeeded_to_executed() {
        let (env, id) = setup();
        let client = SimpleDaoClient::new(&env, &id);
        client.create();
        client.activate();
        client.vote(&true);
        client.finalize();
        client.execute();
        assert_eq!(client.get_state(), ProposalState::Executed);
    }

    #[test]
    #[should_panic(expected = "must be Pending to activate")]
    fn test_cannot_activate_active_proposal() {
        let (env, id) = setup();
        let client = SimpleDaoClient::new(&env, &id);
        client.create();
        client.activate();
        client.activate(); // should panic
    }

    #[test]
    #[should_panic(expected = "voting is not open")]
    fn test_cannot_vote_when_pending() {
        let (env, id) = setup();
        let client = SimpleDaoClient::new(&env, &id);
        client.create();
        client.vote(&true); // should panic
    }

    #[test]
    #[should_panic(expected = "must be Succeeded to execute")]
    fn test_cannot_execute_defeated_proposal() {
        let (env, id) = setup();
        let client = SimpleDaoClient::new(&env, &id);
        client.create();
        client.activate();
        client.vote(&false);
        client.finalize();
        client.execute(); // should panic
    }

    #[test]
    #[should_panic(expected = "must be Active to finalize")]
    fn test_cannot_finalize_pending_proposal() {
        let (env, id) = setup();
        let client = SimpleDaoClient::new(&env, &id);
        client.create();
        client.finalize(); // should panic
    }
}
