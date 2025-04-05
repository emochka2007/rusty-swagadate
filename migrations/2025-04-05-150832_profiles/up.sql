-- Your SQL goes here
CREATE extension if not exists "uuid-ossp";

CREATE table profiles
(
    id         uuid primary key default uuid_generate_v4(),
    user_id    bigint unique not null,
    username   text unique   not null,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp
);