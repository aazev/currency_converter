# Currency Converter [![Build Status](https://github.com/aazev/currency_converter/actions/workflows/linux.yml/badge.svg?branch=main)](https://github.com/aazev/currency_converter/actions/workflows/linux.yml) [![Build Status](https://github.com/aazev/currency_converter/actions/workflows/windows.yml/badge.svg?branch=main)](https://github.com/aazev/currency_converter/actions/workflows/windows.yml)

## About

Simple currency conversion application. Featuring a backend API, a RabbitMQ data-worker, a RabbitMQ seeder, a React Built frontend, and docker container to support all features.

### Environment variables

|         Variable         | Description                                                                                        | Example Value                                                  |
| :----------------------: | :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------- |
|         API_KEY          | [Exchange Rates Data Api](https://apilayer.com/marketplace/exchangerates_data-api) Key             | 32 character lengh String                                      |
|       DATABASE_URL       | Database connection URL                                                                            | postgresql://cc_owner:fh0xm1@127.0.0.1:5432/currency_converter |
| DATABASE_MAX_CONNECTIONS | Maximum number of database connections to spawn                                                    | 10 (default)                                                   |
| DATABASE_MIN_CONNECTIONS | Maximum number of database connections to spawn                                                    | 2 (default)                                                    |
|       SOCKET_ADDR        | Path to UNIX socket file to be created. This enables the backend to communicate via a UNIX Socket. | /var/run/rust/currency_converter.sock                          |
|       BIND_ADDRESS       | IP address to bind backend listener                                                                | 127.0.0.1:8000                                                 |

NOTE: **Database connections are not shared among features. This meaning that, using default values, each feature that uses the database, will spawn 2 minimum connections, allowing a max of 10**.

## Desafio BoraCodar#9 RocketSeat

This application was made in order to take part in the 9th BoraCodar [RocketSeat](https://www.rocketseat.com.br/) challenge;

# Features

## Backend

This feature uses database connections.

The backend microservice can be used in two distinct modes. It can be bound to an IP address and port (this is the default mode), OR it can be bound to a UNIX socket.
When using this feature in address mode (default), the application binds itself to the IP address and port associated with the `BIND_ADDRESS`environment variable, and may be initialized with the following command:

```bash
backend
```

In order to use this feature in socket mode, this application must be run like shown below:

```bash
sudo -u <username> backend -m socket
```

Where `<username>` is an existing user with write and read access to the path associated with the `SOCKET_ADDR` environment variable.
| Obs: Currently there is no way of setting this variable using the command line.

## data-worker

This feature uses database connections.

The data worker is responsible for updating the data stored in the database. It will bind itself using the attached `AMPQ_ADDR` and `RMQ_QUEUE_NAME` environment variables, and will consume any suitable messages in the queue, "acking" the valid ones and "nacking" those invalid.

The **seeder** feature ensures that the database is always up to date.

### Environment variables

|       Variable        | Description                                                             | Example Value                           |
| :-------------------: | :---------------------------------------------------------------------- | :-------------------------------------- |
| WORKER_PREFETCH_COUNT | Sets the amount of messages to be retrieved by each listener's instance | 8 (defaults to the amount of CPU cores) |
|       AMPQ_ADDR       | Address to the RabbitMQ server                                          | amqp://127.0.0.1:5672/                  |
|    RMQ_QUEUE_NAME     | Name of the queue to be used                                            | currency_fetcher                        |

# README is still a WIP
