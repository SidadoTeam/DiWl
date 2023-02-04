#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod diwl_contract {

    use ink_env::debug_message;
    use ink_env::debug_println;
    use ink_prelude::string::*;
    use ink_prelude::vec::*;
    use ink_storage::traits::{PackedLayout, SpreadLayout};
    use scale::{Decode, Encode};

    //https://github.com/paritytech/cargo-contract/issues/158
    //https://docs.rs/ink_storage_derive/latest/ink_storage_derive/derive.PackedLayout.html
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[derive(Debug, Encode, Decode, SpreadLayout, PackedLayout, Clone)]
    pub struct WordList {
        word: String,
        level: i32,   //分级
        mean: String, //解释
        hit_count: i32,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DiwlContract {
        /// Stores a single `bool` value on the storage.
        value: bool,
        words: Vec<WordList>,
    }

    impl DiwlContract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                value: init_value,
                words: Vec::new(),
                hit_count: 0,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_env::debug_println!("hello");
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) -> bool {
            self.words.push(WordList {
                mean: String::from("你好"),
                level: 1,
                word: "hello".to_string(),
                hit_count: 0,
            });
            self.value = !self.value;
            self.value
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            debug_println!("hello");
            self.value
        }

        #[ink(message)]
        pub fn getw(&self) -> Vec<WordList> {
            self.words.clone()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let diwl_contract = DiwlContract::default();
            assert_eq!(diwl_contract.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut diwl_contract = DiwlContract::new(false);
            assert_eq!(diwl_contract.get(), false);
            diwl_contract.flip();
            assert_eq!(diwl_contract.get(), true);

            assert_eq!(
                diwl_contract.getw()[0].word,
                WordList {
                    word: "hello".to_string(),
                    mean: String::from("你好"),
                    level: 1,
                    hit_count: 0
                }
                .word
            );
        }
    }
}
