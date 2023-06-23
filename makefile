
OCAMLMAKEFILE = ./OCamlMakefile

PATH_SRC	= src

SOURCES		= $(addprefix $(PATH_SRC)/, \
				hello.ml \
				main.ml \
			)
PACKS		=
RESULT		= ft_turing
THREADS		= no

# From https://github.com/mmottl/ocaml-makefile/tree/master
include $(OCAMLMAKEFILE)

fclean : clean

re : fclean byte-code