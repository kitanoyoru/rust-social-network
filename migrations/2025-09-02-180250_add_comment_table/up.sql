-- Your SQL goes here
create type if not exists comment_status as enum ('active', 'hidden', 'deleted', 'flagged');

create table if not exists comments (
    id uuid primary key not null,
    user_id uuid not null,
    post_id uuid not null,
    content text not null,
    media_urls text[] not null,
    like_count int not null,
    reply_count int not null,
    status comment_status not null,
    hashtags text[] not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    deleted_at timestamp with time zone not null default now(),
);

alter table comments add constraint fk_comments_user_id foreign key (user_id) references users (id);
alter table comments add constraint fk_comments_post_id foreign key (post_id) references posts (id);