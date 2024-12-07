create table plugins (
    id serial primary key,
    name text not null,
    description text not null,
    author_id integer not null references users(id),
    publish_date timestamp not null default now(),
    last_update_date timestamp not null default now(),
    download_count integer not null default 0
);
