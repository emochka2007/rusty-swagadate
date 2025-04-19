use crate::profile::Profile;
use crate::profile_activities::ProfileActivity;
use crate::profile_view::ProfileView;
use crate::schema::profile_likes::profile_id;
use uuid::Uuid;

pub struct MatchEngine();

impl MatchEngine {
    pub fn match_profiles(viewer_id: &Uuid) -> anyhow::Result<Profile> {
        // Find profile, sort by activity
        // Check if it is in profile_views
        // Match by age
        let profile_activity = ProfileActivity::get_most_active_profile()?;
        let matched = Profile::get_by_id(profile_activity.viewer_id());
        let is_viewed = ProfileView::new(*viewer_id, *matched.id()).exists()?;
        if is_viewed {
            //todo try to find new profile
            Ok(matched)
        } else {
            Ok(matched)
        }
    }
}
