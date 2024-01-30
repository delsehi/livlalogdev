
-- SELECT AVG(epley(weight, reps)) as avg_rep_max FROM lifts 
-- -- where date > '2024-01-01' and date < '2024-01-10'
-- where lift = 'DL'

-- SELECT weight, reps,rpe, epley(weight, reps) as maxrep FROM lifts where lift = 'DL' and max_rep = true;
-- select * from lifts where rpe is not null


SELECT *, EPLEY(weight, reps) as maxrep FROM lifts;