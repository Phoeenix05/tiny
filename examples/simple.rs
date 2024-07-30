//! a simple example of using tiny

use tiny::prelude::*;

#[main]
pub fn tiny() -> () {
    let _cache = Cache::builder()
        .with_capacity(128)
        .with_ttl(TTL::Custom(7200))
        .build();
}
