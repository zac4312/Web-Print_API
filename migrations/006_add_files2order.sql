ALTER TABLE orders 
    add column file_id uuid references files(file_id);
