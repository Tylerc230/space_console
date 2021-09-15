#include "ColorProgram.h"
#include "../leds.h"
#include "../inputs.h"
#include "Arduino.h"
#include "colorpalettes.h"
#include "colorutils.h"
#include "pixeltypes.h"
struct Sparkle {
  uint8_t pixel_idx;//0 - 142
  uint8_t brightness;
};
struct SparkleEffect {
    Sparkle *sparkles = NULL;
    const uint8_t numSparkles = 0;
    const uint8_t speed = 0;
    SparkleEffect(uint8_t count, uint8_t speed): numSparkles(count), speed(speed) {
      sparkles = new Sparkle[count];
      for (int i = 0; i < count; i++) {
        sparkles[i].pixel_idx = random(MAX_LEDS - 1);
        sparkles[i].brightness = random(127);
      }
    }

    ~SparkleEffect() {
      if (sparkles != NULL) {
        delete sparkles;
      }
    }
    
    void update() {
      for (int i = 0; i < numSparkles; i++) {
        sparkles[i].brightness += speed;
        if (sparkles[i].brightness >= 127) {
          sparkles[i].brightness = 0;
          sparkles[i].pixel_idx = random(MAX_LEDS - 1);
        }
      }
    }

    CRGB color(uint8_t index) {
      uint8_t sin_wave = (sin8_avr(sparkles[index].brightness) - 127);
      uint8_t brightness = 255 - sin_wave;
      //if (index == 0) {
        //Serial.println();
      //}
      CRGB color = CRGB::White;
      color.fadeLightBy(brightness);
      return color;
    }

    uint8_t pixelIndex(uint8_t index) {
      return sparkles[index].pixel_idx;
    }


};
void ColorProgram::init(Inputs & inputs, AirplaneLEDs & airplane) {
  inputs.progMod.max = NUM_PALETTES;
  airplane.clearLEDs();
}

void ColorProgram::update(Inputs & inputs, AirplaneLEDs & airplane) {
  if (inputs.s1.didChange() || inputs.s2.didChange()) {
    if (sparkles != NULL) {
      delete sparkles;
      sparkles = NULL;
    }
    if (inputs.s1.isPressed() && inputs.s2.isPressed()) {

      sparkles = new SparkleEffect(10, 4);
    } else if (inputs.s1.isPressed() && !inputs.s2.isPressed()) {
      sparkles = new SparkleEffect(7, 3);

    } else if (!inputs.s1.isPressed() && inputs.s2.isPressed()) {
      sparkles = new SparkleEffect(4, 2);
    }
  }

  CRGBPalette16 palettes [NUM_PALETTES] = {RainbowColors_p, LavaColors_p, OceanColors_p, CloudColors_p, ForestColors_p, HeatColors_p, PartyColors_p};
  CRGBPalette16 currentPalette = palettes[inputs.progMod.pos];
  for( int i = 0; i < MAX_LEDS; i++) {
    CRGB color = ColorFromPalette( currentPalette, index + i, 128, LINEARBLEND);
    airplane.setAll(i, color);
  }
  if (sparkles != NULL) {
    for (int i = 0; i < sparkles->numSparkles; i++) {
      uint8_t idx = sparkles->pixelIndex(i);
      CRGB color = sparkles->color(i);
      airplane.setAll(idx, color);
    }
    sparkles->update();
  }
  index ++;
}
