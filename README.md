# Transgender

A TUI file explorer, written by a ranger hater.

## quick start

compile
```
make
```

put this line into your `~/.bashrc`
```shellscript
alias trans="transgender 2>/tmp/trans && cd \"\`tail -n 1 /tmp/trans\`\""
```

and then enjoy
```bash
trans
```

keys:

|keys|function|
|:---:|:---:|
|o|go to directory under cursor|
|\<ENTER\>|go to directory under cursor|
|s|go to current directory(in left side window)|
|q|quit|
