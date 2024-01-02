#!/bin/bash
# run-migrations.sh

# Set the PostgreSQL connection details
PG_HOST=localhost
PG_PORT=5432
PG_USER=${POSTGRES_USER}
PG_PASSWORD=${POSTGRES_PASSWORD}
PG_DB=${POSTGRES_DB}
export PGPASSWORD=$PG_PASSWORD

# Directory containing migration scripts
MIGRATIONS_DIR=../migrations

# Function to run a single migration file
run_migration() {
    migration=$1
    echo "Applying migration: $migration"
    psql -h $PG_HOST -p $PG_PORT -U $PG_USER -d $PG_DB -f $migration

    # Check if migration failed
    if [ $? -ne 0 ]; then
        echo "Error applying migration: $migration"
        # Attempt to run the corresponding down migration
        down_migration="${migration/.up.sql/.down.sql}"
        if [ -f $down_migration ]; then
            echo "Attempting to revert using: $down_migration"
            psql -h $PG_HOST -p $PG_PORT -U $PG_USER -d $PG_DB -f $down_migration
        else
            echo "No corresponding down migration found for: $down_migration"
        fi
        exit 1
    fi
}

echo "Migrations directory: $MIGRATIONS_DIR"
for migration in $(ls $MIGRATIONS_DIR/*.up.sql | sort -n); do
    echo "Executing migration: $migration"
    run_migration $migration
done

echo "All migrations applied successfully."
