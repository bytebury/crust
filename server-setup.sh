#!/bin/bash

# Usage: ./server-setup.sh example.com 8080 example@example.com

DOMAIN=$1
PORT=$2
EMAIL=$3

if [ -z "$DOMAIN" ] || [ -z "$PORT" ]; then
  echo "Usage: $0 domain port email"
  exit 1
fi

NGINX_AVAILABLE="/etc/nginx/sites-available/$DOMAIN"
NGINX_ENABLED="/etc/nginx/sites-enabled/$DOMAIN"

# Create initial nginx config (HTTPS with HTTP/2)
cat >$NGINX_AVAILABLE <<EOF
server {
    server_name $DOMAIN www.$DOMAIN;

    location / {
        proxy_pass http://127.0.0.1:$PORT;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;

        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;

        proxy_cache_bypass \$http_upgrade;

        proxy_read_timeout 86400s;
        proxy_send_timeout 86400s;
    }

    listen 80;
    # Redirect all HTTP traffic to HTTPS
    return 301 https://\$host\$request_uri;
}

server {
    listen 443 ssl http2;
    server_name $DOMAIN www.$DOMAIN;

    ssl_certificate /etc/letsencrypt/live/$DOMAIN/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/$DOMAIN/privkey.pem;

    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers 'ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-AES128-GCM-SHA256:AES128-GCM-SHA256';
    ssl_prefer_server_ciphers on;

    location / {
        proxy_pass http://127.0.0.1:$PORT;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;

        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;

        proxy_cache_bypass \$http_upgrade;

        proxy_read_timeout 86400s;
        proxy_send_timeout 86400s;
    }
}
EOF

# Enable site
ln -s $NGINX_AVAILABLE $NGINX_ENABLED 2>/dev/null

# Test nginx config
nginx -t || {
  echo "Nginx config test failed"
  exit 1
}

# Reload nginx
systemctl reload nginx

# Run certbot to automatically add SSL
certbot --nginx -d $DOMAIN -d www.$DOMAIN --non-interactive --agree-tos -m $EMAIL

# Test again to ensure http2 is properly set
nginx -t || {
  echo "Nginx config test failed after certbot"
  exit 1
}

# Reload nginx to apply changes
systemctl reload nginx

echo "✅ Site $DOMAIN created, secured with SSL, WebSockets, and HTTP/2 enabled on port $PORT."
