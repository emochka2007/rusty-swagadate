-- Your SQL goes here
CREATE table profile_activities (
    viewer_id         uuid not null,
    activity_count int not null default 0,
    PRIMARY KEY(viewer_id)
);
