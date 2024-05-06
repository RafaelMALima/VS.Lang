%{
#include <stdio.h>
#include <string.h>
extern FILE *yyin;
int yyflex(void);
void yyerror(char *s);
%}

%union{
    char* input;
    char* identifier;
    int number;
}

%token BEGIN_BLOCK END_BLOCK WHILE IF ELSE IS IN HIT WITH FOR USES NUM INPUTS TYPE TWOPS IDENTIFIER PLAYER WAIT PLAYERSTATES THEN
%token ADD SUB MUL DIV ASSIGN EQUALS GREATER LESS AND OR NOT RIGHTP LEFTP COMMA LINEBREAK SAY

%token <input> ATTACK
%token <number> DELAY
%token <number> DAMAGE
%token <number> INT

%%

program
    :BEGIN_BLOCK TWOPS block END_BLOCK program
    |BEGIN_BLOCK TWOPS block END_BLOCK
    ;

block
    :statement LINEBREAK block
    |
    ;

statement
    : assignment_st
    | declare_st
    | say_st
    | while_st
    | if_st
    | wait_st
    ;

wait_st
    : WAIT expression
    ;

assignment_st
    : IDENTIFIER ASSIGN bool_exp
    ;

declare_st
    : TYPE IDENTIFIER ASSIGN bool_exp
    ;

say_st
    :SAY bool_exp
    ;

while_st
    : WHILE TWOPS bool_exp LINEBREAK block END_BLOCK
    ;

if_st
    : IF player_exp THEN block ELSE block END_BLOCK
    ;

player_exp
    : players
    | bool_exp
    ;


bool_exp
    : bool_term OR bool_term bool_exp
    ;

bool_term
    : rel_exp AND rel_exp
    |rel_exp
    ;

rel_exp
    : rel_exp EQUALS expression
    | rel_exp GREATER expression
    | rel_exp LESS expression
    | expression
    ;

expression
    : expression ADD term
    | expression SUB term
    | term 
    ;

term
    : term MUL factor
    | term DIV factor
    | factor
    ;

factor
    : DELAY
    | INPUTS
    | IDENTIFIER
    | ADD factor
    | SUB factor
    | NOT factor
    | RIGHTP expression LEFTP
    ;

players
    : PLAYER IS HIT WITH IDENTIFIER
    | PLAYER IN PLAYERSTATES FOR factor
    ;
%%

void yyerror(const char *s) {
    fprintf(stderr, "Error: %s\n", s);
}

int main(void) {
    if (yyparse() == 0) {
        printf("Parsing complete!\n");
    } else {
        printf("Parsing failed.\n");
    }
    return 0;
}
