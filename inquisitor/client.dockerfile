FROM debian:stable-20230919-slim

RUN apt-get update && apt-get install -y ftp iputils-ping net-tools
