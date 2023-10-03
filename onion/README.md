Instruction :

```bash
docker build --build-arg SSH_PASSWORD="pass" -t ft_onion .
docer run --name onion -p 8080:80 -p 4243:4242 ft_onion
```

To get the onion link:

```bash
docker exec -it onion cat /var/lib/tor/hidden_service/hostname
```
