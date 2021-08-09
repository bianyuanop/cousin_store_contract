#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
mod cousin_tea {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct CousinTea {
        /// Contract owner
        owner: AccountId,
        commody_incre: u32,
        order_incre: u32,
        //// commody_store HashMap<CommodyId>
        commody_store: ink_storage::collections::Vec<u32>,
        commody_info: ink_storage::collections::HashMap<u32, ink_prelude::string::String>,
        //// commody_store HashMap<OrderInfoId, AccountId>
        /// All the judge function leave with front-end to finish
        orders: ink_storage::collections::HashMap<u32, AccountId>,
        orders_info: ink_storage::collections::HashMap<u32, ink_prelude::string::String>,
    }

    impl CousinTea {
        
        /// Constructor that initializes the `AccountId` value to the given `init_owner`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let chair_man = Self::env().caller();
            Self {
                 owner: chair_man,
                 commody_incre: 0,
                 order_incre: 0,
                 commody_store: ink_storage::collections::Vec::new(),
                 commody_info: ink_storage::collections::HashMap::new(),
                 orders: ink_storage::collections::HashMap::new(),
                 orders_info: ink_storage::collections::HashMap::new(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        /// Error code 
        /// 0 -> add successfully
        /// 1 -> not owner, no privileges
        #[ink(message)]
        pub fn add_commody(&mut self, commody_info: ink_prelude::string::String) -> u8 {
            //let caller = Self::env().caller();
            let caller = self.env().caller();
            if self.owner != caller {
                return 1
            }
            self.commody_store.push(self.commody_incre);
            self.commody_info.insert(self.commody_incre, commody_info);
            self.commody_incre += 1;
            0
        }

        /// 0 -> no error
        /// 1 -> not owner no privileges
        /// 2 -> no such commody
        /// 3 -> if happend this, god did it
        #[ink(message)]
        pub fn alter_commody(&mut self, commody_id: u32, commody_info: ink_prelude::string::String) -> u8 {
            let caller = self.env().caller();
            if self.owner != caller {
                return 1;
            }
            if commody_id > self.commody_incre {
                return 2;
            }

            let comm_on_serving = self.commody_info.get_mut(&commody_id);
            match comm_on_serving {
                Some(commody) => {
                    *commody = commody_info;
                },
                None => {
                    return 3;
                }
            }
            0
        }

        #[ink(message)]
        pub fn get_commody(&self, commody_id: u32) -> ink_prelude::string::String {
            let failed_res = ink_prelude::string::String::from("");
            let comm_info = self.commody_info.get(&commody_id).unwrap_or(&failed_res);

            ink_prelude::string::String::from(comm_info)
        }

        #[ink(message)]
        pub fn commody_count(&mut self) -> u32 {
            self.commody_incre
        }

        #[ink(message)]
        pub fn add_order(&mut self, order_info: ink_prelude::string::String) -> u32 {
            let client = self.env().caller();
            self.orders.insert(self.order_incre, client);
            self.orders_info.insert(self.order_incre, order_info); 
            self.order_incre += 1;

            self.order_incre - 1 
        }

        /// Only contract owner and client themself can reveal the order information
        #[ink(message)]
        pub fn get_order(&self, order_id: u32) -> ink_prelude::string::String {
            let failed_res = ink_prelude::string::String::from("");
            let caller = self.env().caller();
            if caller == self.owner {
                let order_info = self.orders_info.get(&order_id).unwrap_or(&failed_res);
                return ink_prelude::string::String::from(order_info);
            } else {
                let client = self.orders.get(&order_id);
                if let Some(acc) = client {
                    if *acc == caller {
                        let order_info = self.orders_info.get(&order_id).unwrap_or(&failed_res);
                        return ink_prelude::string::String::from(order_info);
                    }
                }
            }

            ink_prelude::string::String::from("")
        }


        #[ink(message)]
        pub fn order_count(&mut self) -> u32 {
            self.order_incre
        }

        /// 0 -> no error occur
        /// 1 -> account not correct
        /// 2 -> no such order
        /// 3 -> order info not exists
        #[ink(message)]
        pub fn alter_order(&mut self, order_id: u32, altered_info: ink_prelude::string::String) -> u8 {
            let client = self.env().caller();
            let order = self.orders.get(&order_id);
            match order {
                Some(account) => {
                    ink_env::debug_println!("account: {:?}, client: {:?}", account, client);
                    if *account != client {
                        return 1
                    }
                },
                None => return 2
            }

            let order_info = self.orders_info.get_mut(&order_id);
            match order_info {
                Some(info) => {
                    *info = altered_info;
                },
                None => return 3
            } 
            0
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
            let cousin_tea = CousinTea::default();
            let owner = cousin_tea.owner;
            assert_eq!(cousin_tea.owner, owner);
        }

        #[ink::test]
        fn order_works() {
            let mut cousin_tea = CousinTea::default();
            let order_info = ink_prelude::string::String::from("Some order info in json");
            // add order
            let order_add_result = cousin_tea.add_order(order_info);
            assert_eq!(order_add_result, 0);

            // alter order
            let altered_info = ink_prelude::string::String::from("Some alterd info");
            let alter_order_result = cousin_tea.alter_order(0, altered_info.clone());
            assert_eq!(alter_order_result, 0);

            let count = cousin_tea.order_count();
            assert_eq!(count, 1);

            // get order
            let order_info = cousin_tea.get_order(0);
            assert_eq!(order_info, altered_info);
        }
        #[ink::test]
        fn commody_works() {
            let mut cousin_tea = CousinTea::default();
            let commody_info = ink_prelude::string::String::from("Some commody");
            // add commody
            let add_res = cousin_tea.add_commody(commody_info.clone());
            assert_eq!(add_res, 0);
            cousin_tea.add_commody(commody_info.clone());

            // get commody
            let commody = cousin_tea.get_commody(0);
            assert_eq!(commody, commody_info);

            // alter commody
            let alter_info = ink_prelude::string::String::from("To be altered");
            let alter_res = cousin_tea.alter_commody(0, alter_info);
            assert_eq!(alter_res, 0);


            // get count
            let commody_count = cousin_tea.commody_count();
            assert_eq!(commody_count, 2);
        }
    }
}
