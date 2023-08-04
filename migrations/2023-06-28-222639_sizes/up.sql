-- Your SQL goes here
-- Regex aún más loca
-- \b(\d+)\b(.*?)(\s*'[^']+'\s*)(\d+)
-- susituir por ($1$2$3, $4),

CREATE TABLE SIZE(
id INT, --Es única
title_es VARCHAR(80),
size_type_es VARCHAR(80),
title_en VARCHAR(80),
size_type_en VARCHAR(80),
title_fr VARCHAR(80),
size_type_fr VARCHAR(80),
category_id INT
);

INSERT INTO SIZE (id , title_es, size_type_es, title_en, size_type_en, title_fr , size_type_fr, category_id) VALUES 

1487, '30 x 50 cm', '30 x 50 cm', '30 x 50 cm', 'Medidas del cojín', 'Tailles de coussins', 'Cushion dimensions',1918
1488, '35 x 50 cm', '35 x 50 cm', '35 x 50 cm', 'Medidas del cojín', 'Tailles de coussins', 'Cushion dimensions',1918
1366, '40 x 40 cm', '40 x 40 cm', '40 x 40 cm', 'Medidas del cojín', 'Tailles de coussins', 'Cushion dimensions',1918
1367, '40 x 60 cm', '40 x 60 cm', '40 x 60 cm', 'Medidas del cojín', 'Tailles de coussins', 'Cushion dimensions',1918
1368, '45 x 45 cm', '45 x 45 cm', '45 x 45 cm', 'Medidas del cojín', 'Tailles de coussins', 'Cushion dimensions',1918
1369, '50 x 50 cm', '50 x 50 cm', '50 x 50 cm', 'Medidas del cojín', 'Tailles de coussins', 'Cushion dimensions',1918
1370, '60 x 60 cm', '60 x 60 cm', '60 x 60 cm', 'Medidas del cojín', 'Tailles de coussins', 'Cushion dimensions',1918
1371, '65 x 65 cm', '65 x 65 cm', '65 x 65 cm', 'Medidas del cojín', 'Tailles de coussins', 'Cushion dimensions',1918
1372, 'Otra', 'Autre', 'Other', 'Medidas del cojín', 'Tailles de coussins', 'Cushion dimensions',1918
1462, '35 x 40 cm', '35 x 40 cm', '35 x 40 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1463, '40 x 40 cm', '40 x 40 cm', '40 x 40 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1464, '40 x 60 cm', '40 x 60 cm', '40 x 60 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1465, '40 x 75 cm', '40 x 75 cm', '40 x 75 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1466, '40 x 80 cm', '40 x 80 cm', '40 x 80 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1467, '40 x 90 cm', '40 x 90 cm', '40 x 90 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1468, '50 x 50 cm', '50 x 50 cm', '50 x 50 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1469, '50 x 60 cm', '50 x 60 cm', '50 x 60 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1470, '50 x 66 cm', '50 x 66 cm', '50 x 66 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1471, '50 x 70 cm', '50 x 70 cm', '50 x 70 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1472, '50 x 90 cm', '50 x 90 cm', '50 x 90 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1473, '65 x 65 cm', '65 x 65 cm', '65 x 65 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1474, '80 x 80 cm', '80 x 80 cm', '80 x 80 cm', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1475, 'Otra', 'Autre', 'Other', 'Medidas de la funda de almohada', 'Tailles de taies d''oreiller', 'Pillowcase dimensions',1918
1476, '70 x 90 cm', '70 x 90 cm', '70 x 90 cm', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1477, '80 x 100 cm', '80 x 100 cm', '80 x 100 cm', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1478, '90 x 90 cm', '90 x 90 cm', '90 x 90 cm', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1479, '100 x 150 cm', '100 x 150 cm', '100 x 150 cm', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1480, '110 x 150 cm', '110 x 150 cm', '110 x 150 cm', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1481, '110 x 170 cm', '110 x 170 cm', '110 x 170 cm', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1482, '120 x 160 cm', '120 x 160 cm', '120 x 160 cm', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1483, '125 x 150 cm', '125 x 150 cm', '125 x 150 cm', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1484, '130 x 170 cm', '130 x 170 cm', '130 x 170 cm', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1485, '150 x 200 cm', '150 x 200 cm', '150 x 200 cm', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1486, 'Otro', 'Autre', 'Other', 'Medidas de la manta', 'Tailles de couvertures', 'Blanket dimensions',1918
1492, 'Individual (135-150 cm x 200-220 cm)', 'Simple (135-150 cm x 200-220 cm)', 'Single', 'Medidas del edredón', 'Tailles de couette', 'Duvet size',1918
1493, 'Matrimonio (155-190 cm x 200-220 cm)', 'Double (155-190 cm x 200-220 cm)', 'Double', 'Medidas del edredón', 'Tailles de couette', 'Duvet size',1918
1494, 'Queen Size (200 cm x 200-220 cm)', 'Queen Size (200 cm x 200-220 cm)', 'King', 'Medidas del edredón', 'Tailles de couette', 'Duvet size',1918
1495, 'King Size (220-240 cm x 220-260 cm)', 'King Size (220-240 cm x 220-260 cm)', 'Super King', 'Medidas del edredón', 'Tailles de couette', 'Duvet size',1918
1457, 'Hasta 149 cm', 'Jusqu''à 149 cm', 'Up to 149 cm', 'Largo de las cortinas', 'Longueur de rideau', 'Curtain length',1918
1458, '150-199 cm', '150 cm - 199 cm', '150 cm - 199 cm', 'Largo de las cortinas', 'Longueur de rideau', 'Curtain length',1918
1459, '200-249 cm', '200 cm - 249 cm', '200 cm - 249 cm', 'Largo de las cortinas', 'Longueur de rideau', 'Curtain length',1918
1460, '250-299 cm', '250 cm - 299 cm', '250 cm - 299 cm', 'Largo de las cortinas', 'Longueur de rideau', 'Curtain length',1918
1461, '300 cm o más', '300 cm et plus', '300 cm and up', 'Largo de las cortinas', 'Longueur de rideau', 'Curtain length',1918
1312, 'Individual (90–105 cm x 190–200 cm)', 'Simple (70–100cm x 190–200cm)', 'Single', 'Ropa de cama', 'Literie', 'Bedding',1918
1313, 'Matrimonio (120-140 x 190-200 cm)', 'Double (120-140 x 190-200 cm)', 'Double', 'Ropa de cama', 'Literie', 'Bedding',1918
1314, 'Queen Size (160-170 x 190-200 cm)', 'Queen Size (160-170 x 190-200 cm)', 'King', 'Ropa de cama', 'Literie', 'Bedding',1918
1315, 'King Size (180–200 cm x 200 cm)', 'King Size (180–200cm x 200cm)', 'Super King', 'Ropa de cama', 'Literie', 'Bedding',1918
206, 'XS', 'XS', 'XS', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
207, 'S', 'S', 'S', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
208, 'M', 'M', 'M', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
209, 'L', 'L', 'L', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
210, 'XL', 'XL', 'XL', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
211, 'XXL', 'XXL', 'XXL', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
212, 'XXXL', 'XXXL', 'XXXL', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
308, '4XL', '4XL', '4XL', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
309, '5XL', '5XL', '5XL', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
1192, '6XL', '6XL', '6XL', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
1193, '7XL', '7XL', '7XL', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
1194, '8XL', '8XL', '8XL', 'Ropa', 'Tailles hommes', 'Men''s sizes',5
1631, 'W23 | ES 32', 'W23 | FR 32', 'W23', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1632, 'W24 | ES 34', 'W24 | FR 34', 'W24', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1633, 'W25 | ES 34', 'W25 | FR 34', 'W25', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1634, 'W26 | ES 36', 'W26 | FR 36', 'W26', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1635, 'W27 | ES 36', 'W27 | FR 36', 'W27', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1636, 'W28 | ES 38', 'W28 | FR 38', 'W28', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1637, 'W29 | ES 38', 'W29 | FR 38', 'W29', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1638, 'W30 | ES 40', 'W30 | FR 40', 'W30', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1639, 'W31 | ES 41', 'W31 | FR 40', 'W31', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1640, 'W32 | ES 42', 'W32 | FR 42', 'W32', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1641, 'W33 | ES 42', 'W33 | FR 42', 'W33', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1642, 'W34 | ES 44', 'W34 | FR 44', 'W34', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1662, 'W35 | ES 44', 'W35 | FR 44', 'W35', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1643, 'W36 | ES 46', 'W36 | FR 46', 'W36', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1644, 'W38 | ES 48', 'W38 | FR 48', 'W38', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1645, 'W40 | ES 50', 'W40 | FR 50', 'W40', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1646, 'W42 | ES 52', 'W42 | FR 52', 'W42', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1647, 'W44 | ES 54', 'W44 | FR 54', 'W44', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1648, 'W46 | ES 56', 'W46 | FR 56', 'W46', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1649, 'W48 | ES 58', 'W48 | FR 58', 'W48', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1650, 'XS', 'XS', 'XS', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1651, 'S', 'S', 'S', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1652, 'M', 'M', 'M', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1653, 'L', 'L', 'L', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1654, 'XL', 'XL', 'XL', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1655, '2XL', '2XL', 'XXL', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1656, '3XL', '3XL', 'XXXL', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1657, '4XL', '4XL', '4XL', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1658, '5XL', '5XL', '5XL', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1659, '6XL', '6XL', '6XL', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1660, '7XL', '7XL', '7XL', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1661, '8XL', '8XL', '8XL', 'Pantalones de hombre', 'Pantalons homme', 'Men''s trousers',5
1545, 'XS', 'XS', 'XS', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1546, 'S', 'S', 'S', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1547, 'M', 'M', 'M', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1548, 'L', 'L', 'L', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1549, 'XL', 'XL', 'XL', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1550, 'XXL', 'XXL', 'XXL', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1551, 'XXXL', 'XXXL', 'XXXL', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1552, '4XL', '4XL', '4XL', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1553, '5XL', '5XL', '5XL', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1554, '6XL', '6XL', '6XL', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1555, '7XL', '7XL', '7XL', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1556, '8XL', '8XL', '8XL', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1527, '35', '35 cm','13.5 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1528, '36', '36 cm','14 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1529, '37', '37 cm','14.5 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1530, '38', '38 cm','15 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1531, '39', '39 cm','15.5 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1532, '40', '40 cm','16 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1533, '41', '41 cm','16.5 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1534, '42', '42 cm','17 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1535, '43', '43 cm','17.5 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1536, '44', '44 cm','18 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1537, '45', '45 cm','18.5 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1538, '46', '46 cm','19 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1539, '47', '47 cm','19.5 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1540, '48', '48 cm','20 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1541, '49', '49 cm','20.5 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1542, '50', '50 cm','21 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1543, '51', '51 cm','21.5 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1544, '52', '52 cm','22 inch', 'Camisas de hombre', 'Chemises homme', 'Men''s shirts',5
1609, 'XS', 'XS', 'XS', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1610, 'S', 'S', 'S', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1611, 'M', 'M', 'M', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1612, 'L', 'L', 'L', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1613, 'XL', 'XL', 'XL', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1614, 'XXL', 'XXL', 'XXL', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1615, 'XXXL', 'XXXL', 'XXXL', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1616, '4XL', '4XL', '4XL', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1617, '5XL', '5XL', '5XL', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1618, '6XL', '6XL', '6XL', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1619, '7XL', '7XL', '7XL', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1620, '8XL', '8XL', '8XL', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1583, '22', '22', '32S', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1584, '23', '23', '34S', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1585, '24', '24', '36S', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1586, '25', '25', '38S', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1587, '26', '26', '40S', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1588, '27', '27', '42S', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1589, '28', '28', '44S', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1590, '29', '29', '46S', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1591, '30', '30', '48S', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1592, '42', '42', '32R', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1593, '44', '44', '34R', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1594, '46', '46', '36R', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1595, '48', '48', '38R', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1596, '50', '50', '40R', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1597, '52', '52', '42R', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1598, '54', '54', '44R', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1599, '56', '56', '46R', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1600, '58', '58', '48R', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1601, '90', '90', '36L', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1602, '94', '94', '38L', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1603, '98', '98', '40L', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1604, '102', '102', '42L', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1605, '106', '106', '44L', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1606, '110', '110', '46L', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1607, '112', '112', '48L', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
1608, '118', '118', '50L', 'Tallas de chaquetas de traje para hombre', 'Vestes de costume pour homme', 'Men''s suit jackets',5
776, '38', '38', '4', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
777, '38', '38', '4.5','5','5', 'Men''s shoes',
778, '39', '39', '5', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
779, '39', '39', '5.5','5','5', 'Men''s shoes',
780, '40', '40', '6', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
781, '40', '40', '6.5','5','5', 'Men''s shoes',
782, '41', '41', '7', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
783, '41', '41', '7.5','5','5', 'Men''s shoes',
784, '42', '42', '8', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
785, '42', '42', '8.5','5','5', 'Men''s shoes',
786, '43', '43', '9', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
787, '43', '43', '9.5','5','5', 'Men''s shoes',
788, '44', '44', '10', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
789, '44', '44', '10.5','5','5', 'Men''s shoes',
790, '45', '45', '11', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
791, '45', '45', '11.5','5','5', 'Men''s shoes',
792, '46', '46', '12', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
793, '46', '46', '12.5','5','5', 'Men''s shoes',
794, '47', '47', '13', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
795, '47', '47', '13.5','5','5', 'Men''s shoes',
1190, '48', '48', '14', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
1621, '48', '48', '14.5','5','5', 'Men''s shoes',
1191, '49', '49', '15', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
1327, '50', '50', '15.5', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
1622, '51', '51', '16', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
1623, '52', '52', '16.5', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
1624, 'Talla única', 'Taille unique', 'One size', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
1625, 'Otra', 'Autre', 'Other', 'Calzado', 'Chaussures hommes', 'Men''s shoes',5
1202, '14', '14', '14.1 mm / F½','1 mm Ø / 4.1 mm', 'Rings',
1203, '14', '14', '14.5 mm / G½','5 mm Ø / 5.5 mm', 'Rings',
1204, '14', '14', '14.9 mm / H½','9 mm Ø / 6,9 mm', 'Rings',
1205, '15', '15', '15.3 mm / I½','3 mm Ø / 8.3 mm', 'Rings',
1206, '15', '15', '15.7 mm / J½','7 mm Ø / 9,7 mm', 'Rings',
1207, '16', '16', '16.1 mm / K½','1 mm Ø / 10,1 mm', 'Rings',
1208, '16', '16', '16.5 mm / L½','5 mm Ø / 12.5 mm', 'Rings',
1209, '16', '16', '16.9 mm / M½','9 mm Ø / 13,9 mm', 'Rings',
1210, '17', '17', '17.3 mm / N½','3 mm Ø / 14,3 mm', 'Rings',
1211, '17', '17', '17.7 mm / O½','7 mm Ø / 16.7 mm', 'Rings',
1212, '18', '18', '18.1 mm / P½','1 mm Ø / 18,1 mm', 'Rings',
1213, '18', '18', '18.5 mm / Q½','5 mm Ø / 19,5 mm', 'Rings',
1214, '1.9 mm Ø / 21', '19 mm', '19 mm / R½', 'Anillos', 'Bagues', 'Rings',1904
1215, '19', '19', '19.4 mm / S½','4 mm Ø / 22,4 mm', 'Rings',
1216, '19', '19', '19.8 mm / T½','8 mm Ø / 23,8 mm', 'Rings',
1217, '20', '20', '20.2 mm / U½','2 mm Ø / 25.2 mm', 'Rings',
1218, '20', '20', '20.6 mm / V½','6 mm Ø / 26.6 mm', 'Rings',
1219, '2.1 mm Ø / 27', '21 mm', '21 mm / W½', 'Anillos', 'Bagues', 'Rings',1904
1220, '21', '21', '21.4 mm / X½','4 mm Ø / 29.4 mm', 'Rings',
1221, '21', '21', '21.8 mm / Z','8 mm Ø / 30.8 mm', 'Rings',
1222, '22', '22', '22.2 mm / Z+1','2 mm Ø / 31.2 mm', 'Rings',
1223, '22', '22', '22.6 / Z+2','6 mm Ø / 32.6 mm', 'Rings',
1224, 'Ajustable', 'Ajustable', 'Adjustable', 'Anillos', 'Bagues', 'Rings',1904
1388, 'XS', 'XS', 'XS', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1389, 'S', 'S', 'S', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1390, 'M', 'M', 'M', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1391, 'L', 'L', 'L', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1392, 'XL', 'XL', 'XL', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1393, 'XXL', 'XXL', 'XXL', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1387, 'Talla única', 'Taille unique', 'One size', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1373, '52', '52', 'UK 6 3/8', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1374, '53', '53', 'UK 6 1/2', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1375, '54', '54', 'UK 6 5/8', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1376, '55', '55', 'UK 6 3/4', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1377, '56', '56', 'UK 6 7/8', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1378, '57', '57', 'UK 7', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1379, '58', '58', 'UK 7 1/8', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1380, '59', '59', 'UK 7 1/4', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1381, '60', '60', 'UK 7 3/8', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1382, '61', '61', 'UK 7 1/2', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1383, '62', '62', 'UK 7 5/8', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1384, '63', '63', 'UK 7 3/4', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1385, '64', '64', 'UK 7 7/8', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1386, '65', '65', 'UK 8', 'Sombreros de adulto', 'Chapeaux adulte', 'Adult hats',1904
1429, '30 mm o menos', 'Jusqu''à 30 mm', '30 mm and under', 'Relojes', 'Montres', 'Watches',1904
1430, '31-38 mm', '31 mm - 38 mm', '31 mm - 38 mm', 'Relojes', 'Montres', 'Watches',1904
1431, '39-42 mm', '39 mm - 42 mm', '39 - 42 mm', 'Relojes', 'Montres', 'Watches',1904
1432, '43-46 mm', '43 mm - 46 mm', '43 - 46 mm', 'Relojes', 'Montres', 'Watches',1904
1433, '47 mm o más', '47 mm et plus', '47 mm and above', 'Relojes', 'Montres', 'Watches',1904
1435, 'Talla única', 'Taille unique', 'One size', 'Relojes', 'Montres', 'Watches',1904
1408, '5', '5', '5', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1409, '5', '5', '5.5','5','5', 'Adult gloves',
1410, '6', '6', '6', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1411, '6', '6', '6.5','5','5', 'Adult gloves',
1412, '7', '7', '7', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1413, '7', '7', '7.5','5','5', 'Adult gloves',
1414, '8', '8', '8', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1415, '8', '8', '8.5','5','5', 'Adult gloves',
1416, '9', '9', '9', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1417, '9', '9', '9.5','5','5', 'Adult gloves',
1418, '10', '10', '10', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1419, '10', '10', '10.5','5','5', 'Adult gloves',
1420, '11', '11', '11', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1421, '11', '11', '11.5','5','5', 'Adult gloves',
1422, '12', '12', '12', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1423, 'Talla única', 'Taille unique', 'One size', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1424, 'XS', 'XS', 'XS', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1425, 'S', 'S', 'S', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1426, 'M', 'M', 'M', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1427, 'L', 'L', 'L', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1428, 'XL', 'XL', 'XL', 'Guantes de adulto', 'Gants adulte', 'Adult gloves',1904
1436, '80 cm', '80 cm', '32 inches', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1437, '85 cm', '85 cm', '34 inches', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1438, '90 cm', '90 cm', '36 inches', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1439, '95 cm', '95 cm', '38 inches', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1440, '100 cm', '100 cm', '40 inches', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1441, '105 cm', '105 cm', '42 inches', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1442, '110 cm', '110 cm', '44 inches', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1443, '115 cm', '115 cm', '45 inches', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1444, '120 cm', '120 cm', '47 inches', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1445, '125 cm', '125 cm', '49 inches', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1581, 'Ajustable', 'Ajustable', 'Adjustable', 'Cinturones de hombre', 'Ceintures homme', 'Men''s belts',5
1522, 'XS | 36-37', 'XS | 36-37', 'XS | UK 2-4', 'Calcetines de hombre', 'Chaussettes homme', 'Men''s socks',5
1523, 'S | 38-42', 'S | 38-42', 'S | UK 5-8', 'Calcetines de hombre', 'Chaussettes homme', 'Men''s socks',5
1524, 'M | 43-46', 'M | 43-46', 'M | UK 9-12', 'Calcetines de hombre', 'Chaussettes homme', 'Men''s socks',5
1525, 'L | 47-52', 'L | 47-52', 'L | UK 13-16', 'Calcetines de hombre', 'Chaussettes homme', 'Men''s socks',5
1526, 'Talla única', 'Taille unique', 'One size', 'Calcetines de hombre', 'Chaussettes homme', 'Men''s socks',5
610, 'Prematuro', 'Prématuré', 'Preemie', 'máx. 44 cm', 'jusqu''à 44cm', 'up to 44cm',1193
666, 'Recién nacido / 44 cm', 'Naissance / 44 cm', 'Newborns / 44 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
612, 'De hasta 1 mes / 50 cm', 'Jusqu''à 1 mois / 50 cm', 'Up to 1 month / 50 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
613, '1-3 meses / 56 cm', '1-3 mois / 56 cm', '1-3 months / 56 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
614, '3-6 meses / 62 cm', '3-6 mois / 62 cm', '3-6 months / 62 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
616, '6-9 meses / 68 cm', '6-9 mois / 68 cm', '6-9 months / 68 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
617, '9-12 meses / 74 cm', '9-12 mois / 74 cm', '9-12 months / 74 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
618, '12-18 meses / 80 cm', '12-18 mois / 80 cm', '12-18 months / 80 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
619, '18-24 meses / 86 cm', '18-24 mois / 86 cm', '18-24 months / 86 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
622, '24-36 meses / 92 cm', '24-36 mois / 92 cm', '24-36 months / 92 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
1567, '3 años / 98 cm', '3 ans / 98 cm', '3 years / 98-103 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
623, '4 años / 104 cm', '4 ans / 104 cm', '4 years / 104 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
624, '5 años / 110 cm', '5 ans / 110 cm', '5 years / 110 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
625, '6 años / 116 cm', '6 ans / 116 cm', '6 years / 116 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
626, '7 años / 122 cm', '7 ans / 122 cm', '7 years / 122 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
627, '8 años / 128 cm', '8 ans / 128 cm', '8 years / 128 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
628, '9 años / 134 cm', '9 ans / 134 cm', '9 years / 134 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
629, '10 años / 140 cm', '10 ans / 140 cm', '10 years / 140 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
630, '11 años / 146 cm', '11 ans / 146 cm', '11 years / 146 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
631, '12 años / 152 cm', '12 ans / 152 cm', '12 years / 152 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
632, '13 años / 158 cm', '13 ans / 158 cm', '13 years / 158 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
633, '14 años / 164 cm', '14 ans / 164 cm', '14 years / 164 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
634, '15 años / 170 cm', '15 ans / 170 cm', '15 years / 170 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
635, '16 años /176 cm', '16 ans / 176 cm', '16 years / 176 cm', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
620, 'Talla única', 'Taille unique', 'One size', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
1568, 'XS', 'XS', 'XS', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
1569, 'S', 'S', 'S', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
1570, 'M', 'M', 'M', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
1571, 'L', 'L', 'L', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
1572, 'XL', 'XL', 'XL', 'Ropa', 'Tailles enfants et bébés', 'Babies and children sizes',1193
657, '15 o inferior', '15 et moins', '15 and smaller', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
585, '16', '16', '0 baby / EU 16', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
586, '17', '17', '1 baby / EU 17', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
587, '18', '18', '2 baby / EU 18', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
588, '19', '19', '3 baby / EU 19', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
589, '20', '20', '3.5 baby / EU 20', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
590, '21', '21', '4 baby / EU 21', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
591, '22', '22', '5 baby / EU 22', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
592, '23', '23', '6 baby / EU 23', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
593, '24', '24', '7 child / EU 24', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
594, '25', '25', '8 child / EU 25', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
595, '26', '26', '8.5 child / EU 26', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
596, '27', '27', '9 child / EU 27', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
597, '28', '28', '10 child / EU 28', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
598, '29', '29', '11 child / EU 29', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
599, '30', '30', '11.5 child / EU 30', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
600, '31', '31', '12 child / EU 31', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
601, '32', '32', '13 child / EU 32', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
602, '33', '33', '1 junior / EU 33', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
603, '34', '34', '2 junior / EU 34', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
604, '35', '35', '2.5 junior / EU 35', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
605, '36', '36', '3 junior / EU 36', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
606, '37', '37', '4 junior / EU 37', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
607, '38', '38', '5 junior / EU 38', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
608, '39', '39', '6 junior / EU 39', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
609, '40', '40', '6.5 junior / EU 40', 'Calzado', 'Tailles de chaussures pour enfants', 'Children''s shoe sizes',1193
533, 'max 2 kg', 'jusqu''à 2 kg', 'No. 0 (up to 2 kg)', 'Pañales', 'Couches', 'Diapers',1193
534, '2-5 kg', '2-5 kg', 'No. 1 (2-5 kg)', 'Pañales', 'Couches', 'Diapers',1193
535, '3-6kg', '3-6kg', 'No.2 (3-6 kg)', 'Pañales', 'Couches', 'Diapers',1193
536, '4-9 kg', '4-9 kg', 'No. 3 (4-9 kg)', 'Pañales', 'Couches', 'Diapers',1193
537, '7-18 kg', '7-18 kg', 'No. 4 (7-18 kg)', 'Pañales', 'Couches', 'Diapers',1193
538, '9-20 kg', '9-20 kg', 'No. 4+ (9-20 kg)', 'Pañales', 'Couches', 'Diapers',1193
539, '11-25 kg', '11-25 kg', 'No. 5 (11-25 kg)', 'Pañales', 'Couches', 'Diapers',1193
540, '13-27 kg', '13-27 kg', 'No. 5 + (13-27 kg)', 'Pañales', 'Couches', 'Diapers',1193
541, '16+ kg', '16 kg et +', 'No. 6 (16+ kg)', 'Pañales', 'Couches', 'Diapers',1193
542, '0-13 kg', '0-13 kg', '0-13 kg / 0 - 29 lb', 'Sillas de coche', 'Sièges auto bébé', 'Baby car seats',1193
544, '9-18 kg', '9-18 kg', '9-18 kg / 20 - 40 lb', 'Sillas de coche', 'Sièges auto bébé', 'Baby car seats',1193
547, '15-36 kg', '15-36 kg', '15-36 kg / 33 lb - 5 st 9 lb', 'Sillas de coche', 'Sièges auto bébé', 'Baby car seats',1193
1188, '22-36 kg', '22-36 kg', '22-36 kg / 48 lb - 5 st 9 lb', 'Sillas de coche', 'Sièges auto bébé', 'Baby car seats',1193
662, 'Otros', 'Autre', 'Other', 'Sillas de coche', 'Sièges auto bébé', 'Baby car seats',1193
577, 'Cualquier', 'Peu importe', 'Any', 'Edades', 'Âge enfant', 'Baby age',1193
565, 'Desde 0 meses', '0 mois', 'From 0 months', 'Edades', 'Âge enfant', 'Baby age',1193
566, 'Desde 3 meses', '3 mois', 'From 3 months', 'Edades', 'Âge enfant', 'Baby age',1193
567, 'Desde 6 meses', '6 mois', 'From 6 months', 'Edades', 'Âge enfant', 'Baby age',1193
568, 'Desde 9 meses', '9 mois', 'From 9 months', 'Edades', 'Âge enfant', 'Baby age',1193
569, 'Desde 1 año', '12 mois', 'From 1 year', 'Edades', 'Âge enfant', 'Baby age',1193
570, 'Desde 1 año y medio', '18 mois', 'From 1.5 years', 'Edades', 'Âge enfant', 'Baby age',1193
571, 'Desde 2 años', '2 ans', 'From 2 years', 'Edades', 'Âge enfant', 'Baby age',1193
572, 'Desde 3 años', '3 ans', 'From 3 years', 'Edades', 'Âge enfant', 'Baby age',1193
1311, 'Desde 4 años', '4 ans', 'From 4 years', 'Edades', 'Âge enfant', 'Baby age',1193
573, 'Desde 5 años', '5 ans', 'From 5 years', 'Edades', 'Âge enfant', 'Baby age',1193
574, 'Desde 6 años', '6 ans', 'From 6 years', 'Edades', 'Âge enfant', 'Baby age',1193
575, 'Desde 7 años', '7 ans', 'From 7 years', 'Edades', 'Âge enfant', 'Baby age',1193
576, 'Desde 8 años', '8 ans', 'From 8 years', 'Edades', 'Âge enfant', 'Baby age',1193
928, 'Prematuro', 'Prématurés', 'Preemies', '30 cm', '30 cm', '12 inches',1193
929, '0-3 meses', '0-3 mois', '0-3 months', '38 cm', '38 cm', '15 inches', 1193
930, '3-6 meses', '3-6 mois', '3-6 months', 42 cm', 42 cm', '17 inches', 1193
931, '6-12 meses', '6-12 mois', '6-12 months', '47 cm', 47 cm',' '18 inches', 1193
932, '1-2 años', '1-2 ans', '1-2 years', '49 cm', '49 cm', '19 inches',1193
933, '2-3 años', '2-3 ans', '2-3 years', '51 cm', '51 cm', '20 inches',1193
934, '3-5 años', '3-5 ans', '3-5 years', '53 cm', '53 cm', '21 inches',1193
935, '5-8 años', '5-8 ans', '5-8 years', '54 cm', '54 cm', '22 inches',1193
936, '8-12 años', '8-12 ans', '8-12 years',' 56 cm', '56 cm', '23 inches',1193
937, 'A partir de 12 años >56 cm', '12 ans et plus > 56 cm', '12 and older > 23 inches', 'Gorros', 'Chapeaux enfant', 'Kids hats',1193
1189, 'Talla única', 'Universel', 'One size', 'Gorros', 'Chapeaux enfant', 'Kids hats',1193
1496, '45 cm', '45 cm','18 inch', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1497, '50 cm', '50 cm','20 inch', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1498, '55 cm', '55 cm','22 inch', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1499, '60 cm', '60 cm','24 inch', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1500, '65 cm', '65 cm','26 inch', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1501, '70 cm', '70 cm','28 inch', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1663, '75 cm', '75 cm', '29.5 inch', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1502, '80 cm', '80 cm','31 inch', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1503, '85 cm', '85 cm','33 inch', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1504, 'Ajustable', 'Ajustable', 'Adjustable', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1505, 'Talla única', 'Taille unique', 'One size', 'Cinturones de niño', 'Ceintures enfant', 'Kids'' belts',1193
1507, '13-15', '13-15', 'Tiny Baby', 'Calcetines de niño', 'Chaussettes enfant', 'Kids'' socks',1193
1508, '16-18', '16-18', '0-2 Baby', 'Calcetines de niño', 'Chaussettes enfant', 'Kids'' socks',1193
1509, '19-21', '19-21', '3-4 Baby', 'Calcetines de niño', 'Chaussettes enfant', 'Kids'' socks',1193
1510, '22-24', '22-24', '5 Baby - 7 Child', 'Calcetines de niño', 'Chaussettes enfant', 'Kids'' socks',1193
1511, '25-27', '25-27', '8-9 Child', 'Calcetines de niño', 'Chaussettes enfant', 'Kids'' socks',1193
1512, '28-30', '28-30', '10-11.5 Child', 'Calcetines de niño', 'Chaussettes enfant', 'Kids'' socks',1193
1513, '31-33', '31-33', '12 Child - 1 Junior', 'Calcetines de niño', 'Chaussettes enfant', 'Kids'' socks',1193
1514, '34-36', '34-36', '2-3 Junior', 'Calcetines de niño', 'Chaussettes enfant', 'Kids'' socks',1193
1515, '37-39', '37-39', '4-6 Junior', 'Calcetines de niño', 'Chaussettes enfant', 'Kids'' socks',1193
1516, '40-42', '40-42', '7-10 Junior', 'Calcetines de niño', 'Chaussettes enfant', 'Kids'' socks',1193
1489, 'Moisés', 'Berceau', 'Moses basket / crib', 'Ropa de cama de bebé', 'Literie bébé', 'Baby bedding',1193
1490, 'Minicuna', 'Lit nouveau né', 'Mini cot / Compact cot', 'Ropa de cama de bebé', 'Literie bébé', 'Baby bedding',1193
1566, 'Cuna de viaje', 'Lit de voyage bébé', 'Travel cot', 'Ropa de cama de bebé', 'Literie bébé', 'Baby bedding',1193
1491, 'Cuna', 'Lit bébé', 'Cot / Toddler bed', 'Ropa de cama de bebé', 'Literie bébé', 'Baby bedding',1193
1352, 'XS', 'XS', 'XS', 'Artículos para perros', 'Articles pour chiens', 'Dog items',2093
1353, 'S', 'S', 'S', 'Artículos para perros', 'Articles pour chiens', 'Dog items',2093
1354, 'M', 'M', 'M', 'Artículos para perros', 'Articles pour chiens', 'Dog items',2093
1355, 'L', 'L', 'L', 'Artículos para perros', 'Articles pour chiens', 'Dog items',2093
1356, 'XL', 'XL', 'XL', 'Artículos para perros', 'Articles pour chiens', 'Dog items',2093
1357, 'XXL', 'XXL', 'XXL', 'Artículos para perros', 'Articles pour chiens', 'Dog items',2093
1363, 'Otro', 'Autre', 'Other', 'Artículos para perros', 'Articles pour chiens', 'Dog items',2093
1365, 'Talla única', 'Taille unique', 'One size', 'Artículos para perros', 'Articles pour chiens', 'Dog items',2093
1226, 'XXXS / 30 / 2', 'XXXS / 30 / 2', 'XXXS / 2', 'Ropa', 'Tailles', 'Women''s items',1904
102, 'XXS / 32 / 4', 'XXS / 32 / 4', 'XXS / 4', 'Ropa', 'Tailles', 'Women''s items',1904
2, 'XS / 34 / 6', 'XS / 34 / 6', 'XS / 6', 'Ropa', 'Tailles', 'Women''s items',1904
3, 'S / 36 / 8', 'S / 36 / 8', 'S / 8', 'Ropa', 'Tailles', 'Women''s items',1904
4, 'M / 38 / 10', 'M / 38 / 10', 'M / 10', 'Ropa', 'Tailles', 'Women''s items',1904
5, 'L / 40 / 12', 'L / 40 / 12', 'L / 12', 'Ropa', 'Tailles', 'Women''s items',1904
6, 'XL / 42 / 14', 'XL / 42 / 14', 'XL / 14', 'Ropa', 'Tailles', 'Women''s items',1904
7, 'XXL / 44 / 16', 'XXL / 44 / 16', 'XXL / 16', 'Ropa', 'Tailles', 'Women''s items',1904
310, 'XXXL / 46 / 18', 'XXXL / 46 / 18', 'XXXL / 18', 'Ropa', 'Tailles', 'Women''s items',1904
311, '4XL / 48 / 20', '4XL / 48 / 20', '4XL / 20', 'Ropa', 'Tailles', 'Women''s items',1904
312, '5XL / 50 / 22', '5XL / 50 / 22', '5XL / 22', 'Ropa', 'Tailles', 'Women''s items',1904
1227, '6XL / 52 / 24', '6XL / 52 / 24', '6XL / 24', 'Ropa', 'Tailles', 'Women''s items',1904
1228, '7XL / 54 / 26', '7XL / 54 / 26', '7XL / 26', 'Ropa', 'Tailles', 'Women''s items',1904
1229, '8XL / 56 / 28', '8XL / 56 / 28', '8XL / 28', 'Ropa', 'Tailles', 'Women''s items',1904
1230, '9XL / 58 / 30', '9XL / 58 / 30', '9XL / 30', 'Ropa', 'Tailles', 'Women''s items',1904
90, 'Talla única', 'Taille unique', 'One size', 'Ropa', 'Tailles', 'Women''s items',1904
97, 'Otros', 'Autre', 'Other', 'Ropa', 'Tailles', 'Women''s items',1904
1364, '34', '34', '1', 'Calzado', 'Chaussures', 'Footwear',1904
1580, '34.5', '34.5', '1.5', 'Calzado', 'Chaussures', 'Footwear',1904
55, '35', '35', '2', 'Calzado', 'Chaussures', 'Footwear',1904
1195, '35.5', '35.5', '2.5', 'Calzado', 'Chaussures', 'Footwear',1904
56, '36', '36', '3', 'Calzado', 'Chaussures', 'Footwear',1904
1196, '36.5', '36.5', '3.5', 'Calzado', 'Chaussures', 'Footwear',1904
57, '37', '37', '4', 'Calzado', 'Chaussures', 'Footwear',1904
1197, '37.5', '37.5', '4.5', 'Calzado', 'Chaussures', 'Footwear',1904
58, '38', '38', '5', 'Calzado', 'Chaussures', 'Footwear',1904
1198, '38.5', '38.5', '5.5', 'Calzado', 'Chaussures', 'Footwear',1904
59, '39', '39', '6', 'Calzado', 'Chaussures', 'Footwear',1904
1199, '39.5', '39.5', '6.5', 'Calzado', 'Chaussures', 'Footwear',1904
60, '40', '40', '7', 'Calzado', 'Chaussures', 'Footwear',1904
1200, '40.5', '40.5', '7.5', 'Calzado', 'Chaussures', 'Footwear',1904
61, '41', '41', '8', 'Calzado', 'Chaussures', 'Footwear',1904
1201, '41.5', '41.5', '8.5', 'Calzado', 'Chaussures', 'Footwear',1904
62, '42', '42', '9', 'Calzado', 'Chaussures', 'Footwear',1904
1579, '42.5', '42.5', '9.5', 'Calzado', 'Chaussures', 'Footwear',1904
63, '43', '43', '10', 'Calzado', 'Chaussures', 'Footwear',1904
1573, '43.5', '43.5', '10.5', 'Calzado', 'Chaussures', 'Footwear',1904
1574, '44', '44', '11', 'Calzado', 'Chaussures', 'Footwear',1904
1575, '44.5', '44.5', '44.5', 'Calzado', 'Chaussures', 'Footwear',1904
1576, '45', '45', '12', 'Calzado', 'Chaussures', 'Footwear',1904
1577, '45.5', '45.5', '12.5', 'Calzado', 'Chaussures', 'Footwear',1904
1578, '46', '46', '13', 'Calzado', 'Chaussures', 'Footwear',1904
94, 'Talla única', 'Taille unique', 'One size', 'Calzado', 'Chaussures', 'Footwear',1904
99, 'Otra', 'Autre', 'Other', 'Calzado', 'Chaussures', 'Footwear',1904
1247, '80A y AA', '80A et AA', '30A & AA', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1248, '80B', '80B', '30B', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1249, '80C', '80C', '30C', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1250, '80D', '80D', '30D', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1251, '80E', '80E', '30DD', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1252, '80F', '80F', '30E', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1317, '80G', '80G', '30F', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1318, '80H', '80H', '30FF', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1558, '80I', '80I', '30G', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1319, '80J', '80J', '30GG', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1253, '85A y AA', '85A et AA', '32A & AA', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1254, '85B', '85B', '32B', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1255, '85C', '85C', '32C', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1256, '85D', '85D', '32D', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1257, '85E', '85E', '32DD', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1258, '85F', '85F', '32E', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1320, '85G', '85G', '32F', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1321, '85H', '85H', '32FF', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1559, '85I', '85I', '32G', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1322, '85J', '85J', '32GG', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1259, '90A y AA', '90A et AA', '34A & AA', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1260, '90B', '90B', '34B', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1261, '90C', '90C', '34C', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1262, '90D', '90D', '34D', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1263, '90E', '90E', '34DD', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1264, '90F', '90F', '34E', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1323, '90G', '90G', '34F', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1324, '90H', '90H', '34FF', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1325, '90J', '90J', '34GG', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1265, '95A y AA', '95A et AA', '36A & AA', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1266, '95B', '95B', '36B', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1267, '95C', '95C', '36C', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1268, '95D', '95D', '36D', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1269, '95E', '95E', '36DD', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1270, '95F', '95F', '36E', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1271, '95G', '95G', '36F', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1272, '95H', '95H', '36FF', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1561, '95I', '95I', '36G', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1273, '95J', '95J', '36GG', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1274, '100A y AA', '100A et AA', '38A & AA', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1275, '100B', '100B', '38B', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1276, '100C', '100C', '38C', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1277, '100D', '100D', '38D', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1278, '100E', '100E', '38DD', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1279, '100F', '100F', '38E', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1280, '100G', '100G', '38F', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1281, '100H', '100H', '38FF', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1562, '100I', '100I', '38G', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1282, '100J', '100J', '38GG', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1283, '105A y AA', '105A et AA', '40A & AA', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1284, '105B', '105B', '40B', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1285, '105C', '105C', '40C', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1286, '105D', '105D', '40D', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1287, '105E', '105E', '40DD', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1288, '105F', '105F', '40E', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1289, '106G', '106G', '40F', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1290, '105H', '105H', '40FF', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1563, '105I', '105I', '40G', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1291, '105J', '105J', '40GG', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1292, '110A y AA', '110A et AA', '42A & AA', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1293, '110B', '110B', '42B', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1294, '110C', '110C', '42C', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1295, '110D', '110D', '42D', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1296, '110E', '110E', '42DD', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1297, '110F', '110F', '42E', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1298, '110G', '110G', '42F', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1299, '110H', '110H', '42FF', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1564, '110I', '110I', '42G', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1300, '110J', '110J', '42GG', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1301, '115A y AA', '115A et AA', '44A & AA', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1302, '115B', '115B', '44B', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1303, '115C', '115C', '44C', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1304, '115D', '115D', '44D', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1305, '115E', '115E', '44DD', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1306, '115F', '115F', '44E', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1307, '115G', '115G', '44F', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1308, '115H', '115H', '44FF', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1565, '115I', '115I', '44G', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1309, '115J', '115J', '44GG', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1310, 'Otra', 'Autre', 'Other', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1394, 'XXS', 'XXS', 'XXS', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1395, 'XS', 'XS', 'XS', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1396, 'S', 'S', 'S', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1397, 'M', 'M', 'M', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1398, 'L', 'L', 'L', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1399, 'XL', 'XL', 'XL', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1400, 'XXL', 'XXL', 'XXL', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1401, 'XXXL', 'XXXL', 'XXXL', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1402, '4XL', '4XL', '4XL', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1403, '5XL', '5XL', '5XL', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1404, '6XL', '6XL', '6XL', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1405, '7XL', '7XL', '7XL', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1406, '8XL', '8XL', '8XL', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
1407, '9XL', '9XL', '9XL', 'Sujetadores', 'Soutiens-gorge', 'Bras',1904
578, 'XS', 'XS', 'XS', 'Fajas premamá', 'Ceintures de soutien maternité', 'Maternity support belts',1904
580, 'S', 'S', 'S', 'Fajas premamá', 'Ceintures de soutien maternité', 'Maternity support belts',1904
581, 'M', 'M', 'M', 'Fajas premamá', 'Ceintures de soutien maternité', 'Maternity support belts',1904
582, 'L', 'L', 'L', 'Fajas premamá', 'Ceintures de soutien maternité', 'Maternity support belts',1904
583, 'XL', 'XL', 'XL', 'Fajas premamá', 'Ceintures de soutien maternité', 'Maternity support belts',1904
584, 'XXL', 'XXL', 'XXL', 'Fajas premamá', 'Ceintures de soutien maternité', 'Maternity support belts',1904
1557, 'Talla única', 'Taille unique', 'One size', 'Fajas premamá', 'Ceintures de soutien maternité', 'Maternity support belts',1904
664, 'Otros', 'Autre', 'Other', 'Fajas premamá', 'Ceintures de soutien maternité', 'Maternity support belts',1904
1446, '65 cm', '65 cm', '26 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1447, '70 cm', '70 cm', '28 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1448, '75 cm', '75 cm', '30 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1449, '80 cm', '80 cm', '32 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1450, '85 cm', '85 cm', '34 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1451, '90 cm', '90 cm', '36 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1452, '95 cm', '95 cm', '38 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1453, '100 cm', '100 cm', '40 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1454, '105 cm', '105 cm', '42 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1455, '110 cm', '110 cm', '44 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1456, '115 cm', '115 cm', '46 inches', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1582, 'Ajustable', 'Ajustable', 'Adjustable', 'Cinturones de mujer', 'Ceintures femme', 'Women''s Belts',1904
1517, 'S | 35–38', 'S | 35–38', 'S | 2-5.5', 'Calcetines de mujer', 'Chaussettes femme', 'Women''s socks',1904
1518, 'M | 39–42', 'M | 39–42', 'M | 6-9.5', 'Calcetines de mujer', 'Chaussettes femme', 'Women''s socks',1904
1519, 'L | 43–46', 'L | 43–46', 'L | 10-12.5', 'Calcetines de mujer', 'Chaussettes femme', 'Women''s socks',1904
1520, 'XL | 47–51', 'XL | 47–51', 'XL | 13-15', 'Calcetines de mujer', 'Chaussettes femme', 'Women''s socks',1904
1521, 'Talla única', 'Taille unique', 'One size', 'Calcetines de mujer', 'Chaussettes femme', 'Women''s socks',1904
