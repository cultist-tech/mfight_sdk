// Upgradable

#[macro_export]
macro_rules! impl_non_fungible_token_upgradable {
    ($contract:ident, $tokens:ident, $assert_owner:ident $(, $assert_upgrade:ident)?) => {
        use $crate::nft::NonFungibleTokenUpgradable;

        #[near_bindgen]
        impl NonFungibleTokenUpgradable for $contract {
            #[payable]
            fn nft_upgrade(&mut self, token_id: mfight_sdk::nft::TokenId) { 
                $(self.$assert_upgrade();)?
                self.$tokens.nft_upgrade(token_id)
            }
            
            fn nft_set_upgrade_price(&mut self, rarity: mfight_sdk::nft::TokenRarity, price: U128) {
                self.$assert_owner();
                self.$tokens.nft_set_upgrade_price(rarity, price)
            }
            
            fn nft_upgrade_price(&self, token_id: mfight_sdk::nft::TokenId) -> U128 {
                self.$tokens.nft_upgrade_price(token_id)            
            }
        }
    };
}