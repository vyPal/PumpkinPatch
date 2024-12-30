create table users (
    id serial primary key,
    username text not null unique,
    oidc_id text not null unique,
    email text not null unique,
    access_token text not null,
    refresh_token text,
    created_at timestamp not null default now()
);
