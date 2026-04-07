##
## EPITECH PROJECT, 2026
## Makefile
## File description:
## project makefile
##

CXX = clang++
CXXFLAGS = -Wall -Werror -Wextra -std=c++20
SRC = client/src/main.cpp
OBJ = $(SRC:.cpp=.o)
NAME = myteams_cli
SERVER = myteams_server


all: server client

server:
	cargo build --release
	rm -f $(SERVER)
	mv target/release/$(SERVER) .

client: $(OBJ)
	$(CXX) $(CXXFLAGS) -o $(NAME) $(OBJ)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ)
	cargo clean

fclean: clean
	rm -f $(SERVER)
	rm -f $(NAME)

re: fclean all

.PHONY: all clean fclean re server client
