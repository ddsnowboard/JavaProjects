create temp view staten_islanders AS select * from customers where citystatezip like 'Staten Island%';
create temp view senior_cat_food AS select * from products where sku like 'PET%' AND desc like '%cat%' and desc like '%senior%';
create temp view senior_cat_food_buyers AS select o.customerid, count(*) AS n_orders FROM orders_items oi 
inner join orders o on o.orderid = oi.orderid 
where oi.sku IN (Select sku from senior_cat_food) group by 1;

select c.name, c.phone, scf.n_orders from customers c 
inner join senior_cat_food_buyers scf on scf.customerid = c.customerid 
where c.customerid IN (select customerid from staten_islanders)
order by scf.n_orders desc limit 30;
