SELECT o.color, o.copies, o.print_size as "print_size: size", o.pub_id, o.status as "status: state", o.total, u.name, f.file_path
FROM orders o
LEFT JOIN vendors v
ON o.for_vendor = v.vendor_id
LEFT JOIN users u
ON u.user_id = o.for_user
LEFT JOIN files f
ON f.file_id = o.file_id
WHERE v.vendor_id = $1;
