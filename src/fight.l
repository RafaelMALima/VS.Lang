%{
#include <stdio.h>
#include <string.h>
#include "fight.tab.h"
//#include <stdio.h>

%}

NUMBER ([0-9])+
IDENT  [a-zA-Z_][a-zA-Z0-9_]*
PLAYER ("ENEMY"|"PLAYER")
VALID_INPUTS ([a-d]|u|d|l|r|(L|M|H)(P|K)|←|→|↑|↓|P|K|S|HS|D|[0-9]|\+])+
DELAY ([0-9])+f
STRING (\".*\")

TYPE ("delay"|"damage"|"attack"|"int")
NEWLINES ([\n\r]|\\n)+
WHITESPACE [ \t\n\s]+
PLAYERSTATE ("HITSTUN"|"BLOCKSTUN"|"GROUNDED"|"IDLE")


%%

{WHITESPACE}    ;
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
"print"         {return PRINT;}
"blocks"        {return BLOCKS;}

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
")"             {return LEFTP;}
","             {return COMMA;}
{NEWLINES}      {return LINEBREAK;}
":"             {return TWOPS;}

{TYPE}          {return TYPE;}
{NUMBER}        {return NUM;}
{VALID_INPUTS}  {return INPUTS;}
{PLAYERSTATE}   {return PLAYERSTATES;}
{PLAYER}        {return PLAYER;}
{DELAY}         {return DELAY;}
{IDENT}         {return IDENTIFIER;}
{STRING}        {return STRING;}
.               {printf("Token not found \n"); return OTHER;};

%%

int yywrap(){
    return 1;
}
