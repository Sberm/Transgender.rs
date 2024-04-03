# Transgender

A minimalistic TUI file explorer with **zero dependancy** except libc, written by a [ranger](https://github.com/ranger/ranger) hater.

Works on Linux & MacOS

![](img/trans-img.png)

![](img/trans.gif)

## quick start

before compilation, make sure you have rust's environment installed, if not, goto [HERE](https://www.rust-lang.org/tools/install)

compile
```
make
make install
```

refresh your shell configuration, or open up a new terminal window
```bash
# linux
source ~/.bashrc
# mac 
source ~/.bash_profile
```

enjoy
```bash
ts
```

### Features:

* file exploring
* cd to directories
* search directories

### keys:

| keys                             | function                                     |
| :---:                            | :---:                                        |
| hjkl(vim bindings) or arrow keys | scroll/enter/exit a directory                |
| o or \<ENTER\>                   | go to directory under cursor                 |
| s                                | go to current directory(in left side window) |
| q                                | quit                                         |
| /                                | search                                       |
| n                                | jump to next search match                    |

if you want to uninstall Transgender
```
make uninstall
```


```
WHY DO I MAKE TRANS?

In my opinion trans is simpler than ranger. Creeping
featurism, growing size made ranger less attractive. 
Also, ranger is not a good software, it is filled
with bad designs that irritate its users.
```
