# keyboard-walker

[![Build Status](https://travis-ci.com/Merik88/keyboard-walker.svg?branch=master)](https://travis-ci.com/Merik88/keyboard-walker)

keyboard-walker is a tool for generating passwords based on keyboard keys.

## Installation

Download the executable from the [releases](https://github.com/Merik88/keyboard-walker/releases) page and put it wherever you want.

## Usage

Run this for help with arguments:

```bash
keyboard-walker -h
```

Example run without any arguments (default):

```bash
$ keyboard-walker
§12
123
234
345
456
...
```

## Roadmap

### To-Do

- Run the strategy in different orders¹
- A prepend option, currently it's only appending the words
- Configure the depth for the vertical strategy, so it can output a "qawsed" combination for example
- The format of the keyboard file will change to json and json arrays, similar to <http://www.keyboard-layout-editor.com>

¹ It currently starts on the top left corner in a left to right & top to bottom order.

## Resources

<http://www.unicode.org/cldr/charts/latest/keyboards/layouts/index.html>

<http://www.keyboard-layout-editor.com>

<https://wpengine.com/unmasked/>

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.txt) file or <http://opensource.org/licenses/MIT> for details
