# 🗣️ Say

Say is a Rust-based command-line utility that allows you to display text on the console. It works similarly to the Unix echo command

## Installation
To use Say, you'll need to have Rust installed on your system. Once Rust is installed, you can build Say with the following steps:

1. Clone the Say repository:
```bash
git clone https://github.com/CarlosEduardoL/say.git
```
2. Change into the cloned directory:
```bash
cd say
```
3. Build the project with Cargo:
```bash
cargo build --release
```
4. The built binary will be available in the target/release directory. You can move it to your desired location:
```bash
mv target/release/say /usr/local/bin/
```

## Usage
Say is similar to the Unix echo command. Here are some examples of how to use it:
```bash
say Hello, World!
```
This will display the text "Hello, World!" on the console.

```bash
say -e "Hello,\t\t\t\tWorld!\nHow are you today?\n"
```
This will display the text "Hello," followed by four horizontal tabs, followed by the word "World!", and then a newline character. After the newline, it will display the text "How are you today?" on a new line.

```bash
say -e "Hello, \x1b[31mWorld!\x1b[0m"
```
This will display the text "Hello, World!" on the console, with the word "World!" colored in red.

## Contributing
Say is a small project created for learning purposes, but contributions are always welcome! If you have an idea for an improvement, feel free to create a pull request.

## License
Say is released under the MIT license. See the LICENSE file for details.
