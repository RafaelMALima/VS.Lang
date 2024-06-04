%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern FILE *yyin;

int yylex(void);
void yyerror(char *s);
%}

%union {
    int int_val;
    char *str_val;
    char *type;
    char *inputs;
    char *identifier;
    char *playerstates;
    char *player;
    char *str;
}

%token <player> PLAYER
%token <playerstates> PLAYERSTATES
%token <inputs> INPUTS
%token <identifier> IDENTIFIER
%token <int_val> NUM DELAY
%token <type> TYPE
%token <str> STRING
%token BEGIN_BLOCK END_BLOCK WHILE IF ELSE IS THEN IN HIT WITH FOR BLOCKS USES COMMA LINEBREAK TWOPS
%token WAIT ADD SUB MUL DIV ASSIGN EQUALS GREATER LESS AND OR NOT RIGHTP LEFTP PRINT

%token OTHER

%%

program
    : sequence program
    | sequence
    ;

sequence
    : newlines BEGIN_BLOCK TWOPS IDENTIFIER block END_BLOCK 
    ;


newlines
    : LINEBREAK
    |
    ;

block
    : newlines
    | statement block
    ;


statement
    : IDENTIFIER ASSIGN expression
    | PRINT bool_expression
    | WHILE TWOPS bool_expression block END_BLOCK
    | IF TWOPS bool_expression block END_BLOCK
    | PLAYER player_statement
    | WAIT bool_expression
    ;


player_statement
    : USES bool_expression COMMA bool_expression
    | HIT WITH bool_expression COMMA bool_expression COMMA bool_expression
    | BLOCKS bool_expression COMMA bool_expression
    ;

variable
    : INPUTS
    | PLAYERSTATES
    | NUM
    | DELAY
    | STRING
    ;

bool_expression
    : bool_term
    | bool_expression OR bool_term
//    | player bool_expression
//    | player
    ;

bool_term
    : rel_exp
    | bool_term AND rel_exp
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
    : variable
    | IDENTIFIER
    | player
    | RIGHTP bool_expression LEFTP
    | unop factor
    ;

unop
    : ADD
    | SUB
    | NOT
    ;

player
    : PLAYER IN PLAYERSTATES
    ;

%%


int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Expected args: %s <input_file>\n", argv[0]);
        return 1;
    }
    FILE *input_file = fopen(argv[1], "r");
    if (!input_file) {
        fprintf(stderr, "Error: could not open file %s\n", argv[1]);
        return 1;
    }
    yyin = input_file;
    yyparse();

    fclose(input_file);
    printf("Finished parsing file!");
    return 0;
}

void yyerror(char *s) {
    fprintf(stderr, "error: %s\n", s);
}
