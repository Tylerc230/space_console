#ifndef INCLUDE_COLOR_PROGRAM
#define INCLUDE_COLOR_PROGRAM

#include "Program.h"
#include "colorpalettes.h"
#include <FastLED.h>
#define NUM_PALETTES 7
struct SparkleEffect;
struct ColorProgram: Program {
    ColorProgram() { }
    virtual void init(Inputs & inputs, AirplaneLEDs & airplane);
    virtual void update(Inputs & inputs, AirplaneLEDs & airplane);
    uint8_t index = 0;
    SparkleEffect *sparkles = NULL;
};

#endif
