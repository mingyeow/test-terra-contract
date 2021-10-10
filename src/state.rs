use cw_storage_plus::Item;
use cosmwasm_std::{Uint64, Addr};


// pub const fn new(storage_key: &'a str) -> Self
// to learn: you cannot save primary
// to ask: why can't you save u64?
// the owen new for ping_count does not even matter
pub const PING_COUNT: Item<Uint64> = Item::new("ping_count");
pub const OWNER: Item<Addr> = Item::new("owner_address");
