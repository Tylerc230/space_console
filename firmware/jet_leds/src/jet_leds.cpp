#include <Arduino.h>
#include "program_runner.h"
ProgramRunner runner = ProgramRunner();
void setup() {
  Serial.begin(57600);
  delay( 1000 ); // power-up safety delay
  runner.init();
}

void loop() {
  runner.update();
}

