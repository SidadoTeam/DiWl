#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod diwl_contract {

    // use ink_env::debug_println;
    use ink_prelude::string::*;
    use ink_prelude::vec::*;
    use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};
    use scale::{Decode, Encode};

    //https://github.com/paritytech/cargo-contract/issues/158
    //https://docs.rs/ink_storage_derive/latest/ink_storage_derive/derive.PackedLayout.html
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[derive(Debug, Encode, Decode, SpreadLayout, PackedLayout, Clone, SpreadAllocate)]
    pub struct WordRecord {
        word: String,
        level: i32,   //分级
        mean: String, //解释
        hit_count: i32,
        tag: String, //标签 已逗号分隔
        nfts: Vec<String>,
    }

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[derive(Debug, Encode, Decode, SpreadLayout, PackedLayout, Clone, SpreadAllocate)]
    pub struct UserInfo {
        c_count: i32,
        auth_account: Vec<AccountId>, //授权账号 用于合并单词表
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DiwlContract {
        //单词通过用户id+序号的方式存储 通过序号进行遍历
        g_map: ink_storage::Mapping<String, WordRecord>,
        //存储用户信息 用户id下面有几个自定义单词
        user_info: ink_storage::Mapping<AccountId, UserInfo>,
        user_wlist: ink_storage::Mapping<(AccountId, i32), WordRecord>,
        c_count: i32, //公共词库 总的单词数量
        contract_owner: AccountId,
    }

    impl DiwlContract {
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.g_map = ink_storage::Mapping::default();
                contract.user_info = ink_storage::Mapping::default();
                contract.c_count = 0;
                contract.contract_owner = contract.env().caller();
            })
        }

        #[ink(message)]
        pub fn getw_user(&self, track_index: i32, track_limit: i32) -> Vec<WordRecord> {
            let mut w_list = Vec::new();
            //先取用户词库里的词
            let _info = self.user_info.get(self.env().caller());
            if _info.is_none() {
                return w_list;
            }
            let user_info = _info.unwrap();
            let start_count: i32 = (track_index - 1) * track_limit;
            let mut end_count = track_index * track_limit;
            if start_count >= user_info.c_count {
                return w_list;
            }
            if end_count > self.c_count {
                end_count = self.c_count;
            }
            for x in start_count..end_count {
                let w_record = self.user_wlist.get((self.env().caller(), x));
                if w_record.is_some() {
                    w_list.push(w_record.unwrap());
                }
            }
            w_list
        }

        #[ink(message)]
        pub fn getw_other_user(
            &self,
            other_id: AccountId,
            track_index: i32,
            track_limit: i32,
        ) -> Vec<WordRecord> {
            let mut w_list = Vec::new();

            let _info = self.user_info.get(other_id);
            if _info.is_none() {
                return w_list;
            }

            let user_info = _info.unwrap();
            //判断是否有权限
            if !user_info.auth_account.contains(&self.env().caller()) {
                return w_list;
            }
            let start_count: i32 = (track_index - 1) * track_limit;
            let mut end_count = track_index * track_limit;
            if start_count >= user_info.c_count {
                return w_list;
            }
            if end_count > self.c_count {
                end_count = self.c_count;
            }
            for x in start_count..end_count {
                let w_record = self.user_wlist.get((other_id, x));
                if w_record.is_some() {
                    w_list.push(w_record.unwrap());
                }
            }
            w_list
        }

        #[ink(message)]
        //进行分组查询 一次返回500个单词 page_index start with 1
        pub fn getw_common(&self, track_index: i32, track_limit: i32) -> Vec<WordRecord> {
            let mut w_list = Vec::new();

            let start_count: i32 = (track_index - 1) * track_limit;
            let mut end_count = track_index * track_limit;
            if start_count >= self.c_count {
                return w_list;
            }
            if end_count > self.c_count {
                end_count = self.c_count;
            }
            for x in start_count..end_count {
                let w_record = self.g_map.get(x.to_string());
                if w_record.is_some() {
                    w_list.push(w_record.unwrap());
                }
            }
            w_list
        }

        #[ink(message)]
        pub fn init_wlist(&mut self, word: WordRecord) {
            if self.env().caller() != self.contract_owner {
                // 权限检查
                return;
            }
            self.g_map.insert(self.c_count.to_string(), &word);
            self.c_count = self.c_count + 1;
        }

        #[ink(message)]
        pub fn user_word_in(&mut self, word: WordRecord) {
            let _user_info = self.user_info.get(self.env().caller());
            let mut user_info;
            if _user_info.is_none() {
                user_info = UserInfo {
                    c_count: 0,
                    auth_account: Vec::new(),
                };
                self.user_info.insert(self.env().caller(), &user_info);
            }
            user_info = _user_info.unwrap();
            self.user_wlist
                .insert((self.env().caller(), user_info.c_count), &word);
            user_info.c_count = user_info.c_count + 1;
        }

        #[ink(message)]
        pub fn auth_account(&self, taget: AccountId) {
            let user_info = self.user_info.get(self.env().caller());
            if user_info.is_none() {
                return;
            }
            let mut user = user_info.unwrap();
            user.auth_account.push(taget);
        }
    }
}
