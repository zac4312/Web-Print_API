SELECT vendor_id, availability as "availability: Vacancy" ,email, brand, bw_rate, clrd_rate, lat, long  FROM vendors
where pub_id = $1;
