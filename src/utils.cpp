#include "include/utils.h"

namespace sinbuger::utils
{
  Result<std::vector<std::string>, error::Err>
  String::split(const std::string & s, char delimiter) noexcept
  {
    std::vector<std::string> out{};
    std::stringstream ss{ s };
    std::string item;

    while (std::getline(ss, item, delimiter))
        out.push_back(item);

    return Ok(out);
  }

  Result<bool, error::Err>
  String::is_prefix(const std::string & s, const std::string & of) noexcept
  {
    if (s.size() > of.size())
      return Ok(false);
    return Ok(std::equal(s.begin(), s.end(), of.begin()));
  }
}