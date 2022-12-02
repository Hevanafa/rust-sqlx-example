use sqlx::{mysql::MySqlPoolOptions};
use std::io::{ Write, stdout, stdin };

// https://stackoverflow.com/questions/32015265/unable-to-import-a-module-struct-from-the-same-folder
mod manager;
use manager::Manager;

// data source name
fn dsn(username: &str, password: &str, hostname: &str, db_name: &str) -> String {
	// ?prefer_socket=false
	format!("mysql://{}:{}@{}:3306/{}", username, password, hostname, db_name).to_string()
}

fn flush() {
	Write::flush(&mut stdout()).unwrap();
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
	println!("Connecting to database...");

	// old, with dsn()
	// new way: .env with DATABASE_URL
	// let db_url = &dsn("root", "", "127.0.0.1", "journaling");
	// println!("{}", db_url);

	let db_url = std::env::var("DATABASE_URL").unwrap();

	let pool = MySqlPoolOptions::new()
		.max_connections(5)
		.connect(&db_url)
		.await?;

	
	let m = Manager::new(&pool);
	m.test_connection().await?;

	// list entries
	let entries = m.get_entries().await?;
	println!("Found {} entries.", entries.len());

	for entry in entries.iter() {
		println!("{}: {} | {}", entry.id, entry.datetime, entry.reason);
	}

	// new entry
	println!("New Entry");
	println!("Reason?");
	flush();

	let mut reason = String::new();
	stdin().read_line(&mut reason).unwrap();

	m.insert_entry(reason).await?;


	// let result: (String, i32, chrono::DateTime<chrono::Utc>) =
	// 	sqlx::query_as("SELECT item_name, count, receive_datetime FROM grocery WHERE id=?")
	// 		.bind(25)
	// 		.fetch_one(&pool)
	// 		.await?;

	// let (item_name, amount, receive_datetime) = result;

	// Important: this triggers the fetch, because query_as is lazy
	// transaction.commit().await?;

	// println!("{}: {}", item_name, amount);
	// println!("{}", receive_datetime.format("%d/%m/%Y %H:%M"));

	println!("Registration was successful.");

	Ok(())
}
