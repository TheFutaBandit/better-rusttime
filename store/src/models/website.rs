use crate::store::Store;
use diesel::prelude::*;
use chrono::{NaiveDateTime, Utc};
// use diesel::sql_types::Timestamp;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub user_id: String,
    pub time_added: NaiveDateTime
}

impl Store {
    pub fn create_website(
        &mut self, url: String, 
        user_id: String
    ) -> Result<Website, diesel::result::Error> {
        let id = uuid::Uuid::new_v4();

        let w = Website {
            id: id.to_string(),
            url,
            user_id,
            time_added: Utc::now().naive_utc()
        };

        let website = diesel::insert_into(crate::schema::website::table)
            .values(&w)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;


        Ok(website)
    }

    pub fn get_website(&mut self, input_id: String, input_user_id: String) -> Result<Website, diesel::result::Error> {
        use crate::schema::website::dsl::*;

        let results = website
            .filter(id.eq(input_id))
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .first(&mut self.conn)?;

        Ok(results)

    }


}