use once_cell::sync::{OnceCell};

pub static DB:OnceCell<tokio_postgres::Client> = OnceCell::new();

pub async fn query_erotic() -> Option<String> {
    let client = DB.get().unwrap();

    let res = client.
    query_one("SELECT * FROM erotic 
        WHERE ID >= ((SELECT MAX(ID) FROM erotic)-(SELECT MIN(ID) FROM erotic)) * RAND() + 
        (SELECT MIN(ID) FROM erotic)  LIMIT 1", &[]).await;

    match res {
        Ok(data) => Some(data.get(0)),
        _ => None,
    }
}