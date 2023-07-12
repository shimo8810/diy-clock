#!/usr/bin/env bash

avrdude -c diecimila -p t2313 -U flash:w:$1:e