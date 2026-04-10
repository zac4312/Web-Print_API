UPDATE orders
set status = 'rejected'
WHERE pub_id = $1;
