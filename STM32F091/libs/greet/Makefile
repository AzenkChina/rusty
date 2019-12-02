CROSS    = arm-none-eabi-
CP       = busybox cp
OBJ      = greet.o
LINKOBJ  = greet.o
LIBS     = 
INCS     = 
BIN      = libgreet.a
CFLAGS   = $(INCS) -march=armv6-m
RM       = rm -f

.PHONY: all all-before all-after clean clean-custom

all: all-before $(BIN) all-after

clean: clean-custom
	${RM} $(OBJ) $(BIN)

$(BIN): $(LINKOBJ)
	$(CROSS)ar r $(BIN) $(LINKOBJ)
	$(CROSS)ranlib $(BIN)

all-after:
	$(CP) $(BIN) ../

greet.o: greet.c
	$(CROSS)gcc -c greet.c -o greet.o	$(CFLAGS)
