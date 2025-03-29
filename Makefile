ifeq ($(DEBUG), 1)
	Q =
	msg =
else
	Q = @
	msg = @printf '	%-8s %s%s\n' "$(1)" "$(2)" "$(if $(3), $(3))";
endif

install_path := /usr/local/bin
target := target/release/transgender

all: 
	$(call msg,TRANS,STARTS-BUILDING)
	$(Q)cargo build --release
	$(call msg,TRANS,BUILD-SUCCEED)

install:
	$(call msg,INSTALL,${install_path}/transgender)
	$(Q)sudo install ${target} ${install_path}

uninstall:
	$(call msg,UNINSTALL)
	$(Q)sudo rm -f ${install_path}/transgender

clean:
	$(call msg,CLEAN)
	$(Q)sudo rm -f ${target}
