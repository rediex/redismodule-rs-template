mod command;
mod test;
mod utils;

use redis_module::{redis_module};
use crate::command::command::hello_mul;

//////////////////////////////////////////////////////
redis_module! {
    name: "hello",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [],
    commands: [
        ["hello.mul", hello_mul, "", 0, 0, 0],
    ],
}