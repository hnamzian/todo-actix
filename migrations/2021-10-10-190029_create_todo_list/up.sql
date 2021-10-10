-- This file should undo anything in `up.sql`
create table todo_list (
    id serial primary key,
    title varchar(150) not null
);