-- Your SQL goes here
CREATE table profile_views (
    viewer_id         uuid  not null,
    profile_id uuid not null,
    PRIMARY KEY(viewer_id, profile_id)
);

CREATE table profile_likes (
    viewer_id  uuid not null,
    profile_id uuid not null,
    PRIMARY KEY(viewer_id, profile_id)
);

CREATE table profile_superlikes (
    viewer_id         uuid not null,
    profile_id uuid not null,
    PRIMARY KEY(viewer_id, profile_id)
);
