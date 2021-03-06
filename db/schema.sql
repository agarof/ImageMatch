create extension if not exists pgcrypto;

drop table if exists users,tokens,sessions,registrations,images,images_associations,results,choices;

create table if not exists users
(
    id serial primary key,
    email text unique not null,
    password text not null,
    admin boolean not null default false,
    confirm_limit timestamptz default CURRENT_TIMESTAMP + make_interval(days => 3)
);

create table if not exists tokens
(
    token uuid primary key default gen_random_uuid(),
    expiration timestamptz not null default CURRENT_TIMESTAMP + make_interval(days => 7),
    user_id integer not null
        references users (id) on delete cascade
);

create table if not exists sessions
(
    id serial primary key,
    name text not null,
    phase1 timestamptz not null,
    phase2 timestamptz not null check(phase1 < phase2),
    phase3 timestamptz not null check(phase2 < phase3)
);

create table if not exists images
(
    id serial primary key
);

create table if not exists registrations
(
    id serial primary key,
    user_id integer not null
        references users(id) on delete cascade,
    session_id integer not null
        references sessions(id) on delete cascade,
    unique (user_id, session_id)
);

create table if not exists images_associations
(
    id serial primary key,
    image_id integer not null
        references images(id) on delete cascade,
    session_id integer not null
        references sessions(id) on delete cascade,
    unique (image_id, session_id)
);

create table if not exists choices
(
    id serial primary key,
    user_id integer not null
        references users(id) on delete cascade,
    session_id integer not null
        references sessions(id) on delete cascade,
    image_id integer not null
        references images(id) on delete cascade,
    unique (user_id,session_id)
);

create table if not exists results
(
    id serial primary key,
    session_id integer not null
        references sessions(id) on delete cascade,
    image_id integer not null
        references images(id) on delete cascade,
    user_1_id integer not null
        references users(id) on delete cascade,
    user_2_id integer not null
        references users(id) on delete cascade,
    user_3_id integer
        references users(id) on delete cascade
);
