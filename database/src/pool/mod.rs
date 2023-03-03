use std::{env, str::FromStr};

use dotenv::dotenv;
use sqlx::{database, mysql::MySqlPoolOptions, MySqlPool};

#[derive(Debug)]
pub enum DatabaseType {
    MySql,
    Postgres,
}

impl FromStr for DatabaseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mysql" => Ok(DatabaseType::MySql),
            "postgres" => Ok(DatabaseType::Postgres),
            _ => Err(format!("Unknown database type: {}", s)),
        }
    }
}

#[derive(Debug)]
struct DatabaseConfig {
    _type: DatabaseType,
    username: String,
    password: String,
    host: String,
    port: i32,
    database_name: String,
}

#[allow(dead_code)]
impl DatabaseConfig {
    pub fn to_connect_string(&self) -> String {
        // postgresql://ev_owner:ev@127.0.0.1:5400/metrics
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn from_connection_string(connection_string: &str) -> Self {
        let mut split = connection_string.split("://");
        let database = split.next().unwrap();
        let mut split = split.next().unwrap().split("@");
        let mut split = split.next().unwrap().split(":");
        let username = split.next().unwrap().to_string();
        let password = split.next().unwrap().to_string();
        let mut split = split.next().unwrap().split("/");
        let mut split = split.next().unwrap().split(":");
        let host = split.next().unwrap().to_string();
        let port = split.next().unwrap().parse::<i32>().unwrap();
        let database_name = split.next().unwrap().to_string();
        Self {
            _type: DatabaseType::from_str(database).unwrap(),
            username,
            password,
            host,
            port,
            database_name,
        }
    }
}

pub async fn connect() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();
    let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or("10".to_string())
        .parse()
        .unwrap();
    let min_connections: u32 = env::var("DATABASE_MIN_CONNECTIONS")
        .unwrap_or("2".to_string())
        .parse()
        .unwrap();

    let pool = MySqlPoolOptions::new()
        .min_connections(min_connections)
        .max_connections(max_connections)
        .connect(&connection_string)
        .await?;
    sqlx::migrate!("../migrations").run(&pool).await.unwrap();
    Ok(pool)
}
