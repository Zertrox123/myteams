#include <string>
#include <iostream>
#include <vector>
#include "client.hpp"
#include <memory>

int main(int argc, char **argv)
{
    std::string ip = (argv[1] != NULL) ? argv[1] : "none";
    int port = (argv[2] != NULL) ? atoi(argv[2]) : 0;

    if ((port != 0 && ip != "none") && argc == 3) {
        auto client = std::make_unique<Client>(ip, port);
        if (client)
            client->run();
        else
            exit(84);
    } else {
        std::cout << "USAGE: ./myteams_cli ip port" << std::endl << std::endl;
        std::cout << " ip is the server ip address on which the server socket listens" << std::endl;
        std::cout << " port is the port number on which the server socket listens" << std::endl;
    }
}
