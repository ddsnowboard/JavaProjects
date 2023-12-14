create temp view things_owned_by_person AS select distinct o.customerid, oi.sku from orders_items oi inner join orders o on o.orderid = oi.orderid;
create temp view collectables AS select sku from products where sku like 'COL%';
create temp view people_owning_all_collectables AS select customerid from things_owned_by_person top WHERE NOT EXISTS(select 1 from collectables c where c.sku NOT IN (select sku from things_owned_by_person top2 where top2.customerid = top.customerid));

select customerid, name, phone 
from customers where customerid IN (Select * from people_owning_all_collectables);
