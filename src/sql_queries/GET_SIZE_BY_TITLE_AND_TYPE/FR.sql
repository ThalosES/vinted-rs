SELECT * FROM SIZE WHERE UPPER(title_fr) = UPPER($1) AND UPPER(size_type_fr) = UPPER($2);