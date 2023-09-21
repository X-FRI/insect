/** Copyright (C) 2023 Muqiu Han <muqiu-han@outlook.com> */

#ifndef SINBUGER_ERROR_H
#define SINBUGER_ERROR_H

#include "log.hpp"
#include "result.hpp"
#include <cerrno>
#include <cstdint>
#include <exception>
#include <map>
#include <stdexcept>
#include <string>
#include <utility>

namespace sinbuger::error
{
  enum class Code
  {
    Undefined,
    Unix,
  };

  inline const std::map<Code, std::string> CODE_TO_STRING = {
    { Code::Undefined, "Unknown Error" },
    { Code::Unix, "Unix Error" },
  };

  class Err
  {
   public:
    Err() { std::terminate(); }

    Err(const Code code,
        std::string custom,
        const uint32_t line,
        std::string file,
        std::string function)
      : m_code(code)
      , m_errno(errno)
      , m_custom(std::move(custom))
      , m_line(line)
      , m_file(std::move(file))
      , m_function(std::move(function))
    {
      LOG_ERROR << to_string() << ":" << function;
      LOG_INFO << "In the " << function << " function on line " << line << " of the file "
               << file << " (" << file << ":" << line << ")";
    }

    [[nodiscard]] static int32_t to_exit_code() noexcept;
    [[nodiscard]] std::string to_string() const noexcept;

   private:
    Code m_code;

    int m_errno;
    uint32_t m_line;

    std::string m_custom;
    std::string m_function;
    std::string m_file;
  };

}

#define ERR(CODE) Err(sinbuger::error::Err(CODE, "", __LINE__, __FILE__, __FUNCTION__))
#define ERR_MSG(CODE, MSG)                                                               \
  Err(sinbuger::error::Err(CODE, MSG, __LINE__, __FILE__, __FUNCTION__))

#endif /* sinbuger_ERROR_H */
