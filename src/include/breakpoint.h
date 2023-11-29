#ifndef SINBUGER_BREAKPOINT_H
#define SINBUGER_BREAKPOINT_H

#include "error.h"
#include "result.hpp"
#include <cstdint>
#include <sched.h>

namespace sinbuger::breakpoint
{
  class Breakpoint
  {
   public:
    Breakpoint(pid_t pid, std::intptr_t addr)
      : m_pid{ pid }
      , m_addr{ addr }
      , m_enable{ false }
      , m_saved_data{}
    {
    }

    Result<Void, error::Err> enable() noexcept;
    Result<Void, error::Err> disable() noexcept;

    inline bool
    is_enabled() const noexcept
    {
      return m_enable;
    }

    inline std::intptr_t
    get_address() const noexcept
    {
      return m_addr;
    };

   private:
    const pid_t m_pid;
    const std::intptr_t m_addr;

    bool m_enable;
    uint8_t m_saved_data;
  };
};

#endif /* SINBUGER_BREAKPOINT_H */
