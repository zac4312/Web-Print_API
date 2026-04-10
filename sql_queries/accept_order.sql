UPDATE orders 
SET status = 'accepted'
where pub_id = $1;

