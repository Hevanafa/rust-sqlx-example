use sqlx::{MySql, Pool};
use chrono::{DateTime, Utc, NaiveDateTime};

pub struct Entry {
	pub id: i32,
	pub datetime: NaiveDateTime,
	pub reason: String
}

pub struct Manager<'m> {
    pool: &'m Pool<MySql>,
}

impl<'m> Manager<'m> {
    pub fn new(pool: &Pool<MySql>) -> Manager {
        Manager { pool }
    }

    pub async fn test_connection(&self) -> Result<(), sqlx::Error> {
        let transaction = self.pool.begin().await?;
        println!("Connection was successful.");

        Ok(())
    }

    pub async fn insert_entry(&self, reason: String) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        let entry: Option<(i32,)> =
            sqlx::query_as(r"INSERT INTO main (datetime, reason) VALUES (?, ?)")
                .bind(Utc::now())
                .bind(&reason)
                .fetch_optional(&mut transaction)
                .await?;

        transaction.commit().await?;

        Ok(())
    }

	pub async fn get_entries(&self) -> Result<Vec<Entry>, sqlx::Error> {
		let mut transaction = self.pool.begin().await?;

		let entries: Vec<Entry> = sqlx::query_as!(Entry, r"SELECT * FROM main LIMIT 10")
			.fetch_all(&mut transaction)
			.await?;

		transaction.commit().await?;

		Ok(entries)
	}
}
