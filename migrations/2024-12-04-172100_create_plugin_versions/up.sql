create table plugin_versions (
    id serial primary key,
    plugin_id integer not null references plugins(id),
    version_number text not null,
    release_date timestamp not null default now(),
    download_count integer not null default 0,
    windows_url text,
    linux_url text,
    macos_url text
);
