%{
#include <stdio.h>
#include <string.h>
#include "fight.tab.h"
//#include <stdio.h>
void showError;

%}

NUMBER ([0-9])+
IDENT  [a-zA-Z_][a-zA-Z0-9_]*
PLAYER ("ENEMY"|"PLAYER")
VALID_INPUTS ([a-d]|u|d|l|r|(L|M|H)(P|K)|←|→|↑|↓|P|K|S|HS|D|[0-9]|\+])+
DELAY ([0-9])+f

TYPE ("delay"|"damage"|"attack"|"int")

PLAYERSTATE ("HITSTUN"|"BLOCKSTUN"|"GROUNDED"|"IDLE")
%%

"begin"         {return BEGIN_BLOCK;}
"end"           {return END_BLOCK;}
"while"         {return WHILE;}
"if"            {return IF;}
"else"          {return ELSE;}
"is"            {return IS;}
"then"          {return THEN;}
"in"            {return IN;}
"hit"           {return HIT;}
"with"          {return WITH;}
"for"           {return FOR;}
"uses"          {return USES;}
"wait"          {return WAIT;}

"+"             {return ADD;}
"-"             {return SUB;}
"*"             {return MUL;}
"/"             {return DIV;}
"="             {return ASSIGN;}
"=="            {return EQUALS;}
">"             {return GREATER;}
"<"             {return LESS;}
"and"           {return AND;}
"or"            {return OR;}
"not"           {return NOT;}
"("             {return RIGHTP;}
")"             {return REFTP;}
","             {return COMMA;}
"\\n"           {return LINEBREAK;}
":"             {return TWOPS}

{TYPE}          {return TYPE;}
{NUMBER}        {return NUM;}
{IDENT}         {return IDENTIFIER;}
{VALID_INPUTS}  {return INPUTS;}
{PLAYERSTATE}   {return PLAYERSTATES;}
{PLAYER}        {return PLAYER;}
{DELAY}         {return DELAY;}
.               {showError(); return(OTHER);}

void ShowError(){
    printf("Input not found");
}

%%

int yywrap(){
    return 1;
}
