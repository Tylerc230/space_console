#include "RainProgram.h"
#include "../leds.h"
#include "../inputs.h"
#include "Arduino.h"
#include "colorpalettes.h"
#include "colorutils.h"
#include "pixeltypes.h"
struct Drop {
  LEDStripID stripId;
  uint8_t currentIndex = 0;
  uint8_t delayStart = 0;
  CRGB color = CRGB::White;
  Drop() {
    init();
  }

  void init() {
    currentIndex = 0;
    stripId = random(NUM_STRIPS);
    delayStart = random(100);
  }

  void update(uint8_t max) {
    if (delayed()) {
      delayStart--;
      return;
    }
    currentIndex++;
    if (currentIndex >= max) {
      init();
    }
  }

  bool delayed() {
    return delayStart > 0;
  }
};

void RainProgram::init(Inputs & inputs, AirplaneLEDs & airplane) {
  inputs.progMod.wrap = false;
  inputs.progMod.max = 30;
  createDrops(3, false, false);
}

void RainProgram::update(Inputs & inputs, AirplaneLEDs & airplane) {
  airplane.clearLEDs();
  if (inputs.progMod.didChange() || inputs.s1.didChange() || inputs.s2.didChange()) {
    uint8_t numDrops = map(inputs.progMod.pos, 0, 30, 5, 120);
    createDrops(inputs.progMod.pos * 4, inputs.s1.isPressed(), inputs.s2.isPressed());
  }
  for (int i = 0; i < numDrops; i++) {
    Drop & drop = drops[i];
    LEDStripID id = drop.stripId;
    LEDStrip & strip = airplane.stripForId(id);
    if (!drop.delayed()) {
      strip.setLED(drop.currentIndex, drop.color);
    }
    drop.update(strip.numLEDs);
  }
}
CRGBPalette16 emiColors =
{
    CRGB::Amethyst,
    CRGB::Aqua,
    CRGB::Aquamarine,
    CRGB::Lavender,
    
    CRGB::BlueViolet,
    CRGB::CadetBlue,
    CRGB::CornflowerBlue,
    CRGB::DarkMagenta,
    
    CRGB::DarkOrchid,
    CRGB::DeepPink,
    CRGB::LavenderBlush,
    CRGB::DodgerBlue,

    CRGB::DarkTurquoise,
    CRGB::Fuchsia,
    CRGB::HotPink,
    CRGB::LightPink
};

void RainProgram::createDrops(uint8_t count, bool s1Pressed, bool s2Pressed) {
  CRGBPalette16 currentPalette;
  if (!s1Pressed && !s2Pressed) {
    fill_solid( currentPalette, 16, CRGB::White);
  } else if (s1Pressed && !s2Pressed) {
    currentPalette = RainbowColors_p;
  } else if (!s1Pressed && s2Pressed) {
    currentPalette = LavaColors_p;
  } else {
    currentPalette = emiColors;
  }
  if (drops != NULL) {
    delete drops;
    drops = NULL;
  }
  numDrops = count;
  drops = new Drop[count];
  for (int i = 0; i < count; i++) {
    CRGB color = ColorFromPalette( currentPalette, random(255), 255, LINEARBLEND);
    drops[i].color = color;
  }
}

RainProgram::~RainProgram() {
  if (drops != NULL) {
    delete drops;
    drops = NULL;
  }
}

