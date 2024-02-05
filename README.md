# introduction

A simple command line tool to copy text to clipboard and manage a clipboard stack.

# Installation

Enable `xclip` feature to copy to clipboard:

`cargo install clipse --features xclip`

Otherwise it will just print the text to stdout.

# Usage

To copy, `clipse -c`

To add to stack, `clipse -a <text>`

Once in the stack, press `dd` to delete, `hjkl` to navigate, and `q` to quit.

# Known Issues

If the program is exited abruptly, the terminal might not be restored to its original state. 

# Update Log

- 1.2.2: (5 Feb 2024) I consider the project to be feature complete for my current use cases, PRs and issues are welcome!
