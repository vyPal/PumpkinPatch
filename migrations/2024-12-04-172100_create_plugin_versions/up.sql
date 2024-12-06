create table plugin_versions (
    id serial primary key,
    plugin_id integer references plugins(id),
    version_number text not null,
    release_date timestamp default now(),
    download_count integer default 0,
    windows_url text,
    linux_url text,
    macos_url text
);
