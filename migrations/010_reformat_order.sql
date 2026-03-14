ALTER TABLE vendors
     drop column partner_id cascade,
     add column vendor_id uuid PRIMARY KEY default gen_random_uuid();

ALTER TABLE orders 
    drop column dt_stamp,
    drop column client_id,
    drop column shop_id,

    add column for_vendor uuid not null references vendors(vendor_id),
    add column for_user uuid not null references users(user_id);


