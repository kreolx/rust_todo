-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY ,
    username VARCHAR NOT NULL UNIQUE ,
    email VARCHAR NOT NULL UNIQUE ,
    password VARCHAR NOT NULL ,
    unique_id VARCHAR NOT NULL
);

INSERT INTO users (username, email, password, unique_id)
VALUES ('kreolx', 'kreolx@mail.ru', '224515', '478e8e3f-26d3-429a-b1a1-74e1e2aa4843');

ALTER TABLE to_do ADD user_id integer default 1
CONSTRAINT user_id REFERENCES users NOT NULL ;