#include "include/debugger.h"
#include "include/log.hpp"

using namespace sinbuger;

int
main(int argc, char ** argv)
{
  sinbuger::log::set_level(LOG_LEVEL_DEBUG);
  if (argc < 2)
    {
      LOG_ERROR << "Program name not specified";
      exit(EXIT_FAILURE);
    }

  const std::string program_name = argv[1];
  const pid_t program_pid = fork();

  if (program_pid == 0)
    {
    }
  else if (program_pid >= 1)
    {
      LOG_INFO << "Started debugging process on " << program_pid;
      debugger::Debugger(program_name, program_pid).run();
    }

  return 0;
}
