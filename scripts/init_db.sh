#!/usr/bin/env bash
set -x
set -eo pipefail

#Let's check that both psql and sqlx-cli are installed
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: `psql` is not installed."
    exit 1
fi 

if ! [ -x "$(command -v sqlx)" ]; then 
    echo >&2 "Error: `sqlx` is not installed."
    echo >&2 "Use:"
    echo >&2 "   cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
    echo >&2 "to install it."
    exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
# Change user and password back to generic postgres and password before commiting this file.
DB_USER=${POSTGRES_USER:=fito305}

# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=Allapattah123456}"

# Check if a custom database name has been set, other wise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"

# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then 
# Launch postgres using docker
docker run \
-e POSTGRES_USER=${DB_USER} \
-e POSTGRES_PASSWORD=${DB_PASSWORD} \
-e POSTGRES_DB=${DB_NAME} \
-p "${DB_PORT}":5432 \
-d postgres \
postgres -N 1000
# ^ Increased maximun number of connections for testing purposes
fi

# Keep pinging Postgres until it's ready to accept commands 
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; 
do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - rinning migrations now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create 
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"

# create first migration. Run the first time to create migrations directory.
# export DATABASE_URL=postgres://postgres:Allapattah123456@localhost:5432/newsletter
# sqlx migrate add create_subscriptions_table
