FROM stilliard/pure-ftpd

RUN (echo pass; echo pass) | pure-pw useradd user -u ftpuser -d /home/ftpusers/user && pure-pw mkdb
