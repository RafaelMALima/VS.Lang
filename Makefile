fight: src/fight.y src/fight.l
	bison -d -Wcounterexamples src/fight.y -o src/fight.tab.c
	flex -o src/lex.yy.c src/fight.l
	gcc -o fight src/fight.tab.c src/lex.yy.c -lfl

clean:
	rm -rf fight
	rm -rf src/lex.yy.c
	rm -rf src/fight.tab.c
	rm -rf src/fight.tab.h

rebuild:
	make clean && make fight

all:
	make rebuild
	cargo build --release
	cp target/release/LogComp-Linguagem fight-interpreter
