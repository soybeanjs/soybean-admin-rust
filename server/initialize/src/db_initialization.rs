#![allow(dead_code)]
use std::{sync::Arc, time::Duration};

use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use server_config::{DatabaseConfig, DatabasesConfig};
use server_global::global::{get_config, GLOBAL_DB_POOL, GLOBAL_PRIMARY_DB};

pub async fn init_primary_connection() -> Result<(), String> {
    let db_config = get_config::<DatabaseConfig>().await.unwrap();
    let opt = build_connect_options(&db_config);
    let db = Database::connect(opt)
        .await
        .map_err(|e| format!("[soybean-admin-rust] >>>>>> [server-initialize] Failed to connect to primary database: {}", e))?;
    *GLOBAL_PRIMARY_DB.write().await = Some(Arc::new(db));
    info!(
        "[soybean-admin-rust] >>>>>> [server-initialize] Primary database connection initialized"
    );
    Ok(())
}

pub async fn init_db_pool_connections(
    databases_config: Option<Vec<DatabasesConfig>>,
) -> Result<(), String> {
    if let Some(dbs) = databases_config {
        for db_config in dbs {
            init_db_connection(&db_config.name, &db_config.database).await?;
        }
    }
    Ok(())
}

async fn init_db_connection(name: &str, db_config: &DatabaseConfig) -> Result<(), String> {
    let opt = build_connect_options(db_config);
    let db = Database::connect(opt)
        .await
        .map_err(|e| format!("[soybean-admin-rust] >>>>>> [server-initialize] Failed to connect to database '{}': {}", name, e))?;
    GLOBAL_DB_POOL.write().await.insert(name.to_string(), Arc::new(db));
    info!("[soybean-admin-rust] >>>>>> [server-initialize] Database '{}' initialized", name);
    Ok(())
}

fn build_connect_options(db_config: &DatabaseConfig) -> ConnectOptions {
    let mut opt = ConnectOptions::new(db_config.url.clone());
    opt.max_connections(db_config.max_connections)
        .min_connections(db_config.min_connections)
        .connect_timeout(Duration::from_secs(db_config.connect_timeout))
        .idle_timeout(Duration::from_secs(db_config.idle_timeout))
        .sqlx_logging(false);
    opt
}

pub async fn get_primary_db_connection() -> Option<Arc<DatabaseConnection>> {
    GLOBAL_PRIMARY_DB.read().await.clone()
}

pub async fn get_db_pool_connection(name: &str) -> Option<Arc<DatabaseConnection>> {
    GLOBAL_DB_POOL.read().await.get(name).cloned()
}

pub async fn add_or_update_db_pool_connection(
    name: &str,
    db_config: &DatabaseConfig,
) -> Result<(), String> {
    init_db_connection(name, db_config).await
}

pub async fn remove_db_pool_connection(name: &str) -> Result<(), String> {
    let mut db_pool = GLOBAL_DB_POOL.write().await;
    db_pool.remove(name).ok_or_else(|| "Connection not found".to_string())?;
    info!("[soybean-admin-rust] >>>>>> [server-initialize] Database connection '{}' removed", name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;
    use server_config::Config;
    use server_global::global::get_config;
    use simple_logger::SimpleLogger;
    use tokio::sync::Mutex;

    use super::*;
    use crate::initialize_config;

    fn setup_logger() {
        let _ = SimpleLogger::new().with_level(LevelFilter::Info).init();
    }

    static INITIALIZED: Mutex<Option<Arc<()>>> = Mutex::const_new(None);

    async fn init() {
        let mut initialized = INITIALIZED.lock().await;
        if initialized.is_none() {
            initialize_config("../resources/application.yaml").await;
            *initialized = Some(Arc::new(()));
        }
    }

    #[tokio::test]
    async fn test_primary_connection_persistence() {
        setup_logger();
        init().await;

        let result = init_primary_connection().await;
        assert!(result.is_ok(), "Failed to initialize all connections: {:?}", result.err());

        let connection = get_primary_db_connection().await;
        assert!(connection.is_some(), "Master database connection does not exist");
    }

    #[tokio::test]
    async fn test_db_pool_connection() {
        setup_logger();
        init().await;

        let config = get_config::<Config>().await.unwrap().as_ref().clone();
        let result = init_db_pool_connections(config.databases).await;
        assert!(result.is_ok(), "Failed to initialize db_pool connections: {:?}", result.err());

        let db_config = DatabaseConfig {
            url: "postgresql://soybean:soybean@123.@localhost:35432/postgres".to_string(),
            max_connections: 50,
            min_connections: 5,
            connect_timeout: 15,
            idle_timeout: 600,
        };

        let add_result = add_or_update_db_pool_connection("test_connection", &db_config).await;
        assert!(add_result.is_ok(), "Failed to add database connection");

        let connection = get_db_pool_connection("test_connection").await;
        assert!(connection.is_some(), "Database connection 'test_connection' does not exist");
        println!("Added and retrieved database connection successfully.");

        println!("Current pool size after addition: {}", GLOBAL_DB_POOL.read().await.len());

        let remove_result = remove_db_pool_connection("test_connection").await;
        assert!(remove_result.is_ok(), "Failed to remove database connection");

        let connection_after_removal = get_db_pool_connection("test_connection").await;
        assert!(
            connection_after_removal.is_none(),
            "Database connection 'test_connection' still exists after removal"
        );

        println!("Current pool size after removal: {}", GLOBAL_DB_POOL.read().await.len());
    }
}