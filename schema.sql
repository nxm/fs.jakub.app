CREATE DATABASE fileshare;
\connect fileshare;

create table files
(
    id            serial primary key,
    hash          varchar(50),
    name          varchar(255),
    md5 varchar(60),
    content_type  varchar(50),
    size          int,
    created_at    timestamp default current_timestamp
);

