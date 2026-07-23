use redis::{ErrorKind, RedisError};

pub enum RedisLookup<T> {
    Found(T),
    NotFound,
    Infra(RedisError),
}

pub fn is_infra_error(error: &RedisError) -> bool {
    error.kind() != ErrorKind::UnexpectedReturnType
}
