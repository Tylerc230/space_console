#ifndef INCLUDE_PROGRAM
#define INCLUDE_PROGRAM
struct AirplaneLEDs;
struct Inputs;
struct Program {
  virtual void init(Inputs & inputs, AirplaneLEDs & airplane) = 0;
  virtual void update(Inputs & inputs, AirplaneLEDs & airplane) = 0;
};
#endif

