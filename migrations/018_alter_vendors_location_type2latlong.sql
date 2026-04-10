ALTER TABLE vendors 
     Drop COLUMN location,
    add COLUMN lat DOUBLE PRECISION not null DEFAULT 0,
    add column long DOUBLE PRECISION not null DEFAULT 0;
