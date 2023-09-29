FROM debian:trixie-slim

ARG SSH_PASSWORD

# Update and install necessary software
RUN apt-get update && apt-get install -y \
  nginx \
  tor \
  openssh-server

# Add custom files to container
COPY index.html /var/www/html/
COPY nginx.conf /etc/nginx/
COPY torrc /etc/tor/
COPY sshd_config /etc/ssh/

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN echo "root:${SSH_PASSWORD}" | chpasswd

# Expose necessary ports
EXPOSE 80
EXPOSE 4242

# Start services
CMD service ssh start && service tor start && nginx -g "daemon off;"
