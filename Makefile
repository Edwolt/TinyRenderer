### Copilation
CC      := clang

### Folders
F_BIN     := bin
F_SRC     := .

### Files
OUT     := renderer.out
SRC     := $(wildcard $(F_SRC)/*.c)
OBJ     := $(SRC:$(F_SRC)/%.c=$(F_BIN)/%.o)

### Flags
LIBS    := -lm
# Flags to generate the program
CFLAGS  := $(LIBS)
# Flags to generate a optimized program
FFLAGS  := $(LIBS) -O3 -march=native -w
# Flags to compile a program to debug
DFLAGS  := $(LIBS) -g -Wall -Wextra -pedantic -Werror=implicit-function-declaration -fsanitize=address
# Flags to compile a program that allow to use gprof
GFLAGS  := $(LIBS) -pg $(FFLAGS)
# Flags that actually will be used to compile the program
FLAGS   := $(DFLAGS)

# Others Flags
RMFLAGS := -vf

### Actions
settings:
	echo "Compiler:   $(CC)"
	echo "Flags:      $(FLAGS)"
	echo "Folder SRC: $(F_SRC)"
	echo "source:     $(SRC)"
	echo "Folder Bin: $(F_BIN)"
	echo "objects:    $(OBJ)"

subdirs:
	mkdir -p $(F_SRC)
	mkdir -p $(F_BIN)

all: compile

run: $(OUT)
	./$(OUT)

compile: subdirs $(OUT)

clean:
	rm $(RMFLAGS) $(OBJ) 
	rm $(RMFLAGS) $(OUT) 

### OUT
$(OUT): $(OBJ)
	$(CC) -o $(OUT) $(OBJ) $(FLAGS)

$(F_BIN)/%.o: $(F_SRC)/%.c
	$(CC) -c -o $@ $< $(FLAGS)
