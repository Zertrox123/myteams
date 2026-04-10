#include "client.hpp"

Client::Client(std::string ip, int port) : _port(port), _ip(ip) {}

Client::~Client() {}

int Client::createsocket(void) const
{
    int sock = socket(AF_INET, SOCK_STREAM, 0);

    if (sock == -1) {
        std::cout << "Critical error" << std::endl;
        exit(84);
    }
    return sock;
}

sockaddr_in Client::createaddress(void) const
{
    sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons(_port);

    if (inet_pton(AF_INET, _ip.c_str(), &addr.sin_addr) <= 0) {
        std::cout << "Critical error" << std::endl;
        exit(84);
    }
    return addr;
}

void Client::connectserver(int sock, const sockaddr_in &addr) const
{
    if (connect(sock, reinterpret_cast<const struct sockaddr*>(&addr), sizeof(addr)) == -1) {
        std::cout << "Critical error" << std::endl;
        exit(84);
    }
}

bool Client::handlecmds(int sock) const
{
    std::string input;

    if (std::getline(std::cin, input) && !input.empty()) {
        int bytes = send(sock, (input + "\n").c_str(), input.size() + 1, 0);
        if (bytes < 0)
            return false;
    }
    return true;
}

bool Client::handleserversidemessages(int sock, std::string &buf) const
{
    int bytes = recv(sock, buf.data(), buf.size(), 0);

    if (bytes > 0) {
        std::string msg(buf.begin(), buf.begin() + bytes);
        std::cout << msg << std::endl;
    } else if (bytes == 0) {
        return false;
    }
    return true;
}

void Client::handle(int sock) const
{
    struct pollfd fds[2];
    std::string buf(1024, '\0');

    fds[0].fd = STDIN_FILENO;
    fds[0].events = POLLIN;
    fds[1].fd = sock;
    fds[1].events = POLLIN; 
    while (true) {
        if (poll(fds, 2, -1) < 0)
            break;
        if (fds[0].revents & POLLIN) {
            if (!handlecmds(sock))
                break;
        }
        if (fds[1].revents & POLLIN) {
            if (!handleserversidemessages(sock, buf))
                break;
        }
    }
}

void Client::run(void) const
{
    int socket = createsocket();
    sockaddr_in addr = createaddress();

    connectserver(socket, addr);
    handle(socket);
    close(socket);
}
