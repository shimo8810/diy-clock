#include <MsTimer2.h>

const byte D1 = 6;
const byte D2 = 7;
const byte D3 = 5;
const byte D4 = 8;

const byte SRCLK = 9;
const byte RCLK = 10;
const byte SER = 11;

const byte DIGITS[] = {
  D1,
  D2,
  D3,
  D4
};

const byte NUM[] = {
  0xFC, // 0
  0x60, // 1
  0xDA, // 2
  0xF2, // 3
  0x66, // 4
  0xB6, // 5
  0xBE, // 6
  0xE0, // 7
  0xFE, // 8
  0xF6, // 9
};

byte MIN = 21;
byte SEC = 0;

void timerFire() {
  SEC += 1;
  if (SEC >= 60) {
    SEC = 0;
    MIN += 1;
  }
  if (MIN > 60) { 
    MIN = 0;
  }
}

void setup() {
  for (auto i = 0; i < 4; ++i) {
    pinMode(DIGITS[i], OUTPUT);
    digitalWrite(DIGITS[i], LOW);
  }

  pinMode(SER, OUTPUT);
  pinMode(RCLK, OUTPUT);
  pinMode(SRCLK, OUTPUT);

  byte b = 0x0;
  digitalWrite(RCLK, LOW);
  shiftOut(SER, SRCLK, LSBFIRST, b);
  digitalWrite(RCLK, HIGH);

  pinMode(LED_BUILTIN, OUTPUT);

  MsTimer2::set(1000, timerFire);
  MsTimer2::start();
}



void loop() {
    for (auto i = 3; i >= 0; i--) {
      byte n = 0;
      switch (i) {
        case 0:
          n = MIN / 10;
          break;
        case 1:
          n = MIN % 10;
          break;
        case 2:
          n = SEC / 10;
          break;
        case 3:
          n = SEC % 10;
          break;
        default:
          n = 0;
      }

      digitalWrite(RCLK, LOW);
      shiftOut(SER, SRCLK, LSBFIRST, NUM[n]);
      digitalWrite(RCLK, HIGH);

      digitalWrite(DIGITS[i], HIGH);
      delay(2);

      digitalWrite(RCLK, LOW);
      shiftOut(SER, SRCLK, LSBFIRST, 0x0);
      digitalWrite(RCLK, HIGH);

      digitalWrite(DIGITS[i], LOW);
    }
}
