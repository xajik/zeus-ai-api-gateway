# PSQL 

 * Enter: "psql -U postgres"
 * Change password: "ALTER USER postgres WITH PASSWORD 'postgres';"

# PG Admin 

* host: db
* user name: postgres
* password: postgres

# Notes 

* Make sure all folders created by users not Docker 

# Migration 

* Find container: 
    * ```docker ps``` // find ID
* Attach shell: 
    * ```docker exec -it <container_name_or_id> /bin/bash```
* Enter folder with <b>scripts<b>: 
    * ```cd scripts```
* Execute migration: 
    * ```./wait-for-db.sh && ./run-migrations-up.sh```
* Migrate down in case of error: 
    * ```./run-migrations-down.sh```