#include <iostream>
#include <string>
#include <sys/socket.h>
#include <sys/types.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <unistd.h>
#include <poll.h>

class Client {
    public:
        Client(const std::string ip, const int port);
        ~Client();

        void run(void) const;
    private:
        const int _port;
        const std::string _ip;
        int createsocket(void) const;
        sockaddr_in createaddress(void) const;
        void connectserver(int sock, const sockaddr_in &addr) const;
        bool handlecmds(int sock) const;
        bool handleserversidemessages(int sock, std::string &buf) const;
        void handle(int sock) const;
};
