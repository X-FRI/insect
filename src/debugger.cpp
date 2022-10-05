//
// Created by muqiu on 10/3/22.
//

#include "debugger.h"

#include <sys/ptrace.h>
#include <wait.h>

#include <iostream>

#include "linenoise.h"

namespace sinbuger {
    void debugger::run() const noexcept {
        int32_t status = 0, options = 0;
        waitpid(m_pid, &status, options);

        // we give the command to a handle_command function which we’ll write shortly,
        // then we add this command to the linenoise history and free the resource.
        char *line;
        while ((line = linenoise("sinbuger> ")) != nullptr) {
            handle_command(line);
            linenoiseHistoryAdd(line);
            linenoiseFree(line);
        }
    }

    void debugger::handle_command(const std::string &line) const noexcept {
        // Our commands will follow a similar format to gdb and lldb.
        // To continue the program, a user will type continue or cont or even just c.
        // If they want to set a breakpoint on an address, they’ll write break 0xDEADBEEF,
        // where 0xDEADBEEF is the desired address in hexadecimal format. Let’s add support for these commands.
        std::vector<std::string> args = split(line);
        std::string command = args.at(0);

        if (is_prefix(command, "continue")) {
            continue_execution();
        } else {
            std::cerr << "Unknown command" << std::endl;
        }
    }

    std::vector<std::string> debugger::split(const std::string &s) noexcept {
        size_t pos_start = 0, pos_end;
        std::string token;
        std::vector<std::string> res;

        while ((pos_end = s.find(32, pos_start)) != std::string::npos) {
            token = s.substr(pos_start, pos_end - pos_start);
            pos_start = pos_end + 1;
            res.push_back(token);
        }

        res.push_back(s.substr(pos_start));
        return res;
    }

    bool debugger::is_prefix(const std::string &s, const std::string &of) noexcept {
        if (s.size() > of.size()) {
            return false;
        }
        return std::equal(s.begin(), s.end(), of.begin());
    }

    void debugger::continue_execution() const noexcept {
        ptrace(PTRACE_CONT, m_pid, nullptr, nullptr);
        int32_t wait_status = 0, options = 0;
        waitpid(m_pid, &wait_status, options);
    }
}// namespace sinbuger