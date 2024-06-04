%{
#include <stdio.h>
#include <string.h>
extern FILE *yyin;
int yylex(void);
void yyerror(const char *s);
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

%token OTHER

%%

program
    :BEGIN_BLOCK TWOPS IDENTIFIER block
    |LINEBREAK
    |
    ;

block
    :LINEBREAK statement block
    |END_BLOCK
    ;

statement
    : assignment_st
    | declare_st
    | say_st
    | while_st
    | if_st
    | wait_st
    |
    ;

wait_st
    : WAIT player_exp
    ;

assignment_st
    : IDENTIFIER ASSIGN player_exp
    ;

declare_st
    : TYPE IDENTIFIER ASSIGN player_exp
    | TYPE IDENTIFIER
    ;

say_st
    :SAY player_exp
    ;

while_st
    : WHILE TWOPS player_exp LINEBREAK block END_BLOCK
    ;

if_st
    : IF player_exp THEN block ELSE block END_BLOCK
    ;

player_exp
    : bool_exp
    | players
    ;


bool_exp
    : bool_term OR bool_exp
    | bool_term
    ;

bool_term
    : rel_exp AND bool_term
    | rel_exp
    ;

rel_exp
    : expression EQUALS rel_exp
    | expression GREATER rel_exp
    | expression LESS rel_exp
    | expression
    ;

expression
    : term ADD expression
    | term SUB expression
    | term 
    ;

term
    : factor MUL term
    | factor DIV term
    | factor
    ;

factor
    : DELAY
    | INPUTS
    | ATTACK
    | NUM
    | IDENTIFIER
    | ADD factor
    | SUB factor
    | NOT factor
    | RIGHTP player_exp LEFTP
    ;

players
    : PLAYER IS HIT WITH IDENTIFIER
    | PLAYER IN PLAYERSTATES FOR factor
    ;
%%

void yyerror(const char *s) {
    fprintf(stderr, "Error: %s\n", s);
}

int main(int argc, char *argv[]) {
    if (argc != 2) {printf("Numero errado de args");}
    FILE *input_file = fopen(argv[1], "r");
    if (!input_file) {
        printf("file not found");
        return 1;
    }
    yyin = input_file;
    yyparse();
    fclose(input_file);
    return 0;
}
