CREATE EXTENSION IF NOT EXISTS "pgcrypto";

create type vacancy as enum ('available', 's_busy', 'busy', 'closed');
create type size as enum ('a4', 'a5', 'a3', 'letter', 'legal', 'tabloid');

CREATE TABLE users(    
    user_id uuid PRIMARY KEY default gen_random_uuid(),
    name varchar(16) not null,
    pw_hash varchar(255) not null
);

CREATE TABLE partners ( --now vedors-- 
    partner_id uuid PRIMARY KEY  default gen_random_uuid(),
    name varchar(25) not null,
    pw_hash varchar(255)not null,
    bw_rate decimal(10,2) not null,
    clrd_rate decimal(10,2) not null,
    location point not null,
    availability vacancy not null
);

CREATE TABLE orders (
    order_id uuid PRIMARY KEY  default gen_random_uuid(),
    copies SmallInt not null,
    print_size size not null,
    color boolean not null,
    dt_stamp TimeStamptz default now(), 
    --added files--
    client_id uuid not null references users(user_id),
    shop_id  uuid not null references partners(partner_id)
);

CREATE TABLE files (
    file_id uuid PRIMARY KEY default gen_random_uuid(),
    file_path text not null,
    file_size BigInt not null,
    mime_type varchar(100) not null,
    deleted_on TimeStamptz,
    
    order_for uuid references orders(order_id) --removed--
);
