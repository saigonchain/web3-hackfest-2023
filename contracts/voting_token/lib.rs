#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink_lang::contract;

#[contract]
mod voting_token {
    use ink_prelude::string::String;
    use ink_storage::collections::HashMap as StorageHashMap;
    use ink_storage::traits::{PackedLayout, SpreadLayout};
    use ink_storage::traits::{StorageLayout, StorageNest};

    /// Definition of the VotingToken contract.
    #[ink(storage)]
    pub struct VotingToken {
        name: String,
        symbol: String,
        total_supply: u256,
        balance_of: StorageHashMap<AccountId, u256>,
        allowance: StorageHashMap<(AccountId, AccountId), u256>,
        deposit_of: StorageHashMap<AccountId, u256>,
    }

    impl VotingToken {
        /// Constructor to initialize the VotingToken contract.
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, initial_supply: u256) -> Self {
            let caller = Self::env().caller();
            let mut balance_of = StorageHashMap::new();
            let mut deposit_of = StorageHashMap::new();
            balance_of.insert(caller, initial_supply);
            deposit_of.insert(caller, 0);

            Self {
                name,
                symbol,
                total_supply: initial_supply,
                balance_of,
                allowance: Default::default(),
                deposit_of,
            }
        }

        /// Deposit ETH to receive VotingTokens.
        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            let caller = Self::env().caller();
            let deposit_amount = Self::env().transferred_balance();
            self.deposit_of.insert(caller, deposit_amount);

            // 0.1 ETH = 1000 VotingTokens
            let total_tokens_received = deposit_amount * 1000 / (u256::from(10).pow(18));
            let balance = self.balance_of.entry(caller).or_insert(0);
            *balance = total_tokens_received;
            self.total_supply += total_tokens_received;
        }

        /// Transfer VotingTokens to another account.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: u256) -> bool {
            self._transfer(Self::env().caller(), to, value)
        }

        /// Internal transfer function.
        fn _transfer(&mut self, from: AccountId, to: AccountId, value: u256) -> bool {
            let from_balance = self.balance_of.get(&from).copied().unwrap_or(0);
            let to_balance = self.balance_of.get(&to).copied().unwrap_or(0);
            if from_balance < value {
                return false;
            }

            self.balance_of.insert(from, from_balance - value);
            self.balance_of.insert(to, to_balance + value);
            true
        }

        /// Approve another account to spend tokens on your behalf.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: u256) -> bool {
            let owner = Self::env().caller();
            self.allowance.insert((owner, spender), value);
            true
        }

        /// Transfer tokens from one account to another using the allowance mechanism.
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u256) -> bool {
            let allowance = self.allowance.get(&(from, Self::env().caller())).copied().unwrap_or(0);
            if allowance < value {
                return false;
            }

            self.allowance.insert((from, Self::env().caller()), allowance - value);
            self._transfer(from, to, value)
        }

        /// Get the name of the token.
        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        /// Get the symbol of the token.
        #[ink(message)]
        pub fn get_symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Get the total supply of the token.
        #[ink(message)]
        pub fn get_total_supply(&self) -> u256 {
            self.total_supply
        }

        /// Get the balance of an account.
        #[ink(message)]
        pub fn get_balance(&self, account: AccountId) -> u256 {
            self.balance_of.get(&account).copied().unwrap_or(0)
        }

        /// Get the allowance for a spender on behalf of an owner.
        #[ink(message)]
        pub fn get_allowance(&self, owner: AccountId, spender: AccountId) -> u256 {
            self.allowance.get(&(owner, spender)).copied().unwrap_or(0)
        }

        /// Get the deposit amount for an account.
        #[ink(message)]
        pub fn get_deposit(&self, account: AccountId) -> u256 {
            self.deposit_of.get(&account).copied().unwrap_or(0)
        }
    }

    /// Required trait implementations for PackedLayout.
    impl PackedLayout for VotingToken {
        fn pack(&self) -> Vec<u8> {
            PackedLayout::pack(&(
                &self.name,
                &self.symbol,
                &self.total_supply,
                &self.balance_of,
                &self.allowance,
                &self.deposit_of,
            ))
        }

        fn unpack_from_slice(buf: &[u8]) -> Self {
            let (
                name,
                symbol,
                total_supply,
                balance_of,
                allowance,
                deposit_of,
            ) = PackedLayout::unpack_from_slice(buf);
            Self {
                name,
                symbol,
                total_supply,
                balance_of,
                allowance,
                deposit_of,
            }
        }
    }

    /// Required trait implementations for SpreadLayout.
    impl SpreadLayout for VotingToken {
        fn push_spread(&self, sp: &mut ink_storage::collections::SpreadLayoutStream) {
            sp.push_spread(&self.name);
            sp.push_spread(&self.symbol);
            sp.push_spread(&self.total_supply);
            sp.push_spread(&self.balance_of);
            sp.push_spread(&self.allowance);
            sp.push_spread(&self.deposit_of);
        }

        fn pull_spread(sp: &mut ink_storage::collections::SpreadLayoutStream) -> Self {
            Self {
                name: sp.pull_spread(),
                symbol: sp.pull_spread(),
                total_supply: sp.pull_spread(),
                balance_of: sp.pull_spread(),
                allowance: sp.pull_spread(),
                deposit_of: sp.pull_spread(),
            }
        }
    }

    /// Required trait implementations for StorageLayout.
    impl StorageLayout for VotingToken {
        fn layout(key_ptr: &mut ink_storage::traits::KeyPtr) {
            ink_storage::traits::PackedLayout::layout(key_ptr);
        }
    }
}
