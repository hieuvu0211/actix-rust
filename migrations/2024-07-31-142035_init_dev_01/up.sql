-- Your SQL goes here
create table account (
    id int primary key auto_increment,
    username varchar(255) not null,
    password varchar(255) not null,
    email varchar(255),
    created_at datetime default now(),
    updated_at datetime default now()
);