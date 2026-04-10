INSERT INTO users (name, pw_hash, email, pub_id) VALUES
('alice01', 'pw_test', 'alice01@test.com', 'test_USR_001'),
('bob02', 'pw_test', 'bob02@test.com', 'test_USR_002'),
('charlie03', 'pw_test', 'charlie03@test.com', 'test_USR_003');

INSERT INTO vendors 
(name, pw_hash, bw_rate, clrd_rate, lat, long, availability, email, pub_id, brand) 
VALUES
('printshop01', 'pw_test', 1.50, 5.00, 7.07, 125.61, 'available', 'printshop01@test.com', 'test_VND_001', 'QuickPrint Hub'),
('copycenter02', 'pw_test', 1.25, 4.75, 7.08, 125.60, 'closed', 'copycenter02@test.com', 'test_VND_002', 'City Copy Center'),
('fastprint03', 'pw_test', 1.75, 5.50, 7.06, 125.62, 'busy', 'fastprint03@test.com', 'test_VND_003', 'FastPrint Solutions');

INSERT INTO files (file_path, file_size, mime_type, deleted_on, pub_id) VALUES
('/uploads/orders/doc_001.pdf', 245760, 'application/pdf', NULL, 'FLE_01'),
('/uploads/orders/doc_002.pdf', 532480, 'application/pdf', NULL, 'FLE_02'),
('/uploads/orders/image_003.png', 1048576, 'image/png', NULL, 'FLE_03');
