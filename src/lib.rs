use std::collections::VecDeque;
use std::rc::Rc;

// #[cfg(feature = "postgres")]
pub mod postgres_pool;

pub trait Pool<T> {
    fn create(&mut self) -> T;
}

pub struct Config {
    pub minimum: u16,
    pub maximum: u16,
}

pub struct PoolManager<T> {
    config: Config,
    current_size: u16,
    pool: Box<dyn Pool<T>>,
    connections: VecDeque<Rc<Box<T>>>,
    borrowed_connections: VecDeque<Rc<Box<T>>>,
}

impl<T> PoolManager<T> {
    pub fn new(config: Config, pool: Box<impl Pool<T> + 'static>) -> Self {
        let mut pool_manager = PoolManager {
            config,
            current_size: 0,
            pool,
            connections: VecDeque::new(),
            borrowed_connections: VecDeque::new(),
        };

        pool_manager.init();

        pool_manager
    }

    fn init(&mut self) {
        for _ in 0..self.config.minimum {
            let client = Box::new(self.pool.create());
            self.connections.push_front(Rc::new(client));
            self.current_size += 1;
        }
    }

    fn borrow(&mut self) -> Rc<Box<T>> {
        let conn = self.connections.pop_back().unwrap();
        let conn_clone = Rc::clone(&conn);
        self.borrowed_connections.push_front(conn);
        conn_clone
    }
}

pub mod prelude {
    pub use crate::Config;
    pub use crate::Pool;
    pub use crate::PoolManager;

    // #[cfg(feature = "postgres")]
    pub use crate::postgres_pool::PostgresConnectionPool;
}
