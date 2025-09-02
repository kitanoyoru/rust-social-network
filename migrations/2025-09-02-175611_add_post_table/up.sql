-- Your SQL goes here
create type post_type as enum ('text', 'image', 'video', 'link', 'poll');

create type post_visibility as enum ('public', 'followers', 'private');

create table if not exists posts (
    id uuid primary key not null,
    user_id uuid not null,
    content text not null,
    post_type post_type not null,
    media_urls text[] not null,
    visibility post_visibility not null,
    like_count int not null,
    comment_count int not null,
    share_count int not null,
    view_count int not null,
    tags text[] not null,
    loc text,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    deleted_at timestamp with time zone not null default now(),
);

alter table posts add constraint fk_posts_user_id foreign key (user_id) references users (id);