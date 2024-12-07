create table comments (
    id serial primary key,
    plugin_id integer not null references plugins(id),
    user_id integer not null references users(id),
    content text not null,
    timestamp timestamp not null default now()
);
