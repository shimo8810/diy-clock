#!/usr/bin/env bash

avrdude -c diecimila -p t2313 -B 4800 -U hfuse:r:-:h -U lfuse:r:-:h
