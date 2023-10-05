all:
	cargo build
	cp target/debug/stockholm .

clean:
	cargo clean

fclean: clean
	rm -f stockholm

re: fclean all

.PHONY: all clean fclean re
