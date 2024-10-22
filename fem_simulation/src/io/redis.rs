use redis::AsyncCommands;

pub async fn connect_redis() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;
    con.set("key", "value").await?;
    Ok(())
}