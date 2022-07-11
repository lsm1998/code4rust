comma:= ,
empty:=
space:= $(empty) $(empty)
str:= $(subst $(comma),$(space),${name})

all:
	@for i in ${str} ; do \
		echo $$i; \
	done

