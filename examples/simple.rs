use rsredlock;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut lock = rsredlock::RedisLock::new("redis://127.0.0.1:7777", "mutex", 300)?;
    println!("Acquiring lock...{:?}", lock.acquire());
    Ok(())
}
