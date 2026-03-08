ALTER TABLE users 
    ADD COLUMN pub_id varchar(20) UNIQUE not null;

ALTER TABLE vendors 
    ADD COLUMN pub_id varchar(20) UNIQUE not null;
