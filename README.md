watchdog_rs

port 4035



# Logging builds & deploys to Discord webhook (optional)
Set this env var with your chosen webhook log
```env
WATCHDOG_RS_DISCORD_WEBHOOK=https://discord.com/api/webhooks/xxxxxxxx
```

# Github API key (only for private repos)
Set this as an `env` var
```env
WATCHDOG_RS_GITHUB_KEY=xxxxxxxxxxxxxxxxxxxxxx
```

# Setting a build key
As an auth measure, build_keys are mandatory so set one in the `env` var
```env
WATCHDOG_RS_BUILD_KEY=xxxx
```

# Config `watchdog.yml` 
You can add as many services as you need
```yml
services:
  service_1:
    WATCHDOG_RS_BUILD_KEY: "your_build_key_here"
    WATCHDOG_RS_DISCORD_WEBHOOK: "your_discord_webhook_here"
    WATCHDOG_RS_REPOSITORY_URL: "your_repository_url_here"
    WATCHDOG_RS_BUILD_ID: "your_unique_build_id"
    WATCHDOG_RS_RUST_TEST_COMMAND: "cargo test"
    WATCHDOG_RS_RUST_BUILD_COMMAND: "cargo build --release"

  service_2:
    WATCHDOG_RS_BUILD_KEY: "your_build_key_here"
    WATCHDOG_RS_DISCORD_WEBHOOK: "your_discord_webhook_here"
    WATCHDOG_RS_REPOSITORY_URL: "your_repository_url_here"
    WATCHDOG_RS_BUILD_ID: "your_unique_build_id"
    WATCHDOG_RS_RUST_TEST_COMMAND: "cargo test"
    WATCHDOG_RS_RUST_BUILD_COMMAND: "cargo build --release"

  # Add more services as needed
```

# Exposing the port to a domain via NGINX
```
GNU nano 6.2 /etc/nginx/sites-available/WILDCARD.DOMAIN.TLD 

server { 
    listen 80; 
    server_name WILDCARD.DOMAIN.TLD; 
    location / { 
        proxy_pass http://127.0.0.1:PORT; 
        proxy_set_header Host $host; 
        proxy_set_header X-Real-IP $remote_addr; 
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for; 
        proxy_set_header X-Forwarded-Proto $scheme; 
    } 
}
```
