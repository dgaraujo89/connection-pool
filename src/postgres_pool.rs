use postgres::{Client, NoTls};

use crate::*;

pub struct PostgresConnectionPool {
    config: String,
}

impl PostgresConnectionPool {
    pub fn new(connection_string: &str, config: Config) -> PoolManager<Client> {
        let pool = PostgresConnectionPool {
            config: String::from(connection_string),
        };

        PoolManager::new(config, Box::new(pool))
    }
}

impl Pool<Client> for PostgresConnectionPool {
    fn create(&mut self) -> Client {
        Client::connect(self.config.as_str(), NoTls).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::prelude::*;

    #[test]
    fn create_postgres_pool_manager() {
        let config = Config {
            minimum: 1,
            maximum: 5,
        };

        let pool_manager = PostgresConnectionPool::new("", config);

        assert_eq!(1, pool_manager.config.minimum);
        assert_eq!(5, pool_manager.config.maximum);
        assert_eq!(1, pool_manager.current_size);
    }
}
