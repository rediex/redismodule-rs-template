mod command;

use redis_module::{redis_module};
use crate::command::command::{{crate_name}}_mul;

//////////////////////////////////////////////////////
redis_module! {
    name: "{{crate_name}}",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [],
    commands: [
        ["{{crate_name}}.mul", {{crate_name}}_mul, "", 0, 0, 0],
    ],
}
