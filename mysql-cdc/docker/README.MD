# Mysql - Docker Setup

CDC sample application has a **producer** and a **consumer** that replicate from a database to another. **Cdc-producer** must be co-located with the **master** database to read its **binlog** files and generate Fluvio events. **Cdc-consumer** needs remote access to the **slave** database. For additional information, checkout [MYSQL documentation](https://dev.mysql.com/doc/refman/8.0/en/replication-howto-masterbaseconfig.html).

## Prerequisites

* **Mysql Client** - for accessing mysql databases .
* **Docker** - to run two MYSQL servers in parallel.

**Note** 

If your databases are already installed you may skip the Docker installation. Change the **producer-profile.toml** and **consumer-profile.toml** files instead.


## Installation Script

Installation script script sets-up **root** password and creates a **fluvio** user.
    * user: **root**, password: **root**
    * user: **fluvio**, password: **fluvio4cdc!**
      * **fluvio** user has **full privileges** to all databases prepended with **flvDb**.

To run installer, you must populate the following parameters:
 * **-n**: container name
 * **-d**: local data path to map mysql data
 * **-p**: port of the mysql containers (producer: 3080, consumer: 3090)


## Install Producer

Run installer

```
$ ./install.sh -n mysql-producer -d ~/mysql-cdc/mysql-producer -p 3080
```

Ensure container is running

```
$ docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                               NAMES
a672f78e73da        mysql-80            "docker-entrypoint.s…"    1 minutes ago      Up 1 minutes       33060/tcp, 0.0.0.0:3080->3306/tcp   mysql-producer
```

Ensure **binlog** files are visible:

```
$ ls ~/mysql-cdc/mysql-producer/
... 
binlog.000001
binlog.000002
binlog.000003		
binlog.000004
binlog.000005
binlog.index
...
```

Connect to **producer** database:

```
$ mysql -h 0.0.0.0 -P 3080 -ufluvio -pfluvio4cdc!
...
Server version: 8.0.20 MySQL Community Server - GPL
...

mysql> 
```


## Install Consumer

Run installer

```
$ ./install.sh -n mysql-consumer -d ~/mysql-cdc/mysql-consumer -p 3090
```

To ensure, consumer is also running, perform the same same checks as above on 
* directory: **mysql-consumer**
* port **3090**


Connect to **consumer** database:

```
$ mysql -h 0.0.0.0 -P 3090 -ufluvio -pfluvio4cdc!
...
Server version: 8.0.20 MySQL Community Server - GPL
...

mysql> 
```

## MYSQL Commands

A list of mysql test commands can be found at [MYSQL-COMMANDS.MD](../MYSQL-COMMANDS.MD)