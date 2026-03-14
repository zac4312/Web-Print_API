ALTER TABLE payments 
     add column order_of uuid not null references orders(order_id);
