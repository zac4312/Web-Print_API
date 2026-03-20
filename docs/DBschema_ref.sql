CREATE EXTENSION IF NOT EXISTS "pgcrypto";

create type vacancy as enum ('available', 's_busy', 'busy', 'closed');
create type size as enum ('a4', 'a5', 'a3', 'letter', 'legal', 'tabloid');

CREATE TABLE users(    
    user_id uuid PRIMARY KEY default gen_random_uuid(),
    name varchar(16) not null,
    pw_hash varchar(255) not null,
     pub_id varchar(20) not null unique
);

CREATE TABLE vendors ( --now vedors-- 
    partner_id uuid PRIMARY KEY  default gen_random_uuid(),
    name varchar(25) not null,
    pw_hash varchar(255)not null,
    bw_rate decimal(10,2) not null,
    clrd_rate decimal(10,2) not null,
    location point not null,
    availability vacancy not null,
    pub_id varchar(20) not null unique
);

CREATE TABLE orders (
    order_id uuid PRIMARY KEY  default gen_random_uuid(),
    copies SmallInt not null,
    print_size size not null,
    color boolean not null, 
    pub_id varchar(20) not null unique,
    
    file_id uuid not null references files(file_id),
    for_vendor uuid not null references vendors(vendor_id),
    for_user uuid not null references users(user_id);

);

CREATE TABLE files (
    file_id uuid PRIMARY KEY default gen_random_uuid(),
    file_path text not null,
    file_size BigInt not null,
    mime_type varchar(100) not null,
    deleted_on TimeStamptz
)

create type state as enum ('rejected', 'paid', 'claimed', 'completed');

CREATE TABLE order_history (
    history_id uuid PRIMARY KEY default gen_random_uuid(),
    order_status state not null,
    created_at TimeStamptz default now(),
    paid_at TimeStamptz,
    claimed_at TimeStamptz,
    completed_at TimeStamptz,

    order_of uuid not null references orders(order_id)
)

create type payment_method as enum ('on-site', 'paypal');

CREATE TABLE payments (
   payment_id uuid PRIMARY KEY default gen_random_uuid(),
   transaction_id text not null,
   method payment_method not null,
   amount_paid DECIMAL(10,2),
   paid_at TimeStamptz,

   order_of uuid not null references orders(order_id);
);
