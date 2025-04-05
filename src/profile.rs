use crate::pg::establish_connection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::profiles;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    user_id: i64,
    username: String,
}

impl Profile {
    pub fn user_id(&self) -> &i64 {
        &self.user_id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn new(user_id: i64, username: Option<String>) -> Self {
        Self {
            user_id,
            username: username.unwrap(),
        }
    }

    pub fn insert(&self) -> anyhow::Result<Profile> {
        let connection = &mut establish_connection();

        let profile = diesel::insert_into(profiles::table)
            .values(self)
            .returning(Profile::as_returning())
            .get_result(connection)?;

        Ok(profile)
    }

    pub fn get_by_id(profile_id: Uuid) -> Profile {
        let connection = &mut establish_connection();
        profiles::dsl::profiles
            .find(profile_id)
            .select(Profile::as_select())
            .first(connection)
            .unwrap()
    }

    pub fn get_by_username(msg_username: &str) -> anyhow::Result<Option<Profile>> {
        use crate::schema::profiles::username;
        let connection = &mut establish_connection();
        Ok(profiles::dsl::profiles
            .filter(username.eq(msg_username))
            .select(Profile::as_select())
            .first(connection)
            .optional()?)
    }
}
