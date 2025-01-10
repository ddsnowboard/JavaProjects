with opportunity_by_id AS (
    with card_values AS (
        SELECT rowid,
        data->'$.opportunity[0][1]' AS left,
        data->'$.opportunity[1][1]' AS right
        from logs
    )
    select rowid,
    iif(json_type(left) = 'object', left->>'$.Number', case left->>'$' when 'LowAce' then 1 when 'Jack' then 11 when 'Queen' then 12 when 'King' then 13 when 'HiAce' then 14 end) AS left_value,
    iif(json_type(right) = 'object', right->>'$.Number', case right->>'$' when 'LowAce' then 1 when 'Jack' then 11 when 'Queen' then 12 when 'King' then 13 when 'HiAce' then 14 end) AS right_value
    from card_values
),
joined AS (
    select l.*, o.left_value, o.right_value
    from logs l
    inner join opportunity_by_id o ON o.rowid = l.rowid
),
q1 AS (
    with q AS (
        select left_value, right_value,
        coalesce(sum(iif(source like 'Basic%' AND data->>'$.outcome' = 'Inside', 1.0, 0.0)), 0) / sum(iif(source like 'Basic%', 1.0, 0.0)) AS basic_success_rate,
        coalesce(sum(iif(source like 'Middle%' AND data->>'$.outcome' = 'Inside', 1.0, 0.0)), 0) / sum(iif(source like 'Middle%', 1.0, 0.0)) AS middle_success_rate
        from joined
        where data->>'$.outcome' IS NOT NULL
        group by 1, 2 order by 1, 2
    )
    select *, middle_success_rate - basic_success_rate AS improvement
    from q
    order by abs(improvement) desc nulls first
),
three_jacks AS (
    select source, data->>'$.outcome' AS outcome, count(*)
    from joined
    where (3, 11) IN ((left_value, right_value), (right_value, left_value))
    group by 1, 2
)
select * from three_jacks;
