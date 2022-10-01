#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {

    use ink_storage::Mapping;
     use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub  struct ERC20{
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        approval: Mapping<AccountId,AccountId>,
    }

    #[ink(event)]
    pub  struct Transfer{
        #[ink(topic)]
        from: AccountId,
        to: AccountId,
        value :Balance,
    }

    #[ink(event)]
    pub  struct Approval{
        owner: AccountId,
        spender:AccountId,
        value: Balance,
    }

    pub type Result<T> = core::result::Result<T, Error>;
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
    /// Return if the balance cannot fulfill a request.
    InsufficientBalance,
    }

    impl ERC20 {

        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            // let mut balances = Mapping::default();

            // let sender = Self::env().caller();
            // balances.insert(sender, &total_supply);

            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.balances.insert(&caller, &total_supply);

                Self::env().emit_event(Transfer{
                    from: AccountId::default(),
                    to: caller,
                    value: total_supply,
                });

                ink_env::debug_println!(
                    "打印log constructor total_supply{}: initialize_contract ",
                    total_supply
                );
            })

            //   Self::env().emit_event(Transfer{
            //     from: AccountId::default(),
            //     to: sender,
            //     value: total_supply,
            //     });
            // Self {
            //     total_supply,
            //     balances,
            //    approval: Default::default()
            //      }
        }



        #[ink(message)]
        pub fn get(&self) -> Balance {
            self.total_supply
        }
        #[ink(message)]
        pub fn balance_of(&self,who:AccountId)-> Balance {
            self.balances.get(who).unwrap_or_default()
        }




        #[ink(message)]
        pub fn total_supply(&self)->Balance{
            self.total_supply
        }

        #[ink(message)]
        pub fn transfer(&mut self,to:AccountId, value:Balance)-> Result<()>{

            let from = self.env().caller();
            let from_balance = self.balance_of(from);

            if from_balance < value {
              return Err(Error::InsufficientBalance)
            }


             self.balances.insert(&from,&((from_balance-value)));

            let to_balance = self.balance_of(to);
             self.balances.insert(&to,&((to_balance+value)));

            self.env().emit_event(
                Transfer{
                    from,
                    to,
                    value,
                }
            );
            Ok(())
        }

    }

}
