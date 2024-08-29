use near_sdk::store::LookupSet;
use near_sdk::Promise;
use near_sdk::{
    env, near, require, AccountId, BlockHeight, BorshStorageKey, NearToken, PanicOnDefault,
};

const NUM_SHARDS: u8 = 5;
const STORAGE_BUFFER: NearToken = NearToken::from_yoctonear(29 * 10_000_000_000_000_000_000u128);

#[derive(BorshStorageKey)]
#[near]
enum StorageKeys {
    Claims,
}

#[derive(Default, Ord, PartialOrd, Eq, PartialEq)]
#[near]
pub struct PartialHash(pub [u8; 20]);

impl From<&AccountId> for PartialHash {
    fn from(account_id: &AccountId) -> Self {
        let mut hash: Self = Default::default();
        hash.0
            .copy_from_slice(&env::sha256(account_id.as_bytes())[0..20]);
        hash
    }
}

#[derive(PanicOnDefault)]
#[near(contract_state)]
pub struct Contract {
    claims: LookupSet<PartialHash>,
    transfer_amount: NearToken,
    approved_shard: u8,
    start_block_height: BlockHeight,
    num_claims: u32,
}

#[near]
impl Contract {
    #[init]
    pub fn new(
        transfer_amount: NearToken,
        approved_shard: u8,
        start_block_height: BlockHeight,
    ) -> Self {
        Self {
            claims: LookupSet::new(StorageKeys::Claims),
            transfer_amount,
            approved_shard,
            start_block_height,
            num_claims: 0,
        }
    }

    pub fn get_approved_shard(&self) -> u8 {
        self.approved_shard
    }

    pub fn get_transfer_amount(&self) -> NearToken {
        self.transfer_amount
    }

    pub fn get_start_block_height(&self) -> BlockHeight {
        self.start_block_height
    }

    pub fn get_num_claims(&self) -> u32 {
        self.num_claims
    }

    pub fn estimate_number_of_possible_claims(&self) -> u128 {
        self.get_remaining_balance().as_yoctonear()
            / self
                .transfer_amount
                .checked_add(STORAGE_BUFFER)
                .unwrap()
                .as_yoctonear()
    }

    pub fn can_claim(&self, account_id: &AccountId, block_height: Option<BlockHeight>) -> bool {
        self.is_enough_for_a_claim()
            && self.get_account_shard(account_id) == self.approved_shard
            && !self.claims.contains(&PartialHash::from(account_id))
            && block_height.unwrap_or_else(env::block_height) >= self.start_block_height
    }

    pub fn get_account_shard(&self, account_id: &AccountId) -> u8 {
        let hash = env::sha256(account_id.as_bytes());
        hash[0] % NUM_SHARDS
    }

    pub fn get_current_block_height(&self) -> BlockHeight {
        env::block_height()
    }

    pub fn get_remaining_balance(&self) -> NearToken {
        env::account_balance()
            .checked_sub(
                env::storage_byte_cost()
                    .checked_mul(env::storage_usage() as u128)
                    .unwrap(),
            )
            .unwrap()
    }

    pub fn is_enough_for_a_claim(&self) -> bool {
        self.get_remaining_balance() >= self.transfer_amount.checked_add(STORAGE_BUFFER).unwrap()
    }

    pub fn claim(&mut self) -> Promise {
        require!(
            env::block_height() >= self.start_block_height,
            "Too early to claim"
        );
        let account_id = env::predecessor_account_id();
        assert_eq!(
            self.get_account_shard(&account_id),
            self.approved_shard,
            "Invalid shard"
        );
        require!(
            self.claims.insert(PartialHash::from(&account_id)),
            "Already claimed"
        );
        self.num_claims += 1;
        Promise::new(account_id).transfer(self.transfer_amount)
    }
}