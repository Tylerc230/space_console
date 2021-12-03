#define BRIGHTNESS  64
#define LED_TYPE    WS2812B	
#define COLOR_ORDER GRB
#define UPDATES_PER_SECOND 100

#define TOTAL_LEDS 143
#define MAX_LEDS 30
//These are arduino pins which printed on board
#define L_NOSE_LED_PIN 2
#define R_NOSE_LED_PIN 3
#define L_WING_LED_PIN 4
#define R_WING_LED_PIN 5
#define TAIL_LED_PIN 6

#define SWITCH_A_PIN 7
#define SWITCH_B_PIN 8
#define SWITCH_C_PIN 9

//I think these are know push buttons
#define SWITCH_D_PIN 15 
#define SWITCH_E_PIN 16 

//prog select, red headers
#define KNOB_A_PIN_0 10
#define KNOB_A_PIN_1 11
//mod select, blue headers
//pin 13 doesn't work
#define KNOB_B_PIN_0 12
  //Could move this to pin 17(A3) would need to update board
#define KNOB_B_PIN_1 17

#define NOSE_LED_COUNT 27
#define WING_LED_COUNT 29

#define TAIL_LED_COUNT 30

#define HORN_OUT 17

