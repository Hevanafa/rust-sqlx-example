02-12-2022

# About this Project
A simple demonstration of how to use Rust + SQLx + MySQL, complete with `struct` & `impl` as the OOP example.

# How to Start
1. Open phpMyAdmin or MySQL Workbench,
2. Create a new database `"journaling"`,
3. Import the SQL file `main.sql`,
4. Check if your `DATABASE_URL` inside the `.env` matches your actual MySQL settings

# How to Build & Run
Build:
`cargo build --release`

Run:
`.\target\release\journaling`
