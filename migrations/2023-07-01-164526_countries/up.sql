-- Your SQL goes here
CREATE TABLE COUNTRY (
    id INT PRIMARY KEY,
    name VARCHAR(30),
    local_name VARCHAR(30),
    iso_code VARCHAR(2),
    flag VARCHAR(5)
);



INSERT INTO COUNTRY(id, name, local_name, iso_code , flag) VALUES 
(16 , 'France' , 'France', 'FR', '🇨🇵'),
(19 , 'Belgique' , 'Belgique / België', 'BE', '🇧🇪'),
(7 , 'Espagne' , 'España', 'ES', '🇪🇸'),
(20 , 'Luxembourg' , 'Luxembourg', 'LU', '🇱🇺'),
(10 , 'Pays-Bas' , 'Nederland', 'NL', '🇳🇱'),
(1 , 'Lituanie' , 'Lietuva', 'LT', '🇱🇹'),
(2 , 'Allemagne ' , 'Deutschland', 'DE', '🇩🇪'),
(4 , 'Autriche' , 'Österreich', 'AT', '🇦🇹'),
(18 , 'Italie' , 'Italia', 'IT', '🇮🇹'),
(13 , 'Royaume-Uni' , 'United Kingdom', 'GB', '🇬🇧'),
(21 , 'Portugal' , 'Portugal', 'PT','🇵🇹'),
(14 , 'États-Unis' , 'United States', 'US', '🇺🇸'),
(3 , 'République tchèque' , 'Česká republika', 'CZ', '🇨🇿'),
(22 , 'Slovaquie' , 'Slovenská republika', 'SK', '🇸🇰'),
(15 , 'Pologne' , 'Polska', 'PL', '🇵🇱'),
(12 , 'Suède' , 'Sverige', 'SE', '🇸🇪'),
(25 , 'Roumanie' , 'România', 'RO', '🇷🇴'),
(24 , 'Hongrie' , 'Magyarország', 'HU', '🇭🇺');
