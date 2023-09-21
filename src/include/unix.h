/** Copyright (C) 2023 Muqiu Han <muqiu-han@outlook.com> */

#ifndef SINBUGER_UNIX_H
#define SINBUGER_UNIX_H

#include "error.h"
#include "result.hpp"
#include <linux/prctl.h>
#include <sys/capability.h>
#include <sys/prctl.h>
#include <sys/ptrace.h>
#include <sys/utsname.h>
#include <sys/wait.h>

/** Auto generate wrapper function for system calls function  */
#define GENERATE_SYSTEM_CALL_WRAPPER(OK_TYPE,                                            \
                                     FAILURE_VALUE,                                      \
                                     WRAPPER_FUNCTION_SIGNATURE,                         \
                                     SYSTEM_CALL_FUNCTION_NAME,                          \
                                     ...)                                                \
  Result<OK_TYPE, error::Err> WRAPPER_FUNCTION_SIGNATURE noexcept                        \
  {                                                                                      \
    OK_TYPE SYSTEM_CALL_FUNCTION_RESULT = SYSTEM_CALL_FUNCTION_NAME(__VA_ARGS__);        \
    if (FAILURE_VALUE == SYSTEM_CALL_FUNCTION_RESULT)                                    \
      return ERR(error::Code::Unix);                                                     \
    return Ok(SYSTEM_CALL_FUNCTION_RESULT);                                              \
  }

/** Auto generate wrapper function for system calls function  */
#define GENERATE_NO_RET_VALUE_SYSTEM_CALL_WRAPPER(OK_TYPE,                               \
                                                  FAILURE_VALUE,                         \
                                                  WRAPPER_FUNCTION_SIGNATURE,            \
                                                  SYSTEM_CALL_FUNCTION_NAME,             \
                                                  ...)                                   \
  Result<Void, error::Err> WRAPPER_FUNCTION_SIGNATURE noexcept                           \
  {                                                                                      \
    OK_TYPE SYSTEM_CALL_FUNCTION_RESULT = SYSTEM_CALL_FUNCTION_NAME(__VA_ARGS__);        \
    if (FAILURE_VALUE == SYSTEM_CALL_FUNCTION_RESULT)                                    \
      return ERR(error::Code::Unix);                                                     \
    return Ok(Void());                                                                   \
  }

namespace sinbuger::unix
{
  class Filesystem
  {
   public:
    static Result<Void, error::Err> Mkdir(const std::string & path) noexcept;
    static Result<std::string, error::Err>
    read_entire_file(const std::string & path) noexcept;
    static Result<int, error::Err> Open(const std::string & file, int flag) noexcept;
    static Result<int, error::Err>
    Open(const std::string & file, int flag, int mode) noexcept;
    static Result<Void, error::Err> Rmdir(const std::string & file) noexcept;
    static Result<Void, error::Err> Close(int fd) noexcept;
    static Result<Void, error::Err> Write(int fd, const std::string & s) noexcept;
    static Result<Void, error::Err> Write(const std::string & path,
                                          const std::string & s) noexcept;
  };

  class Process
  {
   public:
    static Result<Void, error::Err>
    Waitpid(pid_t pid, int * wait_status, int options) noexcept;

    static Result<Void, error::Err>
    Ptrace(enum __ptrace_request request, pid_t pid, void * addr, void * data) noexcept;
  };

  class Utsname
  {
   public:
    static Result<utsname, error::Err> Get() noexcept;
  };
};

#endif /* sinbuger_UNIX_H */
