/* A Bison parser, made by GNU Bison 3.8.2.  */

/* Bison interface for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2021 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */

#ifndef YY_YY_SRC_FIGHT_TAB_H_INCLUDED
# define YY_YY_SRC_FIGHT_TAB_H_INCLUDED
/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int yydebug;
#endif

/* Token kinds.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    YYEMPTY = -2,
    YYEOF = 0,                     /* "end of file"  */
    YYerror = 256,                 /* error  */
    YYUNDEF = 257,                 /* "invalid token"  */
    PLAYER = 258,                  /* PLAYER  */
    PLAYERSTATES = 259,            /* PLAYERSTATES  */
    INPUTS = 260,                  /* INPUTS  */
    IDENTIFIER = 261,              /* IDENTIFIER  */
    NUM = 262,                     /* NUM  */
    DELAY = 263,                   /* DELAY  */
    TYPE = 264,                    /* TYPE  */
    STRING = 265,                  /* STRING  */
    BEGIN_BLOCK = 266,             /* BEGIN_BLOCK  */
    END_BLOCK = 267,               /* END_BLOCK  */
    WHILE = 268,                   /* WHILE  */
    IF = 269,                      /* IF  */
    ELSE = 270,                    /* ELSE  */
    IS = 271,                      /* IS  */
    THEN = 272,                    /* THEN  */
    IN = 273,                      /* IN  */
    HIT = 274,                     /* HIT  */
    WITH = 275,                    /* WITH  */
    FOR = 276,                     /* FOR  */
    BLOCKS = 277,                  /* BLOCKS  */
    USES = 278,                    /* USES  */
    COMMA = 279,                   /* COMMA  */
    LINEBREAK = 280,               /* LINEBREAK  */
    TWOPS = 281,                   /* TWOPS  */
    WAIT = 282,                    /* WAIT  */
    ADD = 283,                     /* ADD  */
    SUB = 284,                     /* SUB  */
    MUL = 285,                     /* MUL  */
    DIV = 286,                     /* DIV  */
    ASSIGN = 287,                  /* ASSIGN  */
    EQUALS = 288,                  /* EQUALS  */
    GREATER = 289,                 /* GREATER  */
    LESS = 290,                    /* LESS  */
    AND = 291,                     /* AND  */
    OR = 292,                      /* OR  */
    NOT = 293,                     /* NOT  */
    RIGHTP = 294,                  /* RIGHTP  */
    LEFTP = 295,                   /* LEFTP  */
    PRINT = 296,                   /* PRINT  */
    OTHER = 297                    /* OTHER  */
  };
  typedef enum yytokentype yytoken_kind_t;
#endif

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
union YYSTYPE
{
#line 12 "src/fight.y"

    int int_val;
    char *str_val;
    char *type;
    char *inputs;
    char *identifier;
    char *playerstates;
    char *player;
    char *str;

#line 117 "src/fight.tab.h"

};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE yylval;


int yyparse (void);


#endif /* !YY_YY_SRC_FIGHT_TAB_H_INCLUDED  */
