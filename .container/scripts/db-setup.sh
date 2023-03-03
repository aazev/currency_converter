#!/usr/bin/env bash

export LOG_PATH=.container/scripts/logs/db-setup.log

source .container/scripts/functions.sh

header "Local Database - Setup"

reset_database() {
    pgid=$(get_database_docker_id)
    start_docker_container "$pgid"
    drop_db_connections "$pgid" currency_converter || true

    run_on_db "$pgid" postgres "DROP DATABASE IF EXISTS currency_converter;" >>$LOG_PATH 2>&1 || true
    run_on_db "$pgid" postgres "CREATE DATABASE currency_converter;" >>$LOG_PATH 2>&1 || true
    run_on_db "$pgid" postgres "GRANT ALL PRIVILEGES ON database currency_converter to cc_owner;" >>$LOG_PATH 2>&1
}

if ! is_command_available "docker"; then
    die 'Docker must be installed'
fi

if ! is_command_available "docker compose"; then
    die 'Docker Compose must be installed'
fi

step 'Cleaning previous logs' true

step 'Starting the Local Environment'
make start >>$LOG_PATH 2>&1

step 'Reseting the Database'
reset_database >>$LOG_PATH 2>&1

step 'Your database is set and ready to receive data!'
