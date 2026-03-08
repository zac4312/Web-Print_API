 create extension if not exists citext; 

ALTER TABLE users
    ADD COLUMN email citext not null;

ALTER TABLE vendors 
    ADD COLUMN email citext not null;

