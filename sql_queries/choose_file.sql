INSERT INTO files (file_path, file_size, mime_type, pub_id) 
VALUES ($1, $2, $3, $4)
RETURNING file_id;
