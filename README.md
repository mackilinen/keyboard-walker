# keyboard-walker

[![Build Status](https://travis-ci.com/Merik88/keyboard-walker.svg?branch=master)](https://travis-ci.com/Merik88/keyboard-walker)

keyboard-walker is a tool for generating passwords based on keyboard key sequences.

## Installation

Download the executable from the [releases](https://github.com/Merik88/keyboard-walker/releases) page and put it wherever you want.

## Usage

keyboard-walker <1arg> <2arg> <3arg...Narg>

- 1arg: length of the generated keyboard sequence
- 2arg: kayboard walk strategy (Horizontal or Vertical)
- 3arg: words to append sequences to

```bash
$ keyboard-walker 3 Horizontal firstword secondword
firstwordqwe
firstwordwer
...
secondwordqwe
secondwordwer
...
```

## Roadmap

### To-Do

- Show the generated sequences first
- Read word list from a file
- Possibility to use a range for the sequence length (min length & max length)
- Read keyboard layout from file
- Run the strategy in different orders¹
- Windows binary²

¹ It currently starts on the top left corner in a left to right & top to bottom order.

² This is currently configured for travis but it's not working as it's early release functionality.

## Keyboard layout resources

<http://www.unicode.org/cldr/charts/latest/keyboards/layouts/index.html>

<http://www.keyboard-layout-editor.com>
