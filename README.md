Chip8 Emulator
==============

This is another [Chip8](https://en.wikipedia.org/wiki/CHIP-8) realisation written on [Rust](https://www.rust-lang.org) 
for education purposes (in both creating simple virtual machines and rust).
It uses [pixels](https://github.com/parasyte/pixels) for video, which I found ideally suitable for this project.
 
Build
-----

This project requires stable rust installed, to build use

```shell script
$ cargo build
```

Run
---

To run emulator use

```shell script
$ cargo run -- <path to ROM file>
```

Play
----

Use following control keys for play:

```
Chip-8 Keypad              Keyboard
  +-+-+-+-+                +-+-+-+-+
  |1|2|3|C|                |1|2|3|4|
  +-+-+-+-+                +-+-+-+-+
  |4|5|6|D|                |Q|W|E|R|
  +-+-+-+-+       =>       +-+-+-+-+
  |7|8|9|E|                |A|S|D|F|
  +-+-+-+-+                +-+-+-+-+
  |A|0|B|F|                |Z|X|C|V|
  +-+-+-+-+                +-+-+-+-+
```

One can download Chip8 ROMs pack [here](https://web.archive.org/web/20130702032522/http://www.chip8.com/downloads/Chip-8%20Pack.zip). 

Test
----

This realisation tested against two test ROMs:
1. [Chip8 test ROM](https://github.com/corax89/chip8-test-rom)
2. [BC_Test](https://slack-files.com/T3CH37TNX-F3RF5KT43-0fb93dbd1f) ([explanation](https://slack-files.com/T3CH37TNX-F3RKEUKL4-b05ab4930d))

*NOTE*: There is at last one not documented error code in BC_Test with code `E 34`, 
in my case it pointed to wrong implementation of Fx55 and/or Fx65 operation codes.

TODO
----
[] Fix some minor bugs
[] Add audio
[] Add tests for Chip-8
[] Cleanup code

Useful Resources
---------
- [Chip 8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap)
- [Building a CHIP-8 emulator [C++]](https://austinmorlan.com/posts/chip8_emulator/)
- [Mastering CHIP-8](http://mattmik.com/files/chip8/mastering/chip8.html)
- [Emulation Basics: Write your own Chip 8 Emulator/Interpreter](http://omokute.blogspot.com/2012/06/emulation-basics-write-your-own-chip-8.html)
- [Chip8 web site](https://web.archive.org/web/20130903140414/http://chip8.com/?page=73)
- [David Winter's CHIP-8 emulation page](http://www.pong-story.com/chip8/)
- [EMUBook](http://emubook.emulation64.com)
