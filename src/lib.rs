use std::collections::VecDeque;
use std::rc::Rc;

// #[cfg(feature = "postgres")]
pub mod postgres_pool;

pub trait Pool<T> {
    fn get_client(&mut self) -> &T;
}

pub struct Config {
    pub minimum: u16,
    pub maximum: u16,
}

pub struct PoolManager<T> {
    config: Config,
    current_size: u16,
    _pool: VecDeque<Rc<Box<dyn Pool<T>>>>,
}

impl<T> PoolManager<T> {
    pub fn new(config: Config, _pool: Box<impl Pool<T>>) -> Self {
        let mut pool_manager = PoolManager {
            config,
            current_size: 0,
            _pool: VecDeque::new(),
        };

        pool_manager.init();

        pool_manager
    }

    fn init(&mut self) {
        for _ in 0..self.config.minimum {
            self.current_size += 1;
        }
    }
}

pub mod prelude {
    pub use crate::Config;
    pub use crate::Pool;
    pub use crate::PoolManager;

    // #[cfg(feature = "postgres")]
    pub use crate::postgres_pool::PostgresConnectionPool;
}
