ifeq ($(DEBUG), 1)
	Q =
	msg =
	CFLAGS = -g -std=c11
else
	Q = @
	msg = @printf '	%-8s %s%s\n' "$(1)" "$(2)" "$(if $(3), $(3))";
	CFLAGS = -O2 -std=c11
endif

TRANS_ALIAS ?= "alias ts=\"transgender 2>/tmp/trans && cd \\\"\\\`tail -n 1 /tmp/trans\\\`\\\"\""

all: 
	$(call msg,TRANS,BUILDING)
	$(Q)cargo build --release
	$(Q)echo $(TRANS_ALIAS) >> ~/.bashrc
	$(Q)echo $(TRANS_ALIAS) >> ~/.bash_profile
	$(call msg,TRANS,BUILD-SUCCEED)

install:
	$(call msg,INSTALL)
	$(Q)sudo cp ./target/release/transgender /usr/local/bin/transgender

uninstall:
	$(call msg,UNINSTALL)
	$(Q)sudo rm -f /usr/local/bin/transgender

clean:
	$(call msg,CLEAN)
	$(Q)sudo rm -f ./target/release/transgender
