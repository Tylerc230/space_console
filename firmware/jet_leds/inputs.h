#ifndef INCLUDE_JET_INPUTS
#define INCLUDE_JET INPUTS

#include <Arduino.h>
#include "constants.h"
#include "Encoder.h"

struct Switch {
  Switch(uint8_t pin): pin(pin) { }
  void init();
  void read();
  bool didChange();
  bool isPressed();
  const uint8_t pin;
  uint8_t state = HIGH;
  uint8_t lastState = LOW;
};

struct EncoderKnob {
  EncoderKnob(uint8_t p0, uint8_t p1): enc(Encoder(p0, p1)) { }
  void init();
  void read();
  bool didChange();
  bool wrap = true;//or clamp
  unsigned int max = 5;
  unsigned int pos = 0;
  unsigned int lastPos = 255;
  private:
  Encoder enc;
};

struct Inputs {
  EncoderKnob progSelect = EncoderKnob(KNOB_A_PIN_0, KNOB_A_PIN_1);
  EncoderKnob progMod = EncoderKnob(KNOB_B_PIN_0, KNOB_B_PIN_1);
  Switch horn = Switch(SWITCH_A_PIN);
  Switch s1 = Switch(SWITCH_B_PIN);
  Switch s2 = Switch(SWITCH_C_PIN);
  void init();
  void read();
};

#endif 

