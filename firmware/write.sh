#!/usr/bin/env bash
avrdude -c diecimila -p t2313 -U flash:w:./target/avr-attiny2313/release/blink-attiny2313.elf:e -U lfuse:w:0xff:m
