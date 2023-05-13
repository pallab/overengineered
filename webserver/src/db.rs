use crate::models::*;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

pub fn create_connection(db_url : &str) -> MysqlConnection {
    MysqlConnection::establish(db_url).expect("Connection failed")
}

pub mod table {

    pub mod users {
        use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};
        use crate::models::User;
        use crate::schema::users::dsl::*;

        pub fn list_users(conn : &mut MysqlConnection) -> Vec<User> {
            users
                .limit(5)
                .load::<User>(conn)
                .expect("Error loading users")
        }
    }

}