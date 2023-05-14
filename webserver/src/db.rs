


pub mod table {

    pub mod users {
        use diesel::{QueryDsl, RunQueryDsl};
        use crate::{DbError, PooledConn};
        use crate::models::User;
        use crate::schema::users::dsl::*;

        pub fn list_users(conn : &mut PooledConn) -> Result<Vec<User>, DbError> {
            let user : Vec<User> = users
                .limit(5)
                .load::<User>(conn)? ;

            Ok(user)
        }
    }

}