INSERT INTO orders (copies, print_size, color, file_id, pub_id, for_vendor, for_user, total, status)
VALUES ($1, $2::size, $3, $4, $5, $6, $7, $8, $9);
