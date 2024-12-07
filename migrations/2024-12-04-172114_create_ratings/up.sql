create table ratings (
    id serial primary key,
    plugin_id integer not null references plugins(id),
    user_id integer not null references users(id),
    rating integer not null check (rating between 1 and 5)
);
