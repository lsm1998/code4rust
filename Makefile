comma:= ,
empty:=
space:= $(empty) $(empty)
val:= ${name}
str:= $(subst $(comma),$(space),$(val))

all:
	@for i in ${str} ; do \
		echo $$i; \
	done

