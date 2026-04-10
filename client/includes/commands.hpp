#include <iostream>
#include <string>

class Commands {
    public:
        Commands() = default;
        ~Commands() = default;

        virtual void execute(void) = 0;
};
