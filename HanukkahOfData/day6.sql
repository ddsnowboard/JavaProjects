create temp view profit_by_order AS select oi.orderid, sum(qty * unit_price - qty * wholesale_cost) AS profit
FROM orders_items oi 
inner join products p on p.sku = oi.sku
group by 1;

create temp view profit_by_customer AS select c.customerid, sum(profit) AS total_profit
FROM customers c 
inner join orders o on o.customerid = c.customerid
inner join profit_by_order po on po.orderid = o.orderid
group by 1;

select c.customerid, c.name, c.phone, pc.total_profit
from customers c
inner join profit_by_customer pc on pc.customerid = c.customerid
order by total_profit
limit 30;
