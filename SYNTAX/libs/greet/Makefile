CC       = gcc.exe
CP       = busybox.exe cp
OBJ      = greet.o
LINKOBJ  = greet.o
LIBS     = 
INCS     = 
BIN      = libgreet.a
CFLAGS   = $(INCS) -std=c99
RM       = rm.exe -f

.PHONY: all all-before all-after clean clean-custom

all: all-before $(BIN) all-after

clean: clean-custom
	${RM} $(OBJ) $(BIN)

$(BIN): $(LINKOBJ)
	ar r $(BIN) $(LINKOBJ)
	ranlib $(BIN)

all-after:
	$(CP) $(BIN) ../

greet.o: greet.c
	$(CC) -c greet.c -o greet.o	$(CFLAGS)
