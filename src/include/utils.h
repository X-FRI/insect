#ifndef SINBUGER_UTILS_H
#define SINBUGER_UTILS_H

#include "error.h"
#include "result.hpp"
#include <string>
#include <vector>

namespace sinbuger::utils
{
  struct String
  {
    static Result<std::vector<std::string>, error::Err> split(const std::string & s,
                                                              char delimiter) noexcept;

    static bool is_prefix(const std::string & s, const std::string & of) noexcept;
  };
}

#endif /* SINBUGER_UTILS_H */
