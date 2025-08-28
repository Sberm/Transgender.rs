ifeq ($(DEBUG), 1)
	Q =
	msg =
else
	Q = @
	msg = @printf '	%-8s %s%s\n' "$(1)" "$(2)" "$(if $(3), $(3))";
endif

INSTALL_PATH := "/usr/local/bin"
TRANS        := "transgender"
TARGET       := "target/release/${TRANS}"

all: 
	$(call msg,TRANS,STARTS-BUILDING)
	$(Q)cargo build --release
	$(call msg,TRANS,BUILD-SUCCEED)

install:
	$(call msg,INSTALL,${INSTALL_PATH}/${TRANS})
	$(Q)sudo install ${TARGET} ${INSTALL_PATH}

uninstall:
	$(call msg,UNINSTALL,${INSTALL_PATH}/${TRANS})
	$(Q)sudo rm -f ${INSTALL_PATH}/${TRANS}

clean:
	$(call msg,CLEAN,${TARGET})
	$(Q)sudo rm -f ${TARGET}
