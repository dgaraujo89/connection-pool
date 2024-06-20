use postgres::{Client, NoTls};

use crate::*;

pub struct PostgresConnectionPool {
    config: String,
    client: Option<Client>,
}

impl PostgresConnectionPool {
    pub fn new(connection_string: &str, config: Config) -> PoolManager<Client> {
        let pool = PostgresConnectionPool {
            config: String::from(connection_string),
            client: None,
        };

        PoolManager::new(config, Box::new(pool))
    }

    fn create(&mut self) {
        self.client = Some(Client::connect(self.config.as_str(), NoTls).unwrap());
    }
}

impl Pool<Client> for PostgresConnectionPool {
    fn get_client(&mut self) -> &Client {
        match self.client {
            Some(ref client) => client,
            None => {
                self.create();
                self.client.as_ref().unwrap()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::prelude::*;

    #[test]
    fn my_test() {
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
