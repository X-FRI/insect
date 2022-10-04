#include <iostream>
#include <unistd.h>
#include "debugger.h"

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "Program name not specified";
        return -1;
    }

    auto prog = argv[1];
    auto pid = fork();

    if (pid == 0) {

    } else if (pid >= 1) {
        std::cout << "Started debugging process : " << pid << std::endl;
        sinbuger::debugger dbg(prog, pid);
        dbg.run();
    }
    return 0;
}
