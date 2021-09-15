#include "program_runner.h"
#include "colorutils.h"
#include "constants.h"
#include "programs/ColorProgram.h"
#include "programs/RainProgram.h"
#define NUM_PROG 2


void ProgramRunner::init() {
  inputs.init();
  airplane.init();
  inputs.progSelect.max = NUM_PROG;
}

void ProgramRunner::update() {
  inputs.read();
  Serial.print(inputs.s1.state);Serial.print(" ");
  Serial.print(inputs.s2.state);Serial.print(" ");
  Serial.print(inputs.horn.state);Serial.print(" ");
  Serial.print(inputs.progSelect.pos);Serial.print(" ");
  Serial.print(inputs.progMod.pos);Serial.print(" ");
  Serial.println();
  if (inputs.horn.isPressed()) {
    airplane.playTone(128);
  } else {
    airplane.toneOff();
  }
  int selection = inputs.progSelect.pos;
  if (selection != currentProgramIndex) {
    if (currentProgram != NULL) {
      delete currentProgram;
    }
    currentProgram = createProgram(selection);
    currentProgramIndex = selection;
    currentProgram->init(inputs, airplane);
  }
  currentProgram->update(inputs, airplane);
  FastLED.show();
  FastLED.delay(1000 / UPDATES_PER_SECOND);
}

Program * ProgramRunner::createProgram(uint8_t progIndex) {
  Serial.println(progIndex);
  switch (progIndex) {
    case 0:
      return new ColorProgram();
    case 1:
      return new RainProgram();

  };
}


