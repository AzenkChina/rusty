CROSS    = arm-none-eabi-
OBJ      = led.o
LINKOBJ  = led.o
LIBS     = 
INCS     = 
BIN      = libled.a
CFLAGS   = $(INCS) -march=armv7e-m
RM       = rm -f

.PHONY: all all-before all-after clean clean-custom

all: all-before $(BIN) all-after

clean: clean-custom
	${RM} $(OBJ) $(BIN)

$(BIN): $(LINKOBJ)
	$(CROSS)ar r $(BIN) $(LINKOBJ)
	$(CROSS)ranlib $(BIN)

led.o: led.c
	$(CROSS)gcc -c led.c -o led.o	$(CFLAGS)
