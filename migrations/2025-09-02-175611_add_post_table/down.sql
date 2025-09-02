-- This file should undo anything in `up.sql`
drop type if exists post_visibility;
drop type if exists post_type;
drop table if exists posts;