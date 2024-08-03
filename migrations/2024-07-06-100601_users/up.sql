-- Your SQL goes here
create table users(
    id SERIAL PRIMARY KEY,
    user_types  integer  default 0,
    name character varying(150),
    email character varying(150),
    password text not null,
    phone_number varchar,
    bio text ,
    is_blocked bool default false ,
    blocked_reason text default null,
    address text,
    updated_at timestamp without time zone default CURRENT_TIMESTAMP not null,
    created_at timestamp without time zone default CURRENT_TIMESTAMP not null

);

insert into users(user_types,name,email,password,phone_number,bio,is_blocked,blocked_reason,address)
values('1','John','john@gmail.com','john01','0793778686','I love designing','true','debt payment','juja-thika');