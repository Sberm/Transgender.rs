# Transgender

[![version][version-badge]][version-url]

[version-badge]: https://img.shields.io/github/v/release/sberm/Transgender.rs
[version-url]: https://github.com/Sberm/Transgender.rs/releases

A minimalistic TUI file explorer with **zero dependencies** except libc, written by a [ranger](https://github.com/ranger/ranger) hater.

Works on Linux & MacOS

![](img/trans-img.png)

![](img/trans.gif)

## quick start

clone the repo
```bash
git clone https://github.com/Sberm/Transgender.rs.git
```

before compilation, make sure you have rust's environment installed, if not, goto [HERE](https://www.rust-lang.org/tools/install)

compile & install
```bash
cd Transgender.rs
make
make install
```

paste this function to your shell configuration file, depends on what shell you use (`~/.bashrc`, `~/.bash_profile`, `source ~/.zshrc`, `~/.config/fish/config.fish`)
```
function ts() {
  cd $(transgender 3>&1 1>&2 2>&3 3>&- | tail -n 1)
}
```

refresh your shell configuration, or open up a new terminal window
```bash
# bash
source ~/.bashrc
# or
source ~/.bash_profile

# zsh
source ~/.zshrc

# fish
source ~/.config/fish/config.fish

```

enjoy
```bash
ts
```

<br/>

### keys

| keys                             | function                                     |
| :---:                            | :---:                                        |
| hjkl(vim bindings) or arrow keys | scroll/enter/exit a directory                |
| o or \<ENTER\>                   | go to directory/open file under cursor       |
| i                                | go to current directory(in left side window) |
| q                                | quit                                         |
| /                                | search                                       |
| n                                | jump to next search match                    |

<br/>

### config file
located at
```bash
~/.tsrc
```

**if `~/.tsrc` doesn't exist, user has to create one**

currently only supports changing editor

change what's after equal sign to your favorite editor
```
editor = emacs
```

that will be the editor opened after hitting `o` or `ENTER` when cursor is on a file

the default editor is `vi`

<br/>

### uninstall

if you want to uninstall Transgender
```
make uninstall
```

<br/>

### Features:

* file exploring
* cd to directories
* search directories

```
WHY DO I MAKE TRANS?

In my opinion trans is simpler than ranger. Creeping
featurism, growing size made ranger less attractive. 
Also, ranger is not a good software, it is filled
with bad designs that irritate its users.
```

<br/>

### Todo:

- [ ] More tests

- [x] Read utf-8 input

- [x] Better full-width character handling

- [x] Config file to change text editor(default to vi)
