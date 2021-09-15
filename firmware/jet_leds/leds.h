#ifndef INCLUDE_JET_LEDS
#define INCLUDE_JET_LEDS
#include <FastLED.h>
#include "constants.h"
struct LEDStrip {
  void setLED(int index, CRGB color);
  void fill(CRGB color);
  LEDStrip(uint8_t pinNo, uint8_t numLEDs, bool reversed) : pinNo(pinNo), numLEDs(numLEDs), reversed(reversed) { }
  const uint8_t pinNo;
  const uint8_t numLEDs;
  const bool reversed;
  CRGB leds[MAX_LEDS];
};

struct LEDPair {
  void setPair(int index, CRGB color);
  void fill(CRGB color);
  LEDStrip left;
  LEDStrip right;
  LEDPair(uint8_t numLEDs, uint8_t leftPin, uint8_t rightPin, bool reversed): left(LEDStrip(leftPin, numLEDs, reversed)), right(LEDStrip(rightPin, numLEDs, reversed)) {}
};

enum LEDStripID {
  TAIL, L_NOSE, R_NOSE, L_WING, R_WING, NUM_STRIPS = 5
};

struct AirplaneLEDs {
  void init();
  void fill(CRGB color);
  void clearLEDs();
  void setAll(int index, CRGB color);
  void playTone(int freq);
  void toneOff();
  LEDStrip & stripForId(LEDStripID id);
  LEDPair nose = LEDPair(NOSE_LED_COUNT, L_NOSE_LED_PIN, R_NOSE_LED_PIN, false);
  LEDPair wings = LEDPair(WING_LED_COUNT, L_WING_LED_PIN, R_WING_LED_PIN, false);
  LEDStrip tail = LEDStrip(TAIL_LED_PIN, TAIL_LED_COUNT, false);
};

#endif
