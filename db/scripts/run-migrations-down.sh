#!/bin/bash
# run-migrations-down.sh

# Set the PostgreSQL connection details
PG_HOST=localhost
PG_PORT=5432
PG_USER=${POSTGRES_USER}
PG_PASSWORD=${POSTGRES_PASSWORD}
PG_DB=${POSTGRES_DB}
export PGPASSWORD=$PG_PASSWORD

# Directory containing migration scripts
MIGRATIONS_DIR=../migrations

# Run each ".down.sql" migration script found in $MIGRATIONS_DIR
# Files are sorted in reverse order based on their timestamp prefix
for migration in $(ls $MIGRATIONS_DIR/*.down.sql | sort -nr); do
    echo "Reverting migration: $migration"
    psql -h $PG_HOST -p $PG_PORT -U $PG_USER -d $PG_DB -f $migration
    if [ $? -ne 0 ]; then
        echo "Error reverting migration: $migration"
        exit 1
    fi
done

echo "All down migrations applied successfully."
