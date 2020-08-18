use sp_std::prelude::*;
use codec::{Codec, Encode, Decode};
use frame_support::{Parameter, decl_module, decl_storage, decl_event, decl_error, dispatch::DispatchResult, ensure};
use frame_system::{Self as system, ensure_signed};
use sp_runtime::traits::{CheckedAdd, CheckedSub, Member, AtLeast32BitUnsigned};

// the module trait
// contains type definitions
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type TokenBalance: CheckedAdd + CheckedSub + Parameter + Member + Codec + Default + Copy+ AtLeast32BitUnsigned;
}

// struct to store the token details
#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct Erc20Token<U> {
    name: Vec<u8>,
    ticker: Vec<u8>,
    total_supply: U,
}

// storage for this module
decl_storage! {
    trait Store for Module<T: Trait> as Erc20 {
        // details of the token corresponding to a token id
        Tokens get(token_details): map u32 => Erc20Token<T::TokenBalance>;
        // balances mapping for an account and token
        BalanceOf get(balance_of): map (u32, T::AccountId) => T::TokenBalance;
        // allowance for an account and token
        Allowance get(allowance): map (u32, T::AccountId, T::AccountId) => T::TokenBalance;
    }
}


// events
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, Balance = <T as self::Trait>::TokenBalance {
        // event for transfer of tokens
        // tokenid, from, to, value
        Transfer(u32, AccountId, AccountId, Balance),
        // event when an approval is made
        // 授权给另一个账户的token数目
        Approval(u32, AccountId, AccountId, Balance),
    }
);

decl_error!{
    pub enum Error for Module<T:Trait>{
        StorageOverflow,
    }
}


decl_module!{
    pub struct Module<T:Trait> for enum Call where origin: T::Origin{
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 0]
        fn init(origin, name:Vec<u8>, ticker: Vec<u8>, total_supply: T::TokenBalance) -> DispatchResult{
            let sender = ensure_signed(origin)?;
            ensure!(name.len() <= 64, "token name cannot exceed 64 bytes");
            ensure!(ticker.len() <= 32, "token ticker cannot exceed 32 bytes" );
            
            let token = Erc20Token{
                name,
                ticker,
                total_supply,
            };

            <Tokens<T>>::set<token);
            <Blanceof<T>>::insert(sender, total_supply);
            
            Ok(())
        }
        
        #[weight = 0]
        fn transfer(_origin, to:T::AccountId, value:T::TokenBalance)-> DispatchResult{
            let sender = ensure_signed(_origin)?;
            Self::_transfer(sender, to, value)
        }

        #[weight = 0]
        pub fn transfer_from(_origin, from:T::AccountId, to: T::AccountId, value:T::TokenBalance) -> DispatchResult{
            let allowance = Self::allowance((from.clone(), to.clone()));
            ensure!(allowance >= value, "Not enough allowance");

            let update_allowance = allowance.checked_sub(&value).ok_or(Error::<T>::StorageOverflow)?;
            <Allowance<T>>::insert((fromt.clone(), to.clone()), updated_allowance);;;;;;

            Self::deposit_event(RawEvent::Approval(sender.clone(), spender.clone(), value));

            Ok(())
        }

        #[weight = 0]
        fn approve(_origin, spender: T::AccountId, value: T::TokenBalance) -> Result {
            let sender = ensure_signed(_origin)?

            let allowance = Self::allowance((token_id, sender.clone(), spender.clone()));
            let updated_allowance = allowance.checked_add(&value).ok_or("overflow in calculating allowance")?;
            <Allowance<T>>::insert((token_id, sender.clone(), spender.clone()), updated_allowance);
  
            Self::deposit_event(RawEvent::Approval(token_id, sender.clone(), spender.clone(), value));
  
            Ok(())
        }
    }
}

// implementation of mudule
// utility and private functions
// if marked public, accessible by other modules
impl<T: Trait> Module<T> {
    // the ERC20 standard transfer function
    // internal
    fn _transfer(
        from: T::AccountId,
        to: T::AccountId,
        value: T::TokenBalance,
    ) -> DispatchResult {
        let sender_balance = Self::balance_of((token_id, from.clone()));
        ensure!(sender_balance >= value, "Not enough balance.");

        let updated_from_balance = sender_balance.checked_sub(&value).ok_or("overflow in calculating balance")?;
        let receiver_balance = Self::balance_of((token_id, to.clone()));
        let updated_to_balance = receiver_balance.checked_add(&value).ok_or("overflow in calculating balance")?;
        
        // reduce sender's balance
        <BalanceOf<T>>::insert((token_id, from.clone()), updated_from_balance);

        // increase receiver's balance
        <BalanceOf<T>>::insert((token_id, to.clone()), updated_to_balance);

        Self::deposit_event(RawEvent::Transfer(token_id, from, to, value));
        Ok(())
    }
}