# This build script should be run on MacOS
# to install necessary compilers, do:
# brew install messense/macos-cross-toolchains/aarch64-unknown-linux-gnu

TRANS=transgender
V=

function build () {
  cargo build --release --target=$1
  # change below to your architecture
  if [ $1 = "aarch64-apple-darwin" ]; then
    V=$(target/$1/release/$TRANS -v | grep version | awk '{print $NF}')
  fi
  cp target/$1/release/$TRANS build/$TRANS-$V-$2
}

if [ ! -d build ]; then
  mkdir build
fi

# rustup target add <target>
# your local architecture needs to be the first
build aarch64-apple-darwin          aarch64-apple-darwin
build x86_64-unknown-linux-gnu      x86_64-linux-gnu
build aarch64-unknown-linux-gnu     aarch64-linux-gnu
build x86_64-apple-darwin           x86_64-apple-darwin

#### How to use the release binary
#~~~sh
#sudo cp transgender-<version>-<arch> /usr/local/bin/transgender
#echo 'eval "$(transgender --sh)"' >> ~/.bashrc
#source ~/.bashrc
## on zsh
#echo 'eval "$(transgender --sh)"' >> ~/.zshrc
#source ~/.zshrc
#~~~
