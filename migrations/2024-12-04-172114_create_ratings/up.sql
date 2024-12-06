create table ratings (
    id serial primary key,
    plugin_id integer references plugins(id),
    user_id integer references users(id),
    rating integer check (rating between 1 and 5)
);
