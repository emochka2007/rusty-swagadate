use crate::pg::establish_connection;
use diesel::dsl::exists;
use diesel::{
    ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
    select,
};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::profile_views)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProfileView {
    viewer_id: Uuid,
    profile_id: Uuid,
}

impl ProfileView {
    pub fn new(viewer_id: Uuid, profile_id: Uuid) -> Self {
        Self {
            viewer_id,
            profile_id,
        }
    }

    pub fn insert(&self) -> anyhow::Result<Self> {
        let connection = &mut establish_connection();
        use crate::schema::profile_views;
        let profile_view = diesel::insert_into(profile_views::table)
            .values(self)
            .returning(ProfileView::as_returning())
            .get_result(connection)?;

        Ok(profile_view)
    }

    pub fn exists(&self) -> anyhow::Result<bool> {
        use crate::schema::profile_views::dsl::*;
        use diesel::prelude::*;
        use diesel::select;
        let connection = &mut establish_connection();
        let result = select(exists(profile_views.filter(viewer_id.eq(self.viewer_id))));
        Ok(result.get_result::<bool>(connection)?)
    }
}
