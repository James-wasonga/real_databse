-- Your SQL goes here
 create table loans(
    id SERIAL PRIMARY KEY,
    user_id integer not null,
    amount numeric,
    interest_rate numeric,
    status bool default false,
    updated_at timestamp without time zone default CURRENT_TIMESTAMP not null,
    created_at timestamp without time zone default CURRENT_TIMESTAMP not null


 );