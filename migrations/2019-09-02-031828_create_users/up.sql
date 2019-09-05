-- Your SQL goes here

create table users(
    id serial primary key,
    name varchar not null,
    email varchar not null,
    password varchar not null ,
    UNIQUE(name) ,UNIQUE( email)

)