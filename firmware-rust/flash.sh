#! /bin/zsh

set -e

if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "usage: $0 <path-to-binary.elf>" >&2
    exit 1
fi

if [ "$#" -lt 1 ]; then
    echo "$0: Expecting a .elf file" >&2
    exit 1
fi

pushd avr
cargo build
popd
avrdude -C ~/Library/Arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/etc/avrdude.conf -q  -v -b 57600 -patmega328p -carduino -P/dev/cu.usbserial-2 -D "-Uflash:w:$1"
