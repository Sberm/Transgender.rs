# Transgender

A TUI file explorer, written by a ranger hater.

## quick start

put this line in ~/.bashrc
```shellscript
alias trans="/bin/transgender 2>/tmp/trans && cd \`tail -n 1 /tmp/trans\`"
```

then
```bash
trans
```

keys:

|keys|function|
|:---:|:---:|
|o|go to directory under cursor|
|<ENTER>|go to directory under cursor|
|s|go to current directory(in left side window)|
|q|quit|
