use sqlx::{Connection, PgConnection};

pub mod data;

pub async fn connect_db() -> PgConnection {
    const USER: &str = "postgre";
    const PASSWORD: &str = "postgre";
    const HOST: &str = "localhost";
    const PORT: u32 = 6300;
    const DATABASE: &str = "db";

    let url = format!("postgresql://{USER}:{PASSWORD}@{HOST}:{PORT}/{DATABASE}");

    PgConnection::connect(&url).await.unwrap()
}
