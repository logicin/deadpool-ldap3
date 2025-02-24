pub mod config;

use deadpool::managed::{Metrics, RecycleError, RecycleResult};
use deadpool::{managed, Runtime};
use ldap3::{Ldap, LdapConnAsync, LdapError};

use crate::config::{Config, LdapConfigError};

deadpool::managed_reexports!(
    "ldap",
    Manager,
    managed::Object<Manager>,
    LdapError,
    LdapConfigError
);

pub struct Manager {
    /// LDAP addr
    pub url: String,
    /// Manager Account
    pub bind_dn: Option<String>,
    /// Manager Password
    pub bind_password: Option<String>,
    /// Runtime, Use deadpool runtime
    pub runtime: Runtime,
}

impl Manager {
    #[must_use]
    pub fn from_config(config: &Config, runtime: Runtime) -> Self {
        Self {
            url: config.url.clone(),
            bind_dn: config.bind_dn.clone(),
            bind_password: config.bind_password.clone(),
            runtime,
        }
    }
}

pub type LdapConnection = Ldap;

impl managed::Manager for Manager {
    type Type = LdapConnection;
    type Error = LdapError;

    /// create conn
    async fn create(&self) -> Result<LdapConnection, LdapError> {
        let (conn, mut ldap) = LdapConnAsync::new(&self.url).await?;
        match self.runtime {
            #[cfg(feature = "rt_tokio_1")]
            Runtime::Tokio1 => {
                tokio::spawn(async move { conn.drive().await });
            }
            #[cfg(feature = "rt_async-std_1")]
            Runtime::AsyncStd1 => {
                async_std::task::spawn(async move { conn.drive().await });
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        };
        if let (Some(bind_dn), Some(bind_password)) = (&self.bind_dn, &self.bind_password) {
            ldap.simple_bind(bind_dn, bind_password).await?.success()?;
        }
        Ok(ldap)
    }

    /// recycle conn
    async fn recycle(&self, ldap: &mut LdapConnection, _: &Metrics) -> RecycleResult<LdapError> {
        // re
        if let (Some(bind_dn), Some(bind_password)) = (&self.bind_dn, &self.bind_password) {
            ldap.simple_bind(bind_dn, bind_password).await?.success()?;
        }
        // check if the LDAP connection is closed
        if ldap.is_closed() {
            Err(RecycleError::message("ldap connection is closed"))
        } else {
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_ldap() {
    let cfg = Config {
        url: "ldap://127.0.0.1:389".to_string(),
        bind_dn: Some("cn=admin,dc=demo,dc=com".to_string()),
        bind_password: Some("123456".to_string()),
        pool: None,
    };
    let pool = cfg.create_pool(Runtime::Tokio1).unwrap();
    let mut a = pool.get().await.unwrap();
    a.simple_bind("admin", "123456").await.unwrap();
}
