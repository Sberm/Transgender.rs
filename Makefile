ifeq ($(DEBUG), 1)
	Q =
	msg =
	CFLAGS = -g -std=c11
else
	Q = @
	msg = @printf '	%-8s %s%s\n' "$(1)" "$(2)" "$(if $(3), $(3))";
	CFLAGS = -O2 -std=c11
endif

TRANS_ALIAS ?= "alias trans=\"transgender 2>/tmp/trans && cd \\\"\\\`tail -n 1 /tmp/trans\\\`\\\"\""
all: 
	$(Q)cargo build --release 2>/dev/null
	$(Q)cp ./target/release/transgender /usr/local/bin/transgender
	$(Q)echo $(TRANS_ALIAS) >> ~/.bashrc
	$(Q)echo $(TRANS_ALIAS) >> ~/.bash_profile
	$(call msg,TRANS,BUILD-SUCCEED)
