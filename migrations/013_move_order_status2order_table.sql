ALTER TABLE order_history
    drop column order_status;

ALTER TABLE orders 
    add column status state not null;
