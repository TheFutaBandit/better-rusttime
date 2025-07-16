use diesel::prelude::*;
use crate::store::Store;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl Store {
    pub fn sign_up(&mut self, username: String, password: String) -> Result<String, diesel::result::Error> {
        let id = uuid::Uuid::new_v4();

        let u = User {
            id: id.to_string(),
            username,
            password
        };

        diesel::insert_into(crate::schema::user::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;
        
        Ok(id.to_string())
    } 

    pub fn sign_in(&mut self, input_username: String, input_password: String) -> Result<String, diesel::result::Error> {
        use crate::schema::user::dsl::*;

        let results = user
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;

        if results.password != input_password {
            return Err(diesel::result::Error::NotFound);
        }

        Ok(results.id)
    }
}