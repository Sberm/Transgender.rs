TRANS=transgender
V=

function build () {
  cargo build --release --target=$1
  if [ $1 = "x86_64-unknown-linux-gnu" ]; then
	  V=$(target/$1/release/$TRANS -v | grep version | awk '{print $NF}')
  fi
  cp target/$1/release/$TRANS build/$TRANS-$V-$1
}

if [ ! -d build ]; then
	mkdir build
fi

# x86_64 needs to be the first
build x86_64-unknown-linux-gnu
build i686-unknown-linux-gnu
build aarch64-unknown-linux-gnu
build loongarch64-unknown-linux-gnu
build riscv64gc-unknown-linux-gnu

# how to use the release binary
#    cp transgender-<version>-<arch> /usr/local/bin
#    echo 'eval "$(transgender --sh)"' >> ~/.bashrc # ~/.zshrc on zsh
