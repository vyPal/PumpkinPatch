create table comments (
    id serial primary key,
    plugin_id integer references plugins(id),
    user_id integer references users(id),
    content text not null,
    timestamp timestamp default now()
);
