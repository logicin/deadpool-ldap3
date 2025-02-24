use crate::{CreatePoolError, Manager, Pool, PoolBuilder};
use deadpool::managed::PoolConfig;
use std::convert::Infallible;
use deadpool::Runtime;

pub type LdapConfigError = Infallible;

#[derive(Clone)]
pub struct Config {
    /// ldap url
    pub url: String,
    /// bind_dn
    pub bind_dn: Option<String>,
    /// bind_password
    pub bind_password: Option<String>,
    /// [`Pool`] configuration.
    pub pool: Option<PoolConfig>,
}

impl Config {
    pub fn create_pool(&self, runtime: Runtime) -> Result<Pool, CreatePoolError> {
        self.builder(runtime)
            .map_err(CreatePoolError::Config)?
            .build()
            .map_err(CreatePoolError::Build)
    }

    pub fn builder(&self, runtime: Runtime) -> Result<PoolBuilder, LdapConfigError> {
        let manager = Manager::from_config(self, runtime);
        Ok(Pool::builder(manager)
            .config(self.pool.unwrap_or_default())
            .runtime(runtime))
    }
}
