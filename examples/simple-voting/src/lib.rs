#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Proposal(u32),
    Vote(u32, Address),
    ProposalCount,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Choice {
    Yes,
    No,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub yes_votes: u32,
    pub no_votes: u32,
    pub is_active: bool,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ProposalNotFound = 1,
    AlreadyVoted = 2,
    ProposalClosed = 3,
}

#[contract]
pub struct SimpleVoting;

#[contractimpl]
impl SimpleVoting {
    /// Create a new proposal and return its ID.
    pub fn create_proposal(env: Env, title: String) -> u32 {
        let count_key = DataKey::ProposalCount;
        let id: u32 = env.storage().instance().get(&count_key).unwrap_or(0);
        let proposal = Proposal {
            id,
            title,
            yes_votes: 0,
            no_votes: 0,
            is_active: true,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Proposal(id), &proposal);
        env.storage().instance().set(&count_key, &(id + 1));
        id
    }

    /// Cast a yes or no vote. Each address may vote only once per proposal.
    pub fn vote(
        env: Env,
        voter: Address,
        proposal_id: u32,
        choice: Choice,
    ) -> Result<(), Error> {
        voter.require_auth();

        let vote_key = DataKey::Vote(proposal_id, voter.clone());
        if env.storage().persistent().has(&vote_key) {
            return Err(Error::AlreadyVoted);
        }

        let proposal_key = DataKey::Proposal(proposal_id);
        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&proposal_key)
            .ok_or(Error::ProposalNotFound)?;

        if !proposal.is_active {
            return Err(Error::ProposalClosed);
        }

        match choice {
            Choice::Yes => proposal.yes_votes += 1,
            Choice::No => proposal.no_votes += 1,
        }

        env.storage().persistent().set(&proposal_key, &proposal);
        env.storage().persistent().set(&vote_key, &choice);

        Ok(())
    }

    /// Close a proposal so no further votes are accepted.
    pub fn close_proposal(env: Env, proposal_id: u32) -> Result<(), Error> {
        let key = DataKey::Proposal(proposal_id);
        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::ProposalNotFound)?;
        proposal.is_active = false;
        env.storage().persistent().set(&key, &proposal);
        Ok(())
    }

    /// Return the current vote tally for a proposal.
    pub fn tally(env: Env, proposal_id: u32) -> Result<Proposal, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .ok_or(Error::ProposalNotFound)
    }

    /// Return how a specific address voted, or None if they have not voted.
    pub fn get_vote(env: Env, voter: Address, proposal_id: u32) -> Option<Choice> {
        env.storage()
            .persistent()
            .get(&DataKey::Vote(proposal_id, voter))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    fn setup() -> (Env, soroban_sdk::Address, SimpleVotingClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(SimpleVoting, ());
        let client = SimpleVotingClient::new(&env, &contract_id);
        (env, contract_id, client)
    }

    #[test]
    fn test_create_proposal_returns_first_id() {
        let (env, _, client) = setup();
        let title = String::from_str(&env, "Should we add feature X?");
        let id = client.create_proposal(&title);
        assert_eq!(id, 0);
    }

    #[test]
    fn test_proposal_ids_increment() {
        let (env, _, client) = setup();
        let id0 = client.create_proposal(&String::from_str(&env, "Proposal A"));
        let id1 = client.create_proposal(&String::from_str(&env, "Proposal B"));
        let id2 = client.create_proposal(&String::from_str(&env, "Proposal C"));
        assert_eq!(id0, 0);
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_vote_yes_increments_yes_count() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);
        let id = client.create_proposal(&String::from_str(&env, "Fund the grant?"));

        client.vote(&alice, &id, &Choice::Yes);

        let proposal = client.tally(&id);
        assert_eq!(proposal.yes_votes, 1);
        assert_eq!(proposal.no_votes, 0);
    }

    #[test]
    fn test_vote_no_increments_no_count() {
        let (env, _, client) = setup();
        let bob = Address::generate(&env);
        let id = client.create_proposal(&String::from_str(&env, "Change the logo?"));

        client.vote(&bob, &id, &Choice::No);

        let proposal = client.tally(&id);
        assert_eq!(proposal.yes_votes, 0);
        assert_eq!(proposal.no_votes, 1);
    }

    #[test]
    fn test_tally_reflects_all_votes() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let carol = Address::generate(&env);
        let dave = Address::generate(&env);

        let id = client.create_proposal(&String::from_str(&env, "Upgrade protocol?"));

        client.vote(&alice, &id, &Choice::Yes);
        client.vote(&bob, &id, &Choice::Yes);
        client.vote(&carol, &id, &Choice::No);
        client.vote(&dave, &id, &Choice::Yes);

        let proposal = client.tally(&id);
        assert_eq!(proposal.yes_votes, 3);
        assert_eq!(proposal.no_votes, 1);
    }

    #[test]
    fn test_one_vote_per_address() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);
        let id = client.create_proposal(&String::from_str(&env, "Double vote attempt"));

        client.vote(&alice, &id, &Choice::Yes);
        let result = client.try_vote(&alice, &id, &Choice::No);

        assert_eq!(result, Err(Ok(Error::AlreadyVoted)));

        let proposal = client.tally(&id);
        assert_eq!(proposal.yes_votes, 1);
        assert_eq!(proposal.no_votes, 0);
    }

    #[test]
    fn test_vote_on_nonexistent_proposal() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);
        let result = client.try_vote(&alice, &99, &Choice::Yes);
        assert_eq!(result, Err(Ok(Error::ProposalNotFound)));
    }

    #[test]
    fn test_tally_on_nonexistent_proposal() {
        let (_, _, client) = setup();
        let result = client.try_tally(&99);
        assert!(matches!(result, Err(Ok(Error::ProposalNotFound))));
    }

    #[test]
    fn test_vote_on_closed_proposal() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);
        let id = client.create_proposal(&String::from_str(&env, "Close me"));

        client.close_proposal(&id);
        let result = client.try_vote(&alice, &id, &Choice::Yes);

        assert_eq!(result, Err(Ok(Error::ProposalClosed)));
    }

    #[test]
    fn test_close_proposal_marks_inactive() {
        let (env, _, client) = setup();
        let id = client.create_proposal(&String::from_str(&env, "To be closed"));

        let before = client.tally(&id);
        assert!(before.is_active);

        client.close_proposal(&id);

        let after = client.tally(&id);
        assert!(!after.is_active);
    }

    #[test]
    fn test_get_vote_returns_none_before_voting() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);
        let id = client.create_proposal(&String::from_str(&env, "Empty ballot"));

        assert_eq!(client.get_vote(&alice, &id), None);
    }

    #[test]
    fn test_get_vote_returns_choice_after_voting() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let id = client.create_proposal(&String::from_str(&env, "Record the vote"));

        client.vote(&alice, &id, &Choice::Yes);
        client.vote(&bob, &id, &Choice::No);

        assert_eq!(client.get_vote(&alice, &id), Some(Choice::Yes));
        assert_eq!(client.get_vote(&bob, &id), Some(Choice::No));
    }

    #[test]
    fn test_votes_are_independent_across_proposals() {
        let (env, _, client) = setup();
        let alice = Address::generate(&env);

        let id0 = client.create_proposal(&String::from_str(&env, "Proposal 0"));
        let id1 = client.create_proposal(&String::from_str(&env, "Proposal 1"));

        client.vote(&alice, &id0, &Choice::Yes);
        client.vote(&alice, &id1, &Choice::No);

        let p0 = client.tally(&id0);
        let p1 = client.tally(&id1);

        assert_eq!(p0.yes_votes, 1);
        assert_eq!(p0.no_votes, 0);
        assert_eq!(p1.yes_votes, 0);
        assert_eq!(p1.no_votes, 1);
    }
}
