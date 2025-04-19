use crate::pg::establish_connection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::str::FromStr;
use uuid::Uuid;

use crate::schema::profiles;

pub enum ProfileGender {
    Male,
    Female,
}
impl ToString for ProfileGender {
    fn to_string(&self) -> String {
        match self {
            ProfileGender::Male => "MALE".to_string(),
            ProfileGender::Female => "FEMALE".to_string(),
        }
    }
}
impl FromStr for ProfileGender {
    type Err = DieselError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let gender = match s {
            "MALE" => Self::Male,
            "FEMALE" => Self::Female,
            _ => return Err(DieselError::NotFound),
        };
        Ok(gender)
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    id: Uuid,
    user_id: i64,
    username: String,
    description: String,
    displayed_name: String,
    location: String,
    age: i32,
    gender: String,
}

impl Profile {
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn user_id(&self) -> &i64 {
        &self.user_id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn displayed_name(&self) -> &str {
        &self.displayed_name
    }
    pub fn age(&self) -> &i32 {
        &self.age
    }
    pub fn location(&self) -> &str {
        &self.location
    }
    pub fn new(user_id: i64, username: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            username: username.unwrap(),
            description: String::new(),
            displayed_name: String::new(),
            location: String::new(),
            age: 0,
            gender: String::new(),
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

    pub fn update_age(msg_username: &str, new_age: i32) -> anyhow::Result<()> {
        use crate::schema::profiles::dsl::*;
        let connection = &mut establish_connection();
        diesel::update(profiles)
            .filter(username.eq(msg_username))
            .set(age.eq(new_age))
            .execute(connection)?;
        Ok(())
    }

    pub fn update_gender(msg_username: &str, profile_gender: &str) -> anyhow::Result<()> {
        use crate::schema::profiles::dsl::*;
        let connection = &mut establish_connection();
        diesel::update(profiles)
            .filter(username.eq(msg_username))
            .set(gender.eq(profile_gender))
            .execute(connection)?;
        Ok(())
    }

    pub fn get_by_id(profile_id: &Uuid) -> Profile {
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

    pub fn get_profile() -> Profile {
        let connection = &mut establish_connection();
        profiles::dsl::profiles
            .select(Profile::as_select())
            .first(connection)
            .unwrap()
    }
}
