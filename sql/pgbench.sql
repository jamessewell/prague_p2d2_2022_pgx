select max_timed_pgx(instant, temperature) from weather_data; 
select max(temperature) from weather_data; 
select max_timed(instant, temperature) from weather_data; 
SELECT instant, temperature
FROM weather_data
WHERE temperature = (
  SELECT max(temperature)
  FROM weather_data
)
ORDER BY instant
LIMIT 1;
