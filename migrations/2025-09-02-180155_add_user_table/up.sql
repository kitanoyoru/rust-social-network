-- Your SQL goes here
create type if not exists user_role as enum ('user', 'moderator', 'admin');

create table if not exists users (
    id uuid primary key not null,
    email text not null,
    username text not null,
    first_name text,
    last_name text,
    bio text,
    avatar_url text,
    password_hash text not null,
    role user_role not null,
    is_active boolean not null default true,
    is_verified boolean not null default false,
    is_private boolean not null default false,
    followers_count int not null default 0,
    following_count int not null default 0,
    posts_count int not null default 0,
    last_login_at timestamp with time zone not null default now(),
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    deleted_at timestamp with time zone not null default now(),
);