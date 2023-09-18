#include "include/log.hpp"

int
main(int argc, char ** argv)
{
  sinbuger::log::set_level(LOG_LEVEL_DEBUG);
  if (argc < 2)
    {
      LOG_ERROR << "Program name not specified";
      exit(EXIT_FAILURE);
    }
  return 0;
}
