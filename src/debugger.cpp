#include "include/debugger.h"
#include "include/breakpoint.h"
#include "include/linenoise.hpp"
#include "include/log.hpp"
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

    return Ok(Void());
  }

  Result<Void, error::Err>
  Debugger::handle_command(std::string line) noexcept
  {
    const std::vector<std::string> args = utils::String::split(line, ' ').unwrap();
    const std::string command = args[0];

    if (utils::String::is_prefix(command, "continue"))
      return continue_execution();
    if (utils::String::is_prefix(command, "break"))
      // TODO: Assume that the user has written 0xADDRESS
      set_breakpoint_at_addr(std::stol(std::string{ args[1], 2 }, 0, 16));
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

  Result<Void, error::Err>
  Debugger::set_breakpoint_at_addr(std::intptr_t addr) noexcept
  {
    LOG_INFO << "Set breakpoint at address 0x" << std::hex << addr;

    Breakpoint breakpoint{ m_pid, addr };
    breakpoint.enable().unwrap();
    m_breakpoints.insert(std::make_pair(addr, breakpoint));

    return Ok(Void());
  }
}
