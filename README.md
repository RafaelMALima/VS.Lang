# LogComp-Linguagem

## Motivation
The goal of this language is for it to be a simple and intuitive way for 
players in the fighting game community to easily theorize, annotate and simulate potential combos,
sequences, blockstring and OKI, withou needing to actually perform said combos.

### EBNF da linguagem
```
LOCALS = {"begin", ":", BLOCK, "end" };

BLOCK = {STATEMENT};

STATEMENT = ( "λ" | ASSIGNMENT | SAY | WHILE | IF | WAIT), "\n" ;

ASSIGNMENT = (TYPE, IDENTIFIER, "=", EXPRESSION ) | ("attack", IDENTIFIER, "=" ,EXPRESSION, ",",EXPRESSION,",",EXPRESSION,",",EXPRESSION,",", EXPRESSION) ;
SAY = "say", EXPRESSION ;
WHILE = "while", ":", BOOL_EXP, "\n", "λ", { ( STATEMENT ), "λ" }, "end";
IF = "if", BOOL_EXP, "then", "\n", "λ", { ( STATEMENT ), "λ" }, ( "λ" | ( "else", "\n", "λ", { ( STATEMENT ), "λ" })), "end" ;
WAIT = "wait", EXPRESSION;

BOOL_EXP = BOOL_TERM, { ("or"), BOOL_TERM } ;

BOOL_TERM = REL_EXP, { ("and"), REL_EXP } ;

REL_EXP = EXPRESSION, { ("==" | ">" | "<"), EXPRESSION } ;

EXPRESSION = TERM, { ("+" | "-"), TERM } ;

TERM = FACTOR, { ("*" | "/"), FACTOR } ;

FACTOR = DELAY | INPUT | IDENTIFIER | PLAYERS | (("+" | "-" | "not"), FACTOR ) | "(", EXPRESSION, ")" | "read", "(", ")" ;

PLAYERS = ("ENEMY" | "PLAYER"), (PLAYERSTATES | ("is", ("hit", "with", IDENTIFIER) | ("in", PLAYERSTATES, "for", EXPRESSION)) | "uses", IDENTIFIER);

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
begin: COMBO
    delay first_input_startup = 10f
    input first_input = 412K
    delay enemy_hitstun = 10f
    delay player_recovery = 9f
    damage attack_damage = 10
    attack player_attack = first_input_startup, player_recovery, first_input, attack_damage, enemy_hitstun;

    PLAYER uses player_attack
    ENEMY is hit with player_attack

    wait player_recovery

    if: player_recovery > enemy_hitstun
        ENEMY is in HITSTUN
        say "player is plus"
    end

    sequence first loop = 5S 2f 236HS
    delay player_recovery = 10f
    delay initial_hitstun = 15f
    delay enemy_hitstun = initial_hitstun
    dleay hitstun_falloff = 1f

    int how_many_loops = 0;

    while: ENEMY is in HITSTUN
        // do first_loop
        how_many_loops += 1

        hitstun_accumulation += hitstun_accumulation
        hitstun_falloff += 1
        enemy_hitstun -= hitstun_falloff
        if: player_recovery > enemy_hitstun
            ENEMY is not in HITSTUN
        end
    end

    say how_many_loops
end

begin: BLOCKSTRING
    delay first_input_startup = 8f
    input first_input = 6P
    delay enemy_blockstun = 10f
    delay player_recovery = 6f
    damage attack_damage = 10
    attack player_attack = first_input_startup, player_recovery, first_input, attack_damage, enemy_blockstun;

    PLAYER uses player_attack
    ENEMY blocks player_attack

    wait player_recovery

    if: ENEMY is in BLOCKSTUN and PLAYER is IDLE
        PLAYER uses player_attack;
    end
end

```
