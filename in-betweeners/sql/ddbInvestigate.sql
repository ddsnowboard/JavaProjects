with payout_by_outcome (outcome, payout) AS (
    select 'Double', -2
    UNION
    select 'Inside', 1
    UNION
    select 'Outside', -1
),
opportunity_by_id AS (
    with card_values AS (
        SELECT rowid,
        data->'$.opportunity[0][1]' AS l,
        data->'$.opportunity[1][1]' AS r
        from logs
    ),
    as_number AS (
        select rowid,
        if(json_type(l) = 'OBJECT', l.Number::INTEGER, case l->>'$' when 'LowAce' then 1 when 'Jack' then 11 when 'Queen' then 12 when 'King' then 13 when 'HiAce' then 14 end) AS left_value,
        if(json_type(r) = 'OBJECT', r.Number::INTEGER, case r->>'$' when 'LowAce' then 1 when 'Jack' then 11 when 'Queen' then 12 when 'King' then 13 when 'HiAce' then 14 end) AS right_value
        from card_values
    )
    select rowid, least(left_value, right_value) AS left_value, greatest(left_value, right_value) AS right_value
    from as_number
),
joined AS (
    select l.source, message, l.data->>'$.outcome' AS outcome, l.data, o.left_value, o.right_value
    from logs l
    inner join opportunity_by_id o ON o.rowid = l.rowid
),
q1 AS (
    with q AS (
        select left_value, right_value,
        coalesce(sum(if(source like 'Basic%' AND data->>'$.outcome' = 'Inside', 1.0, 0.0)), 0) / sum(if(source like 'Basic%', 1.0, 0.0)) AS basic_success_rate,
        coalesce(sum(if(source like 'Middle%' AND data->>'$.outcome' = 'Inside', 1.0, 0.0)), 0) / sum(if(source like 'Middle%', 1.0, 0.0)) AS middle_success_rate
        from joined
        where data->>'$.outcome' IS NOT NULL
        group by 1, 2 order by 1, 2
    )
    select *, middle_success_rate - basic_success_rate AS improvement
    from q
    order by abs(improvement) desc nulls first
),
profit_by_opp AS (
    with q AS (
        select left_value, right_value,
        coalesce(sum(if(source like 'Basic%', case data->>'$.outcome' when 'Inside' then 1.0 when 'Outside' then -1.0 when 'Double' then -2.0 else 0.0 END, 0.0)), 0) / sum(if(source like 'Basic%', 1.0, 0.0)) AS basic_profit,
        coalesce(sum(if(source like 'Middle%', case data->>'$.outcome' when 'Inside' then 1.0 when 'Outside' then -1.0 when 'Double' then -2.0 else 0.0 END, 0.0)), 0) / sum(if(source like 'Middle%', 1.0, 0.0)) AS middle_profit
        from joined
        group by 1, 2 order by 1, 2
    )
    select *, middle_profit - basic_profit AS improvement
    from q
    order by abs(improvement) desc nulls first
),
three_jacks AS (
    select source, data->>'$.outcome' AS outcome, count(*)
    from joined
    where (3, 11) IN ((left_value, right_value), (right_value, left_value))
    group by 1, 2
)
select *
from profit_by_opp
order by improvement limit 20;
