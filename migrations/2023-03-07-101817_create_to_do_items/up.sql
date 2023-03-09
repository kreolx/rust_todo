-- Your SQL goes here
CREATE TABLE to_do (
    id SERIAL PRIMARY KEY ,
    title VARCHAR NOT NULL ,
    status varchar not null ,
    date timestamp not null default NOW()
)