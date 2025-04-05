-- Your SQL goes here
ALTER table profiles
ADD column description text not null default '',
ADD column file_ids text[] null,
ADD column displayed_name text not null default '',
ADD column age int not null default 0;
