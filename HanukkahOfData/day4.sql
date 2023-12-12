CREATE TEMP VIEW pastry_orders AS SELECT * from orders o where o.orderid IN (select orderid from orders_items where sku like 'BKY%');
create temp view early_risers AS select customerid, count(*) from pastry_orders o where not exists(select 1 from pastry_orders oo where date(oo.shipped) = date(o.shipped) and oo.shipped < o.shipped) group by 1 order by 2 desc limit 10;
-- This returns a list and the first woman on it is the answer
select * from customers where customerid IN (select customerid from early_risers);
