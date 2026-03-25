##
## EPITECH PROJECT, 2026
## Makefile
## File description:
## project makefile
##

all:
	cargo build --release
	mv target/release/myteams_cli    .
	mv target/release/myteams_server .

clean:
	rm -rf target

fclean: clean
	rm -rf  myteams_cli myteams_server

re: fclean all
