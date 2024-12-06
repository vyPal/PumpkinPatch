create table plugins (
    id serial primary key,
    name text not null,
    description text,
    author_id integer references users(id),
    publish_date timestamp default now(),
    last_update_date timestamp default now(),
    download_count integer default 0
);
