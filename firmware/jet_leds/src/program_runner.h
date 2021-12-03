#ifndef INCLUDE_PROGRAM_RUNNER
#define INCLUDE_PROGRAM_RUNNER
#include <FastLED.h>
#include "leds.h"
#include "inputs.h"
struct Program;
struct ProgramRunner {
  uint8_t currentProgramIndex = 0xFF;
  AirplaneLEDs airplane = AirplaneLEDs();
  Inputs inputs = Inputs();
  void init();
  void update();
  private:
  Program * createProgram(uint8_t progIndex);
  Program * currentProgram = NULL;
};

#endif
