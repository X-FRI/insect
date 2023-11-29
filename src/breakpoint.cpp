#include "include/breakpoint.h"
#include "include/unix.h"
#include <sys/ptrace.h>

namespace sinbuger::breakpoint
{

  Result<Void, error::Err>
  Breakpoint::enable() noexcept
  {
    auto data = ptrace(PTRACE_PEEKDATA, m_pid, m_addr, nullptr);
    // Save bottom byte and set the bottom byte to 0xcc (int3)
    m_saved_data = static_cast<uint8_t>(data & 0xff);
    unix::Process::Ptrace(PTRACE_POKEDATA,
                          m_pid,
                          reinterpret_cast<void *>(m_addr),
                          reinterpret_cast<void *>(((data & ~0xff) | 0xcc)))
      .unwrap();
    m_enable = true;

    return Ok(Void());
  }

  Result<Void, error::Err>
  Breakpoint::disable() noexcept
  {
    auto data = ptrace(PTRACE_PEEKDATA, m_pid, m_addr, nullptr);
    // Clear bottom byte and set the bottom byte to `m_saved_data`
    unix::Process::Ptrace(PTRACE_POKEDATA,
                          m_pid,
                          reinterpret_cast<void *>(m_addr),
                          reinterpret_cast<void *>(((data & ~0xff) | m_saved_data)));

    m_enable = false;
    return Ok(Void());
  }

}
