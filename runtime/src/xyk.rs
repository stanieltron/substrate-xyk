/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, StorageValue, StorageMap, dispatch::DispatchResult};
use system::ensure_signed;


/// The module's configuration trait.
pub trait Trait: balances::Trait {
        // TODO: Add other types and constants required configure this module.

        // The overarching event type.
        //type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
        trait Store for Module<T: Trait> as XykStorage {
        // Declare storage and getter functions here
        owned_e: map T::AccountId => u64;
        owned_x: map T::AccountId => u64;
        owned_y: map T::AccountId => u64;
        owned_pool_ex: map T::AccountId => u64; //in e
        owned_pool_ey: map T::AccountId => u64; //in e


        pool_exe: u64;
        pool_exx: u64;
        pool_eye: u64;
        pool_eyy: u64;

        precision: u64;
        precision_percentage: u64;

        FeeSignificand: u64;
        FeeDecimals: u64;
    }
}

// The module's dispatchable functions.
decl_module! {
        /// The module declaration.
        pub struct Module<T: Trait> for enum Call where origin: T::Origin {
                // Initializing events
                // this is needed only if you are using events in your module

        fn mint_e_for_user(origin, value: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let sum = <owned_e<T>>::get(&sender) + value;

            <owned_e<T>>::insert(sender, sum);

            Ok(())
        }

        fn mint_x_for_user(origin, value: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let sum = <owned_x<T>>::get(&sender) + value;

            <owned_x<T>>::insert(sender, sum);

            Ok(())
        }

        fn mint_y_for_user(origin, value: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let sum = <owned_y<T>>::get(&sender) + value;

            <owned_y<T>>::insert(sender, sum);

            Ok(())
        }

        fn set_default_values(origin) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            pool_exe::put(10000000 as u64);
            pool_exx::put(500000000 as u64);
            pool_eye::put(10000000 as u64);
            pool_eyy::put(500000000 as u64);

            precision::put(100000 as u64); // add 6 zeros
            precision_percentage::put(1000 as u64); // from 3 -> 0.3%

            FeeSignificand::put(3 as u64);
            FeeDecimals::put(2 as u64);

            Ok(())
        }

        fn sell_e_ex(origin, e_amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            if <owned_e<T>>::get(&sender) < e_amount {
               //nejaky mesage a exit ?
            }
            else {
                let e = pool_exe::get();
                let x = pool_exx::get();
                let g = precision_percentage::get() - FeeSignificand::get(); // 997 (will be used as 0.997%)
                let de = e_amount;
                
                let e1 = e + de;
                let x1 = x * e / (e + de * g / precision_percentage::get());
                let dx = x - x1;

       

                let balanceE = <owned_e<T>>::get(&sender) - de;
                <owned_e<T>>::insert(&sender, balanceE);
                let balanceX = <owned_x<T>>::get(&sender) + dx;
                <owned_x<T>>::insert(&sender, balanceX);
                pool_exe::put(e1 as u64);
                pool_exx::put(x1 as u64);

            }

            Ok(())
        }

        fn sell_e_ey(origin, e_amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            if <owned_e<T>>::get(&sender) < e_amount {
               //nejaky mesage a exit ?
            }
            else {
                let e = pool_eye::get();
                let y = pool_eyy::get();
                let g = precision_percentage::get() - FeeSignificand::get(); // 997 (will be used as 0.997%)
                let de = e_amount;
                
                let e1 = e + de;
                let y1 = y * e / (e + de * g / precision_percentage::get());
                let dy = y - y1;

       

                let balanceE = <owned_e<T>>::get(&sender) - de;
                <owned_e<T>>::insert(&sender, balanceE);
                let balanceY = <owned_y<T>>::get(&sender) + dy;
                <owned_y<T>>::insert(&sender, balanceY);
                pool_eye::put(e1 as u64);
                pool_eyy::put(y1 as u64);

            }

            Ok(())
        }


        fn sell_x_ex(origin, x_amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            if <owned_x<T>>::get(&sender) < x_amount {
               //nejaky mesage a exit ?
            }
            else {
                let x = pool_exx::get();
                let e = pool_exe::get();
                let g = precision_percentage::get() - FeeSignificand::get(); // 997 (will be used as 0.997%)
                        
                let dx = x_amount;
                let x1 = x + dx;
                let e1 = e * x / (x + dx * g / precision_percentage::get());
                let de = e - e1;

                let balanceE = <owned_e<T>>::get(&sender) + de;
                <owned_e<T>>::insert(&sender, balanceE);
                let balanceX = <owned_x<T>>::get(&sender) - dx;
                <owned_x<T>>::insert(&sender, balanceX);
                pool_exe::put(e1 as u64);
                pool_exx::put(x1 as u64);

            }

            Ok(())
        }


        fn sell_y_ey(origin, y_amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            if <owned_y<T>>::get(&sender) < y_amount {
               //nejaky mesage a exit ?
            }
            else {
                let y = pool_eyy::get();
                let e = pool_eye::get();
                let g = precision_percentage::get() - FeeSignificand::get(); // 997 (will be used as 0.997%)
                        
                let dy = y_amount;
                let y1 = y + dy;
                let e1 = e * y / (y + dy * g) / precision_percentage::get();
                let de = e - e1;

                let balanceE = <owned_e<T>>::get(&sender) + de;
                <owned_e<T>>::insert(&sender, balanceE);
                let balanceY = <owned_y<T>>::get(&sender) - dy;
                <owned_y<T>>::insert(&sender, balanceY);
                pool_eye::put(e1 as u64);
                pool_eyy::put(y1 as u64);

            }

            Ok(())
        }

        fn mint_ex(origin, e_amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let e: u64 = pool_exe::get();
            let x: u64 = pool_exx::get();


            let x_amount: u64 = x * e_amount / e;
            

            if <owned_e<T>>::get(&sender) < e_amount {
                //nejaky mesage a exit ?
            }
            else if <owned_x<T>>::get(&sender) < x_amount {
                //nejaky mesage a exit ?
            }
            else {
                
                
                let balanceE = <owned_e<T>>::get(&sender) - e_amount;
                <owned_e<T>>::insert(&sender, balanceE);
                let balanceX = <owned_x<T>>::get(&sender) - x_amount;
                <owned_x<T>>::insert(&sender, balanceX);

                pool_exe::put(e + e_amount);
                pool_exx::put(x + x_amount);

                let balanceEXToken = <owned_pool_ex<T>>::get(&sender) + e_amount * 1000000 / e;
                <owned_pool_ex<T>>::insert(&sender, balanceEXToken);
            }

            Ok(())
        }

        fn mint_ey(origin, e_amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let e: u64 = pool_eye::get();
            let y: u64 = pool_eyy::get();


            let y_amount: u64 = y * e_amount / e;
            

            if <owned_e<T>>::get(&sender) < e_amount {
                //nejaky mesage a exit ?
            }
            else if <owned_x<T>>::get(&sender) < y_amount {
                //nejaky mesage a exit ?
            }
            else {
                
                
                let balanceE = <owned_e<T>>::get(&sender) - e_amount;
                <owned_e<T>>::insert(&sender, balanceE);
                let balanceY = <owned_y<T>>::get(&sender) - y_amount;
                <owned_x<T>>::insert(&sender, balanceY);

                pool_exe::put(e + e_amount);
                pool_exx::put(y + y_amount);

                let balanceEYToken = <owned_pool_ey<T>>::get(&sender) + e_amount * 1000000 / e;
                <owned_pool_ey<T>>::insert(&sender, balanceEYToken);
            }

            Ok(())
        }

        fn sell_x_for_y(origin, x_amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            if <owned_x<T>>::get(&sender) < x_amount {
               //nejaky mesage a exit ?
            }
            else {
                let x = pool_exx::get();
                let mut e = pool_exe::get();
                let g = precision_percentage::get() - FeeSignificand::get(); // 997 (will be used as 0.997%)
                        
                let dx = x_amount;
                let x1 = x + dx;
                let e1 = e * x / (x + dx * g / precision_percentage::get());
                let de = e - e1;

                let mut balanceE = <owned_e<T>>::get(&sender) + de;
                <owned_e<T>>::insert(&sender, balanceE);
                let balanceX = <owned_x<T>>::get(&sender) - dx;
                <owned_x<T>>::insert(&sender, balanceX);
                pool_exe::put(e1 as u64);
                pool_exx::put(x1 as u64);


                e = pool_eye::get();
                let y = pool_eyy::get();               
                let e1 = e + de;
                let y1 = y * e / (e + de * g / precision_percentage::get());
                let dy = y - y1;

                 balanceE = <owned_e<T>>::get(&sender) - de;
                <owned_e<T>>::insert(&sender, balanceE);
                let balanceY = <owned_y<T>>::get(&sender) + dy;
                <owned_y<T>>::insert(&sender, balanceY);
                pool_eye::put(e1 as u64);
                pool_eyy::put(y1 as u64);

            }

            Ok(())
        }

        fn sell_y_for_x(origin, y_amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            if <owned_y<T>>::get(&sender) < y_amount {
               //nejaky mesage a exit ?
            }
            else {
                let y = pool_eyy::get();
                let mut e = pool_eye::get();
                let g = precision_percentage::get() - FeeSignificand::get(); // 997 (will be used as 0.997%)
                        
                let dy = y_amount;
                let y1 = y + dy;
                let e1 = e * y / (y + dy * g) / precision_percentage::get();
                let de = e - e1;

                let mut balanceE = <owned_e<T>>::get(&sender) + de;
                <owned_e<T>>::insert(&sender, balanceE);
                let balanceY = <owned_y<T>>::get(&sender) - dy;
                <owned_y<T>>::insert(&sender, balanceY);
                pool_eye::put(e1 as u64);
                pool_eyy::put(y1 as u64);


                e = pool_exe::get();
                let x = pool_exx::get();                
                let e1 = e + de;
                let x1 = x * e / (e + de * g / precision_percentage::get());
                let dx = x - x1;

                balanceE = <owned_e<T>>::get(&sender) - de;
                <owned_e<T>>::insert(&sender, balanceE);
                let balanceX = <owned_x<T>>::get(&sender) + dx;
                <owned_x<T>>::insert(&sender, balanceX);
                pool_exe::put(e1 as u64);
                pool_exx::put(x1 as u64);
                

            }

            Ok(())
        }




    }
}