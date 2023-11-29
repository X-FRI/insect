#ifndef SINBUGER_DEBUGGER_H
#define SINBUGER_DEBUGGER_H

#include "breakpoint.h"
#include "error.h"
#include "result.hpp"
#include <cstdint>
#include <string>
#include <unistd.h>
#include <unordered_map>

namespace sinbuger::debugger
{

  using namespace breakpoint;

  class Debugger
  {
   public:
    Debugger(const std::string & program_name, pid_t program_pid)
      : m_name(program_name)
      , m_pid(program_pid)
    {
    }

    Result<Void, error::Err> run() noexcept;
    Result<Void, error::Err> handle_command(std::string line) noexcept;
    Result<Void, error::Err> continue_execution() noexcept;
    Result<Void, error::Err> set_breakpoint_at_addr(std::intptr_t addr) noexcept;

   private:
    const std::string & m_name;
    const pid_t m_pid;

    std::unordered_map<std::intptr_t, Breakpoint> m_breakpoints;
  };
};

#endif /* SINBUGER_DEBUGGER_H */
