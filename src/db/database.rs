use bcrypt::{hash, verify};
use sqlx::SqlitePool;

use crate::{
    http::error::UserCreateError,
    models::{
        AuthPayload, CreateEntryPayload, CreateUserPayload, HighscoreList, MeterData, MeterDataList, HighscoreEntry,
    },
};

#[derive(Clone)]
pub struct Database {
    conn: SqlitePool,
}

impl Database {
    pub async fn auth_user(self, user: &AuthPayload) -> bool {
        let hash: (String,) = sqlx::query_as("SELECT hash FROM users WHERE username = ?")
            .bind(&user.username)
            .fetch_one(&self.conn)
            .await
            .unwrap_or(("".to_owned(),));
        if hash.0.is_empty() {
            return false;
        }

        verify(&user.pass, &hash.0).unwrap()
    }

    pub async fn create_user(self, user: CreateUserPayload) -> Option<UserCreateError> {
        let same_username_users = sqlx::query("select id from users where username=?")
            .bind(&user.username)
            .fetch_all(&self.conn)
            .await
            .unwrap();

        if !same_username_users.is_empty() {
            // println!("usn taken");
            return Some(UserCreateError::UsernameTaken);
        }

        if user.pass.len() < 8 {
            return Some(UserCreateError::PasswordTooShort);
        }

        if user.username.len() < 4 {
            return Some(UserCreateError::UsernameTooShort);
        }

        if !user
            .username
            .chars()
            .into_iter()
            .all(|c| c.is_ascii_alphanumeric())
        {
            return Some(UserCreateError::UsernameBad);
        }

        if !user.pass.chars().into_iter().any(|c| c.is_ascii_digit()) {
            return Some(UserCreateError::PasswortNotGoodEnough);
        }

        let hash = hash(user.pass, bcrypt::DEFAULT_COST).unwrap();
        sqlx::query("INSERT INTO users (username, email, hash) VALUES (?, ?, ?)")
            .bind(user.username)
            .bind(user.email)
            .bind(hash)
            .execute(&self.conn)
            .await
            .unwrap();

        None
    }

    pub async fn create_entry(self, username: &String, payload: &CreateEntryPayload) {
        sqlx::query("insert into data (username, meters) values (?, ?)")
            .bind(username)
            .bind(&payload.meters)
            .execute(&self.conn)
            .await
            .unwrap();
    }

    pub async fn get_meters(self, username: &str) -> MeterDataList {
        let meters_vec = sqlx::query_as::<_, MeterData>("SELECT * FROM data WHERE username = ? order by time desc")
            .bind(username)
            .fetch_all(&self.conn)
            .await
            .unwrap();
        // if meters_vec.is_empty(){ return 0;}

        let mut sum = 0.0;
        for d in &meters_vec {
            sum += d.meters;
        }

        //let sum = meters_vec.into_iter().sum();
        return MeterDataList {
            sum,
            entries: meters_vec,
        };
    }

    pub async fn highscore(self) -> HighscoreList {
        let mut meter_vec = sqlx::query_as::<_, HighscoreEntry>(
            "select username, sum(meters) as meters from data group by username order by meters desc",
        )
        .fetch_all(&self.conn)
        .await
        .unwrap();

        // for i in 0..50 {
        //     if i >= meter_vec.len() {
        //         break;
        //     }
        //     highscore_vec.push(meter_vec[i].meters);
        // }
        if meter_vec.len() > 50 {
            meter_vec.resize(
                50,
                HighscoreEntry {
                    username: "".to_string(),
                    meters: 0.0,
                },
            );
        }

        HighscoreList{ entries: meter_vec }
    }
}



pub async fn init_db() -> Database {
    println!("initializing database");
    let con = SqlitePool::connect("sqlite:db/db.db").await.unwrap();
    // sqlx::query(
    //     "CREATE TABLE IF NOT EXISTS users (
    //   id                INTEGER PRIMARY KEY,
    //   username          TEXT NOT NULL,
    //   email             TEXT NOT NULL,
    //   hash              TEXT NOT NULL
    //   )",
    // )
    // .execute(&con)
    // .await
    // .unwrap();

    // sqlx::query("insert into users (username, email, hash) values ('asdf', 'asdf@asdf.com' , ?)")
    //     .bind(bcrypt::hash("asdf", bcrypt::DEFAULT_COST).unwrap())
    //     .execute(&con)
    //     .await
    //     .unwrap();

    // sqlx::query(
    //     "CREATE TABLE IF NOT EXISTS data (
    //   id INTEGER PRIMARY KEY,
    //   username TEXT NOT NULL,
    //   time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    //   meters UNSIGNED BIG INT NOT NULL)", /* FOREIGN KEY(username) REFERENCES users(username) */
    // )
    // .execute(&con)
    // .await
    // .unwrap();

    // let username = "testname";
    // let meters = 10;

    // sqlx::query(
    //     "INSERT INTO data
    //   (username, meters)
    //   VALUES (?, ?)",
    // )
    // .bind(username)
    // .bind(meters)
    // .execute(&con)
    // .await
    // .unwrap();

    sqlx::migrate!("./src/db/migrations")
        .run(&con)
        .await
        .unwrap();

    Database { conn: con }
}
