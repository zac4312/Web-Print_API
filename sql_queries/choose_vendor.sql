SELECT pub_id, availability as "availability: Vacancy" FROM vendors
where pub_id = $1;
