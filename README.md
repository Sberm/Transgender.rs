# Transgender.rs

[![version][version-badge]][version-url]

[version-badge]: https://img.shields.io/github/v/release/sberm/Transgender.rs
[version-url]: https://github.com/Sberm/Transgender.rs/releases

A minimalistic TUI file explorer with **minimal dependencies**, written by a [ranger](https://github.com/ranger/ranger) hater, works on Linux & MacOS.

![](img/trans_.jpg)

![](img/lucius.jpg)

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

paste this shell script function into your shell configuration file, its location depends on which shell you use (`~/.bashrc`, `~/.bash_profile`, `source ~/.zshrc`, `~/.config/fish/config.fish`)
```
ts() {
  cd "$(transgender 3>&1 1>&2 2>&3 3>&- | tail -n 1)"
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

**if `~/.tsrc` doesn't exist, you need to create one**

Supports changing the editor and the theme.

Example of `~/.tsrc`:

```tsrc
# Set your preferred editor
editor = emacs

# Set your preferred theme
theme = dark
```

#### Editor

The specified editor will open when `o` or `ENTER` is pressed while the cursor is on a file.

By default, the editor is set to `vi`.

#### Theme

Currently, there are only two themes available: `dark` and `trans`, the default
theme is `trans`.

Image of the `dark` theme:

![](img/dark.jpg)

Other themes:

* [lucius](https://github.com/jonathanfilip/lucius)
* [acme](https://github.com/ianyepan/acme-emacs-theme)
* [sakura](https://imgur.com/a/5YhgVMG)
* [vscode](https://github.com/Mofiqul/vscode.nvim)

<br/>

### Search

Now supports `>> regular expression <<`

`Trangender.rs` performs a search whenever a key is pressed

Due to the use of the `regex` crate, the executable can be a bit bloated (
~474KB). If you don't want regular expression feature, simply switch to the
vanilla version by checking out to the vanilla branch and building
`Transgender.rs`.

```bash
git checkout vanilla
make
make install
```

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

- [ ] tests

- [x] Read utf-8 input

- [x] Better full-width character handling

- [x] Config file to change text editor(default to vi)
