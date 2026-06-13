use std::env;

pub fn get_redis_client() -> redis::Client {
    let redis_host_name =
        env::var("REDIS_HOST_NAME").expect("Redis hostname should be present to run the app");
    let redis_password =
        env::var("REDIS_PASSWORD").expect("Redis password shoul be present to run the app");
    let redis_user = env::var("REDIS_USER_NAME").unwrap_or_else(|_| "".into());
    let uri_scheme = match env::var("REDIS_TLS_AVAILABLE") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };
    let url = format!(
        "{}://{}:{}@{}",
        uri_scheme, redis_user, redis_password, redis_host_name
    );
    redis::Client::open(url).expect("Invalid redis connection url")
}
