# Reservation service in Mahjong project.

## Make a PostgresSql container in docker.
```bash
docker pull postgres
docker run --name reservation-service-pgdb -p 5432:5432 -e POSTGRES_PASSWORD=123456 -d postgres
```