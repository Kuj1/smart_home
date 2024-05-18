- If you use Postgres - good, create db and create tables in it as below
- If not - its bad. Use schema below as example for you sql system or install Postgres.
- Before you start, check .env and correct "DATABASE_URL"
- If you want run examples in examples folder (-_-), you need to correct the values of the variables to your own, based on what you have in the table.

*Create table home*
```sql
CREATE TABLE IF NOT EXISTS houses (
    id serial PRIMARY KEY,
    house_name varchar(45) NOT NULL
);
```


*Create table rooms*
```sql
CREATE TABLE IF NOT EXISTS rooms (
    id serial PRIMARY KEY,
    house_id int,
    room_name varchar(45) NOT NULL,
    FOREIGN KEY (house_id) REFERENCES houses (id) ON DELETE CASCADE
);
```

*Create table smart_device*
```sql
CREATE TABLE IF NOT EXISTS smart_devices (
    id serial PRIMARY KEY,
    house_id int,
    room_id int, 
    vendor_id varchar(50) NOT NULL,
    device_name varchar(45) NOT NULL,
    is_on bool NOT NULL,
    voltage varchar(3) NOT NULL,
    power varchar(3) NOT NULL,
    FOREIGN KEY (house_id) REFERENCES houses (id) ON DELETE CASCADE, 
    FOREIGN KEY (room_id) REFERENCES rooms (id) ON DELETE CASCADE   
);
```
