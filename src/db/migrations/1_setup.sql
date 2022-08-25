create table if not exists users (
    id INTEGER primary key,
    username TEXT not null,
    email TEXT not null,
    hash TEXT not null
);
create table if not exists data (
    id INTEGER primary key,
    username TEXT not null,
    time DATETIME not null default current_timestamp,
    meters unsigned FLOAT not null
);