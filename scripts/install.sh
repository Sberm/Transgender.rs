ARCH="$(uname -m)"
SYS="$(uname -s)"
V="1.5.7"
TRANS="transgender"
BINARY=
TMP="/tmp/transgender"
BIN_PATH="/usr/local/bin"
CYAN="\033[38;5;81m"
NORMAL="\033[39m"

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
  printf "${CYAN}* ${NORMAL}$1"
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
  log "${NORMAL}[Success]${NORMAL} Installed at $BIN_PATH/$TRANS\n\n"
  log "Add this line to your shell configuration file (${CYAN}.bashrc${NORMAL} or ${CYAN}.zshrc${NORMAL}):\n\n  ${CYAN}eval \"\$(transgender --sh)\"\n\n${NORMAL}and do ${CYAN}source ~/.bashrc ${NORMAL}or ${CYAN}source ~/.zshrc${NORMAL} .\n"
fi
