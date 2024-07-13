watchdog_rs

port 4035


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