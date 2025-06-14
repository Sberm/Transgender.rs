# Transgender.rs

<p align="center">
  <img style="height:200px;" src="img/logo.png"/>
</p>

[![version][version-badge]][version-url]

[version-badge]: https://img.shields.io/github/v/release/sberm/Transgender.rs
[version-url]: https://github.com/Sberm/Transgender.rs/releases

A minimalistic file explorer with **minimal dependencies**, written by a [ranger](https://github.com/ranger/ranger) hater, works on Linux & MacOS.

### Features:

* File exploration
* File searching
* Switching directories
* Opening files
* Multiple themes

> [!TIP]
> Documentation can be found [here](https://sberm.cn/trans).

<br/>

![](img/catppuccin.png)

## Quick start

### Installation

#### Linux & MacOS
```sh
curl -fsSL https://raw.githubusercontent.com/Sberm/Transgender.rs/refs/heads/main/scripts/install.sh | bash
```

> Currently only supports Linux and MacOS of x86_64 and Arm64 architectures.

#### Arch Linux

[transgender](https://aur.archlinux.org/packages/transgender) is available as a package in the [AUR](https://aur.archlinux.org). you can install it using your preferred AUR helper. example:

```sh
paru -S transgender
```

### Compiling from source

clone the repo
```bash
git clone https://github.com/Sberm/Transgender.rs.git
```

before compilation, make sure you have rust's environment installed, if not, goto [HERE](https://www.rust-lang.org/tools/install)
<br/>

compile & install
```bash
cd Transgender.rs
make
make install
```

### Shell configuration

Currently only supports `bash` and `zsh`.

paste this shell script function into your shell configuration file, its location depends on which shell you use (`~/.bashrc`, `~/.bash_profile`, `source ~/.zshrc`)
```
eval "$(transgender --sh)"
```

refresh your shell configuration, or open up a new terminal window
```bash
# bash
source ~/.bashrc
# or
source ~/.bash_profile

# zsh
source ~/.zshrc
```

launch `Transgender.rs` with:
```bash
ts
```

<br/>

### Usage

| keys                             | function                                                            |
| :---:                            | :---:                                                               |
| hjkl(vim bindings) or arrow keys | scroll/enter/exit a directory                                       |
| o or \<ENTER\>                   | go to directory/open file under cursor                              |
| i                                | go to the current directory (in the left window)                    |
| q                                | quit                                                                |
| /                                | search                                                              |
| ?                                | reverse search                                                      |
| n / N                            | jump to the next/previous search match                              |
| Ctrl + U / Ctrl + D              | half pageup/pagedown                                                |
| (when in search mode) arrow keys | up/down for search history, left/right for editing the search query |

To open up a directory with trans
```bash
ts /usr/lib/
ts ..
ts /root
```

<br/>

### Config file

located at

```bash
~/.tsrc
```

**if `~/.tsrc` doesn't exist, you need to create one**

It supports changing the editor (opened with `o` or `ENTER`) and the theme.

An example of `~/.tsrc`:

```tsrc
# Set your preferred editor
editor = emacs

# 'open = ' has the same effect as 'editor = '
open = emacs

# Set your preferred theme
theme = dark
```

Because `o` and `ENTER` can both open files, you can specify their commands separately.
```tsrc
# The line below will be overridden by the following lines
editor = code

# Command to run after pressing 'o', this overrides 'editor = ' and 'open = '
o = open

# Command to run after pressing 'enter', this overrides 'editor = ' and 'open = '
enter = vim
```

Command line arguments are supported as well.
```tsrc
# Option value with whitespace, such as '--title "I Love Emacs"' is not supported yet
enter = emacs -nbc --no-desktop --title I-Love-Emacs
o = code --sync on --disable-lcd-text --disable-gpu
```

### Theme

The default theme is `lucius`.

`dark` theme:

![](img/dark.jpg)

Other themes:

* [catppuccin](https://imgur.com/a/mK2Toin)
* [trans](https://imgur.com/a/m4dmLig)
* [lucius](https://github.com/jonathanfilip/lucius)
* [acme](https://github.com/ianyepan/acme-emacs-theme)
* [sakura](https://imgur.com/a/5YhgVMG)
* [vscode](https://github.com/Mofiqul/vscode.nvim)
* [jesus](https://imgur.com/a/creZltw)
* [lucius-l](https://imgur.com/a/RyImZYW)

The theme name is case-insensitive; e.g., Catppuccin and catppuccin both work.

### Editor

The specified editor will open when `o` or `ENTER` is pressed while the cursor is on a
file.

The default editor is `vi`; You can change it to `vscode` by adding this line into your
`~/.tsrc` file:
```tsrc
editor = code
```

`o` and `ENTER` can open different editors and programs, for example, press `o` to open `vim`,
and `enter` to open `emacs`. See the guide above for how to configure them using `.tsrc`.

<br/>

### Searching

Searching is case-insensitive by default, use `<search term>\C` to search
case-sensitively. e.g. `README\C` will return the search result for `README.md`.

`Transgender.rs` supports **regular expression**.

`Trangender.rs` performs a search whenever a key is pressed in search mode

Due to the use of the `regex-lite` crate, the executable can be a bit bloated (
~504KB). If you don't need the regular expression feature, simply switch to the
vanilla version by checking out to the vanilla branch and building
`Transgender.rs` from there.

```bash
git checkout vanilla
make
make install
```

<br/>

### Uninstallation

if you want to uninstall `Transgender.rs`, run
```
make uninstall
```

and delete the `eval "$(transgender --sh)"` in your `.bashrc` or `.zshrc` file

<br/>

```
WHY DID I MAKE TRANS?

In my opinion, Trans is simpler than ranger. Creeping
featurism and growing size have made ranger less
attractive. Additionally, ranger is not a good
software; it is filled with bad designs that irritate
its users.
```

<br/>

#### Chris the Sea Slug

On the logo of `Transgender.rs` is Chris the Sea Slug. He likes to use Clang, and his favorite rapper is LL Cool J.
By the way, sea slugs are hermaphrodites, meaning they are both male and female.
