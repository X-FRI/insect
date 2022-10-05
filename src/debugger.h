//
// Created by muqiu on 10/3/22.
//

#ifndef SINBUGER_DEBUGGER_H
#define SINBUGER_DEBUGGER_H

#include <string>
#include <vector>

namespace sinbuger {

    /// Give it a loop for listening to user input, and launch that from our parent fork of our main function.
    class debugger {
    public:
        debugger(std::string &&prog_name, pid_t pid)
            : m_prog_name{std::move(prog_name)},
              m_pid{pid} {}

        void handle_command(const std::string &line) const noexcept;

        /// We need to wait until the child process has finished launching,
        /// then keep on getting input from linenoise until we get an EOF (ctrl+d).
        void run() const noexcept;

        /// continue_execution function will use ptrace to tell the process to continue
        void continue_execution() const noexcept;

    private:
        /// split and is_prefix are a couple of small helper functions:
        static std::vector<std::string> split(const std::string &s) noexcept;
        static bool is_prefix(const std::string &s, const std::string &of) noexcept;

    private:
        std::string m_prog_name;
        pid_t m_pid;
    };

}// namespace sinbuger

#endif// SINBUGER_DEBUGGER_H
