#include "inputs.h"
void Switch::init() {
  state = HIGH;
  pinMode(pin, INPUT_PULLUP);
}

void Switch::read() {
  lastState = state;
  state = digitalRead(pin);
}

bool Switch::didChange() {
  return state != lastState;
}

bool Switch::isPressed() {
  return state == LOW;
}

void EncoderKnob::init() {
  pos = 0;
  enc.write(pos);
}

void EncoderKnob::read() {
  lastPos = pos;
  int total = enc.read() / 4;
  if (wrap) {
    if (total < 0) {
      total = max - 1;
    }
    pos = (total) % max;
  } else {//clamp
    if (total < 0) {
      total = 0;
    }
    pos = min(total, max);
  }
}

bool EncoderKnob::didChange() {
  return lastPos != pos;
}


void Inputs::init() {
  horn.init();
  s1.init();
  s2.init();
  progSelect.init();
  progMod.init();
}

void Inputs::read() {
  horn.read();
  s1.read();
  s2.read();
  progSelect.read();
  progMod.read();
}


