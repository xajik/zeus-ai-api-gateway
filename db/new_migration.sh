#!/bin/bash

# Check if an argument is provided
if [ $# -eq 0 ]; then
  echo "Usage: $0 <migration_name>"
  exit 1
fi

# Get the current timestamp
timestamp=$(date +"%Y%m%d%H%M%S")

# Get the current date in DD MM YYYY format
current_date=$(date +"%d %m %Y")

# Migration name provided as the first argument
migration_name=$1

# Create the filenames
up_filename="migrations/${timestamp}_${migration_name}.up.sql"
down_filename="migrations/${timestamp}_${migration_name}.down.sql"

# Check if the migrations directory exists
if [ ! -d "migrations" ]; then
  mkdir migrations
fi

# Create the up migration file
echo "-- ${current_date}: $migration_name Up Migration" > "$up_filename"

# Create the down migration file
echo "-- ${current_date}: $migration_name Down Migration" > "$down_filename"

echo "New migration files created:"
echo "  - $up_filename"
echo "  - $down_filename"
