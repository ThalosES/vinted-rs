-- SELECT * FROM SIZE WHERE title_es = UPPER('XL') AND size_type_es = 'Pantalones de hombre';

SELECT * FROM SIZE WHERE $1 = $3 AND $2 = $4 