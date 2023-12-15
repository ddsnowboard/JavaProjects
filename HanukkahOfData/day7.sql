-- Pick the first man's name. This guy does a lot of business for there to be so many options here. It was 838-335-7157.
-- The speedrun answer is 516-544-4187. There are so many options. How am I to know which one?
create temp view girls_id AS select 8884 AS customerid;
create temp view girls_orders AS select * from orders where customerid IN (Select * from girls_id);
create temp view girls_colored_items AS select p.sku, p.desc, oi.orderid from products p 
inner join orders_items oi on oi.sku = p.sku
where oi.orderid IN (select orderid from girls_orders)
and p.desc like '%(%';

create temp table girls_items_without_color_by_date AS select sku, substr(desc, 0, instr(desc, '(') - 1) AS desc_no_color, date(o.shipped) AS date from girls_colored_items gci
    inner join orders o on o.orderid = gci.orderid;

create temp view buyers_of_same_item_different_color_same_date AS select o.customerid, p.sku
FROM orders o
inner join orders_items oi on oi.orderid = o.orderid
inner join products p on p.sku = oi.sku
inner join girls_items_without_color_by_date giwc ON giwc.desc_no_color = substr(p.desc, 0, instr(p.desc, '(') - 1)
WHERE date(o.shipped) = date(giwc.date) and giwc.sku <> oi.sku;

select distinct c.customerid, c.phone, c.name, p.desc
from buyers_of_same_item_different_color_same_date b
inner join customers c on c.customerid = b.customerid
inner join products p on p.sku = b.sku;
