#------------------------------------------------------------------------------
SOURCE=main.cpp
MYPROGRAM=rtxon
LIBS=-lSDL2
CC=g++

#------------------------------------------------------------------------------

all: $(MYPROGRAM)

$(MYPROGRAM): $(SOURCE)
	$(CC) $(SOURCE) -o $(MYPROGRAM) $(LIBS)

clean:
	rm -f $(MYPROGRAM) *.ppm