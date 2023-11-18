pub mod command{

    use redis_module::{Context, RedisError, RedisResult, RedisString};

    pub fn {{crate_name}}_mul(_: &Context, args: Vec<RedisString>) -> RedisResult {
        if args.len() < 2 {
            return Err(RedisError::WrongArity);
        }

        let nums = args
            .into_iter()
            .skip(1)
            .map(|s| s.parse_integer())
            .collect::<Result<Vec<i64>, RedisError>>()?;

        let product = nums.iter().product();

        let mut response = nums;
        response.push(product);

        Ok(response.into())
    }
}