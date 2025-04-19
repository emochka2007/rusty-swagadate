use crate::pg::establish_connection;
use crate::schema::profile_activities::activity_count;
use crate::schema::profile_activities::dsl::profile_activities;
use crate::schema::profile_likes::viewer_id;
use diesel::{
    ExpressionMethods, Insertable, OptionalExtension, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::profile_activities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProfileActivity {
    viewer_id: Uuid,
    activity_count: i32,
}

impl Default for ProfileActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileActivity {
    pub fn new() -> Self {
        Self {
            viewer_id: Uuid::new_v4(),
            activity_count: 1,
        }
    }

    pub fn from_id(id: Uuid) -> Self {
        Self {
            viewer_id: id,
            activity_count: 1,
        }
    }

    pub fn viewer_id(&self) -> &Uuid {
        &self.viewer_id
    }

    pub fn upsert_and_increment(&self) -> anyhow::Result<Self> {
        use crate::schema::profile_activities;
        let connection = &mut establish_connection();
        let profile_activity = profile_activities::dsl::profile_activities
            .find(self.viewer_id)
            .select(ProfileActivity::as_select())
            .first(connection)
            .optional()?;
        let profile = match profile_activity {
            Some(profile) => profile,
            None => Self::insert(self)?,
        };
        profile.increment()?;
        Ok(profile)
    }

    pub fn insert(&self) -> anyhow::Result<Self> {
        use crate::schema::profile_activities;
        let connection = &mut establish_connection();

        let activity = diesel::insert_into(profile_activities::table)
            .values(self)
            .returning(ProfileActivity::as_returning())
            .get_result(connection)?;

        Ok(activity)
    }

    pub fn increment(&self) -> anyhow::Result<()> {
        use crate::schema::profile_activities::viewer_id;
        let connection = &mut establish_connection();
        let new_count = self.activity_count + 1;
        diesel::update(profile_activities)
            .filter(viewer_id.eq(self.viewer_id))
            .set(activity_count.eq(new_count))
            .execute(connection)?;
        Ok(())
    }

    pub fn get_most_active_profile() -> anyhow::Result<Self> {
        use crate::schema::profile_activities;
        use crate::schema::profile_activities::activity_count;
        let connection = &mut establish_connection();
        let profile_activity = profile_activities::dsl::profile_activities
            .select(ProfileActivity::as_select())
            .order(activity_count.desc())
            .first(connection)?;
        Ok(profile_activity)
    }
}
