#include "include/debugger.h"
#include "include/linenoise.hpp"
#include "include/unix.h"
#include "include/utils.h"
#include "linenoise.h"
#include <sys/wait.h>

namespace sinbuger::debugger
{
  Result<Void, error::Err>
  Debugger::run() noexcept
  {
    int wait_status = 0;
    int options = 0;

    char * line = nullptr;
    while ((line = linenoise("sinbuger#=> ")) != nullptr)
      {
        handle_command(line);
        linenoiseHistoryAdd(line);
        linenoiseFree(line);
      }
  }

  Result<Void, error::Err>
  Debugger::handle_command(std::string line) noexcept
  {
    const std::vector<std::string> args = utils::String::split(line, ' ').unwrap();
    const std::string command = args[0];

    if (utils::String::is_prefix(command, "continue").unwrap())
      return continue_execution();
    else
      return ERR_MSG(error::Code::Debugger, "Unknown command");
  }

  Result<Void, error::Err>
  Debugger::continue_execution() noexcept
  {
    unix::Process::Ptrace(PTRACE_CONT, m_pid, nullptr, nullptr).unwrap();

    int wait_status = 0;
    int options = 0;
    return unix::Process::Waitpid(m_pid, &wait_status, options);
  }
}
