use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::*;
use crate::{enc::{self, hash_password}, models::Password};

pub struct Db {
    conn: SqlitePool,
}

impl Db {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let conn = SqlitePool::connect("sqlite://database.db").await?;
        
        // Create the `users` and `passwords` tables if they don't exist.
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                masterpassword TEXT NOT NULL
            )"
        )
        .execute(&conn)
        .await?;
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS passwords (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                website TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                user_id INTEGER NOT NULL,
                FOREIGN KEY(user_id) REFERENCES users(id)
            )"
        )
        .execute(&conn)
        .await?;

        Ok(Db { conn })
    }

    pub async fn insert_user(&self, username: String, password: String) -> Result<i64, sqlx::Error> {
        // Hash the password
        let hashed_password = hash_password(password).unwrap();
        
        let id = sqlx::query(
            "INSERT INTO users(username, masterpassword) VALUES(?1, ?2)"
        )
        .bind(username)
        .bind(hashed_password)
        .execute(&self.conn)
        .await?
        .last_insert_rowid();
        
        Ok(id)
    }

    pub async fn authenticate_user(&self, username: String, password: String) -> Result<Option<i64>, sqlx::Error> {
        match sqlx::query(
            "SELECT id, masterpassword FROM users WHERE username = ?1"
        )
        .bind(username)
        .fetch_one(&self.conn)
        .await {
            Ok(row) => {
                let stored_password: String = row.get("masterpassword");
                let user_id: i64 = row.get("id");

                // Verify the password using bcrypt
                if verify(&password, &stored_password).unwrap_or(false) {
                    Ok(Some(user_id))
                } else {
                    Err(sqlx::Error::RowNotFound) // Password mismatch
                }
            }
            Err(sqlx::Error::RowNotFound) => {
                // User not found
                eprintln!("Error: User not found.");
                Ok(None)
            }
            Err(e) => {
                // Other SQL errors
                eprintln!("SQL Error: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn insert_password(&self, username: String, masterpassword: String, website: String, webusername: String, webpassword: String) -> Result<i64, sqlx::Error> {
        if let Some(user_id) = self.authenticate_user(username, masterpassword).await? {
            let rows = sqlx::query(
                "INSERT INTO passwords(website, username, password, user_id) values(?1, ?2, ?3, ?4)"
            )
            .bind(website)
            .bind(webusername)
            .bind(webpassword)
            .bind(user_id)
            .execute(&self.conn)
            .await?
            .last_insert_rowid();

            Ok(rows)
        } else {
            Err(sqlx::Error::RowNotFound) // Authentication failed
        }
    }

    pub async fn update_password(&self, username: String, masterpassword: String, website: String, webusername: String, webpassword: String) -> Result<u64, sqlx::Error> {
        if let Some(user_id) = self.authenticate_user(username, masterpassword).await? {
            let rows = sqlx::query(
                "UPDATE passwords SET password = ?1, username = ?2 WHERE user_id = ?3 AND website = ?4"
            )
            .bind(webpassword)
            .bind(webusername)
            .bind(user_id)
            .bind(website)
            .execute(&self.conn)
            .await?
            .rows_affected();

            Ok(rows)
        } else {
            Err(sqlx::Error::RowNotFound) // Authentication failed
        }
    }

    pub async fn display_all(&self, username: String, masterpassword: String) -> Result<Vec<Password>, sqlx::Error> {
        if let Some(user_id) = self.authenticate_user(username, masterpassword).await? {
            let passwords = sqlx::query_as::<_, Password>(
                "SELECT website, username, password FROM passwords WHERE user_id = ?1"
            )
            .bind(user_id)
            .fetch_all(&self.conn)
            .await?;

            Ok(passwords)
        } else {
            Err(sqlx::Error::RowNotFound) // Authentication failed
        }
    }

    pub async fn delete_password(&self, username: String, masterpassword: String, website: &String) -> Result<u64, sqlx::Error> {
        if let Some(user_id) = self.authenticate_user(username, masterpassword).await? {
            let rows = sqlx::query(
                "DELETE FROM passwords WHERE user_id = ?1 AND website = ?2"
            )
            .bind(user_id)
            .bind(website)
            .execute(&self.conn)
            .await?
            .rows_affected();

            Ok(rows)
        } else {
            Err(sqlx::Error::RowNotFound) // Authentication failed
        }
    }
}
