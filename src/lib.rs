use redis;

const RELEASE_SCRIPT: &str = r#"if redis.call("get", KEYS[1]) == ARGV[1] then return redis.call("del", KEYS[1]) else return 0 end"#;

pub struct RedisLock {
    client: redis::Client,
    key: String,
    value: String,
    ttl: usize,
    acquired: bool
}

impl RedisLock {
    pub fn new(address: &str, key: &str, ttl: usize) -> Result<RedisLock, redis::RedisError> {
        let value = RedisLock::gen_rand();
        redis::Client::open(address).map(|client| RedisLock {
            client: client,
            key: key.to_string(),
            value: value,
            ttl: ttl,
            acquired: false,
        })
    }

    pub fn acquire(&mut self) -> Result<(), redis::RedisError> {
        let cmd = redis::Cmd::pset_ex(&self.key, &self.value, self.ttl);
        match self.client.get_connection() {
            Ok(mut conn) => cmd.query(&mut conn).map(
                |_x: ()| {
                    self.acquired = true;
                    ()
                }
            ),
            Err(err) => Err(err),
        }
    }

    fn gen_rand() -> String {
        "foo".to_string()
    }

    pub fn release(&mut self) -> Result<(), redis::RedisError> {
        match self.acquired {
            true => self.do_release(),
            false => Ok(())
        }
    }

    fn do_release(&mut self) -> Result<(), redis::RedisError> {
        let script = redis::Script::new(RELEASE_SCRIPT);
        script.arg(&self.key).arg(&self.value);
        match self.client.get_connection() {
            Ok(mut conn) => script.invoke(&mut conn).map(
                |_x: ()| {
                    self.acquired = false;
                    ()
                }
            ),
            Err(err) => Err(err),
        }
    }
}

impl Drop for RedisLock {
    fn drop(&mut self) {
        let _ = self.release();
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
