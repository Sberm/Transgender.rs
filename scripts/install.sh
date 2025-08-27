ARCH="$(uname -m)"
SYS="$(uname -s)"
V="1.5.6"
TRANS="transgender"
BINARY=
TMP="/tmp/transgender"
BIN_PATH="/usr/local/bin"

if [[ $ARCH =~ "arm64" ]]; then
  ARCH="aarch64"
fi

if [[ $SYS =~ "Linux" ]]; then
  BINARY=$TRANS-$V-$ARCH-linux-gnu
elif [[ $SYS =~ "Darwin" ]]; then
  BINARY=$TRANS-$V-$ARCH-apple-darwin
else
  echo "Binary for $ARCH-$SYS not found, please build trans from source (guide: https://sberm.cn/trans/)"
  exit 1
fi

log() {
  printf "\033[38;5;81m* \033[39m$1"
}

URL="https://github.com/Sberm/Transgender.rs/releases/download/$V/$BINARY"
fetched=0
CURL=curl
WGET=wget
if command -v $CURL >/dev/null 2>&1; then
  log "Fetching: $URL\n\n"
  $CURL -fsSL --progress-bar $URL > $TMP
  fetched=1
elif command -v $WGET >/dev/null 2>&1; then
  log "Fetching: $URL\n\n"
  $WGET $URL -O $TMP
  fetched=1
else
  log "$CURL/$WGET not found, please install either one"
fi
if [ $fetched -eq 1 ]; then
  log "Fetched [$BINARY]\n\n"
  sudo install $TMP $BIN_PATH
  rm -f $TMP
  log "\033[38;5;81m[Success]\033[39m Installed at $BIN_PATH/$TRANS\n\n"
  log "Add this line to your shell configuration file:\n\n  \033[38;5;81meval \"\$(transgender --sh)\"\n\n\033[39mand do \033[38;5;81msource ~/.bashrc \033[39mor \033[38;5;81msource ~/.zshrc\033[39m .\n"
fi
