#ifndef INCLUDE_RAIN_PROGRAM
#define INCLUDE_RAIN_PROGRAM

#include "Program.h"
#include "colorpalettes.h"
#include <FastLED.h>
struct Drop;
struct RainProgram: Program {
  RainProgram() { }
  virtual void init(Inputs & inputs, AirplaneLEDs & airplane);
  virtual void update(Inputs & inputs, AirplaneLEDs & airplane);

  void createDrops(uint8_t count, bool s1Pressed, bool s2Pressed);
  ~RainProgram();
  uint8_t numDrops;
  Drop *drops = NULL;
  uint8_t currentPaletteIndex = 0;
};

#endif
