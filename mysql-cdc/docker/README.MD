# Docker Mysql Installation

CDC sample application has a **producer** and a **consumer** that replicate from a database to another.

In general these databases are on different machines, but for the purpose of this demo, we'll use **Docker** to run 2 MYSQL instances in parallel.

## TL;TR

If you just want to get started, ensure **Docker is Running** and invoke **Install** for **consumer** and **producer**:

```
$ ./install.sh -n mysql-consumer -p 3080
$ ./install.sh -n mysql-consumer -p 3090
```

The 2 mysql instance should be up and running. For addition information checkout the rest of the document.

## Setup Script

We've built an **install.sh** script to simplify the installation. The script takes a "name" and a "port" as parameters:

* **name**:
  * uniquely identifies the container
  * marks directory where we'll make /var/lib/mysql data (ie: "~/mysql-cdc/<name>)

* **port**:
  * uniquely identifies a mysql instance

The script also sets-up **root** password and creates a fluvio user.

* Root username/password: 
    * **root/root**
* Fluvio username/pass: 
    * **fluvio/fluvio4cdc!**

Fluvio user is allowed **full privileges** on databases prepended with:

```
flvDb
```

## Start Producer

Start producer mysql container:

```
./install.sh -n mysql-consumer -p 3080
```

Ensure container is running

```
$ docker ps
CONTAINER ID        IMAGE               COMMAND                  CREATED             STATUS              PORTS                               NAMES
a672f78e73da        mysql-80            "docker-entrypoint.s…"    1 minutes ago      Up 1 minutes       33060/tcp, 0.0.0.0:3080->3306/tcp   mysql-producer
```

The files have been mapped

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

You can login (assumes **mysql client** is installed on your machine).

first, **as root**

```
$ mysql -h 0.0.0.0 -P 3080 -uroot -proot
...
Server version: 8.0.20 MySQL Community Server - GPL
...

mysql> 
```

then, **as fluvio**

```
$ mysql -h 0.0.0.0 -P 3080 -ufluvio -pfluvio4cdc!
...
Server version: 8.0.20 MySQL Community Server - GPL
...

mysql> 
```


## Start Consumer

Start consumer mysql container:

```
$ ./install.sh -n mysql-consumer -p 3090
```

To ensure, consumer is also running, perform the same same checks as above on 
* directory: **mysql-consumer**
* port **3090**
