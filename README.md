# Krakend/Keycloak integration example

I needed to use Keycloak as an authentication provider for Krakend, but I couldn't find any example on how to do it. So I decided to create this repository to show how to do it.

I generated this project using Krakend playground as the base and deleted everything apart from logging.

I then created a simple http server in rust that listen on two ports (to see the load balancing of Krakend), I secured this endpoints (`/api/health`).

The realm is named as `seasonist`, because it was the name of the project I was working on when I created this example.

## How to run

```bash
docker compose up
```
