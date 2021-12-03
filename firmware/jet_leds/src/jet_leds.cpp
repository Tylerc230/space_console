#include <Arduino.h>
#include "program_runner.h"
ProgramRunner runner = ProgramRunner();
void setup() {
  Serial.begin(9600);
  delay( 3000 ); // power-up safety delay
  runner.init();
}

void loop() {
  runner.update();
}

