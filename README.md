# LogComp-Linguagem

## Motivation
The goal of this language is for it to be a simple and intuitive way for 
players in the fighting game community to easily theorize, annotate and simulate potential combos,
sequences, blockstring and OKI, withou needing to actually perform said combos.

### EBNF da linguagem
```
PROGRAM = { LOCALS }

LOCALS = {"begin", ":", BLOCK, "end" };

BLOCK = {STATEMENT};

STATEMENT = ( "λ" | ASSIGNMENT | PRINT | WHILE | IF | WAIT | PLAYER_STATEMENT), "\n" ;

ASSIGNMENT = ( IDENTIFIER, "=", EXPRESSION ) | ("attack", IDENTIFIER, "=" ,EXPRESSION, ",",EXPRESSION,",",EXPRESSION,",",EXPRESSION,",", EXPRESSION) ;
PRINT = "PRINT", EXPRESSION ;
WHILE = "while", ":", BOOL_EXP, "\n", "λ", { ( STATEMENT ), "λ" }, "end";
IF = "if", ":", BOOL_EXP, "\n", "λ", { ( STATEMENT ), "λ" }, ( "λ" | ( "else", "\n", "λ", { ( STATEMENT ), "λ" })), "end" ;
WAIT = "wait", BOOL_EXP;
PLAYER_STATEMENT = ("PLAYER"|"ENEMY"), (("uses" BOOL_EXP "," BOOL_EXP) | ("hit with " BOOL_EXP "," BOOL_EXP "," BOOL_EXP) | (BLOCKS BOOL_EXP "," BOOL_EXP))

BOOL_EXP = BOOL_TERM, { ("or"), BOOL_TERM } ;

BOOL_TERM = REL_EXP, { ("and"), REL_EXP } ;

REL_EXP = EXPRESSION, { ("==" | ">" | "<"), EXPRESSION } ;

EXPRESSION = TERM, { ("+" | "-"), TERM } ;

TERM = FACTOR, { ("*" | "/"), FACTOR } ;

FACTOR = DELAY | INPUT | IDENTIFIER | PLAYERS | (("+" | "-" | "not"), FACTOR ) | "(", EXPRESSION, ")" | "read", "(", ")" ;

PLAYERS = ("ENEMY" | "PLAYER"), (("in" PLAYERSTATES);

IDENTIFIER = LETTER, { LETTER | DIGIT | "_" } ;

TYPE = ( "delay" | "damage" | "attack" );

NUMBER = DIGIT, { DIGIT } ;
DELAY = NUMBER, "f";
INPUT = {VALID_INPUTS};
VALID_INPUTS = ( "a" | "b" | "c" | "d" | "u"| "d"| "l"|"r"|"LP"|"MP"|"HP"|"LK"|"MK"|"HK"|"← "| "→"| "↑"| "↓"| "P"| "K"|"S"|"HS"|DIGIT | "+"|);
PLAYERSTATES = (HITSTUN | BLOCKSTUN | GROUNDED | IDLE);
LETTER = ( "a" | "..." | "z" | "A" | "..." | "Z" ) ;
DIGIT = ( "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" ) ;
```
### Exemplo da linguagem
```
begin : COMBO
    first_input_startup = 10f
    first_input = 412K
    enemy_hitstun = 10f
    player_recovery = 9f
    attack_damage = 10

    PLAYER uses first_input, first_input_startup
    ENEMY hit with first_input, enemy_hitstun, attack_damage 

    wait player_recovery

    if: player_recovery > enemy_hitstun
        print "player is plus"
    end
end

begin: BLOCKSTRING
    first_input_startup = 8f
    first_input = 6P
    enemy_blockstun = 10f
    player_recovery = 6f
    attack_damage = 10

    PLAYER uses first_input, first_input_startup
    wait player_recovery
    ENEMY blocks first_input, enemy_blockstun

    wait player_recovery

    second_input = 623HS
    second_input_startup = 13f
    second_input_blockstun = 20f

    if: ENEMY in BLOCKSTUN and PLAYER in IDLE
        PLAYER uses second_input, second_input_startup
        ENEMY blocks second_input, second_input_blockstun
    end
end
```

## Compilando e rodando o projeto
Para rodar esse projeto, é necessário que se tenha instalado flex, o bison, a libgcc (parser), e o cargo (interpretador).
Como o parser foi feito com o flex/bison, e o interpretador com Rust, é necessário essa menagerie de programas

### Usando o parser:  

```
make all
```
Na raiz do projeto

O arquivo fight, presente na raiz do projeto, é o programa resultante.

Segue como usá-lo
```
./fight <arquivo .fight alvo>
```

### Rodando o interpretador

Para interpretar im programa da linguagem, basta rodar o programa via o cargo, como no exemplo

```
cargo run <arquivo .fight alvo>
```

