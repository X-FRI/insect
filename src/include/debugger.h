#ifndef SINBUGER_DEBUGGER_H
#define SINBUGER_DEBUGGER_H

#include "error.h"
#include "result.hpp"
#include <string>
#include <unistd.h>

namespace sinbuger::debugger
{

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

   private:
    const std::string & m_name;
    const pid_t m_pid;
  };
};

#endif /* SINBUGER_DEBUGGER_H */
