/** Copyright (C) 2023 Muqiu Han <muqiu-han@outlook.com> */

#include "include/unix.h"
#include <fcntl.h>
#include <filesystem>
#include <fstream>
#include <sys/capability.h>
#include <sys/ptrace.h>
#include <sys/utsname.h>
#include <unistd.h>

namespace sinbuger::unix
{
  Result<Void, error::Err>
  Filesystem::Mkdir(const std::string & path) noexcept
  {
    try
      {
        std::filesystem::create_directories(path);
      }
    catch (...)
      {
        LOG_ERROR << "Cannot create direcotry " << path;
        return ERR(error::Code::Unix);
      }

    return Ok(Void());
  }

  /** Filesystem::Close */
  GENERATE_NO_RET_VALUE_SYSTEM_CALL_WRAPPER(
    int, -1, Filesystem::Close(int fd), close, fd);

  /** Filesystem::Rmdir */
  GENERATE_NO_RET_VALUE_SYSTEM_CALL_WRAPPER(
    int, -1, Filesystem::Rmdir(const std::string & path), rmdir, path.c_str());

  /** Filesystem::Open */
  GENERATE_SYSTEM_CALL_WRAPPER(int,
                               -1,
                               Filesystem::Open(const std::string & file, int flag),
                               open,
                               file.c_str(),
                               flag);

  /** Filesystem::Open */
  GENERATE_SYSTEM_CALL_WRAPPER(int,
                               -1,
                               Filesystem::Open(const std::string & file,
                                                int flag,
                                                int mode),
                               open,
                               file.c_str(),
                               flag,
                               mode);

  /** Filesystem::Write */
  GENERATE_NO_RET_VALUE_SYSTEM_CALL_WRAPPER(int,
                                            -1,
                                            Filesystem::Write(int fd,
                                                              const std::string & s),
                                            write,
                                            fd,
                                            s.c_str(),
                                            s.length());

  /** Process::Waitpid */
  GENERATE_NO_RET_VALUE_SYSTEM_CALL_WRAPPER(int,
                                            -1,
                                            Process::Waitpid(pid_t pid,
                                                             int * wait_status,
                                                             int options),
                                            waitpid,
                                            pid,
                                            wait_status,
                                            options);

  /** Process::Ptrace */
  GENERATE_NO_RET_VALUE_SYSTEM_CALL_WRAPPER(
    long,
    -1,
    Process::Ptrace(enum __ptrace_request request, pid_t pid, void * addr, void * data),
    ptrace,
    request,
    pid,
    addr,
    data);

  Result<utsname, error::Err>
  Utsname::Get() noexcept
  {
    struct utsname host = { 0 };
    if (-1 == uname(&host))
      return ERR(error::Code::Unix);

    return Ok(host);
  }

  Result<std::string, error::Err>
  Filesystem::read_entire_file(const std::string & path) noexcept
  {
    constexpr auto read_size = std::size_t(4096);

    auto stream = std::ifstream(path);
    stream.exceptions(std::ios_base::badbit);

    if (!stream.is_open())
      return ERR(error::Code::Unix);

    auto out = std::string();
    auto buf = std::string(read_size, '\0');
    while (stream.read(&buf[0], read_size))
      out.append(buf, 0, stream.gcount());

    out.append(buf, 0, stream.gcount());
    return Ok(out);
  }

  Result<Void, error::Err>
  Filesystem::Write(const std::string & path, const std::string & s) noexcept
  {
    int fd = Filesystem::Open(path, O_WRONLY | O_CREAT, 0777).unwrap();
    Filesystem::Write(fd, s).unwrap();
    return Filesystem::Close(fd);
  }
}