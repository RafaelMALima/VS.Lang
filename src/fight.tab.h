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
    BEGIN_BLOCK = 258,             /* BEGIN_BLOCK  */
    END_BLOCK = 259,               /* END_BLOCK  */
    WHILE = 260,                   /* WHILE  */
    IF = 261,                      /* IF  */
    ELSE = 262,                    /* ELSE  */
    IS = 263,                      /* IS  */
    IN = 264,                      /* IN  */
    HIT = 265,                     /* HIT  */
    WITH = 266,                    /* WITH  */
    FOR = 267,                     /* FOR  */
    USES = 268,                    /* USES  */
    NUM = 269,                     /* NUM  */
    INPUTS = 270,                  /* INPUTS  */
    TYPE = 271,                    /* TYPE  */
    TWOPS = 272,                   /* TWOPS  */
    IDENTIFIER = 273,              /* IDENTIFIER  */
    PLAYER = 274,                  /* PLAYER  */
    WAIT = 275,                    /* WAIT  */
    PLAYERSTATES = 276,            /* PLAYERSTATES  */
    THEN = 277,                    /* THEN  */
    ADD = 278,                     /* ADD  */
    SUB = 279,                     /* SUB  */
    MUL = 280,                     /* MUL  */
    DIV = 281,                     /* DIV  */
    ASSIGN = 282,                  /* ASSIGN  */
    EQUALS = 283,                  /* EQUALS  */
    GREATER = 284,                 /* GREATER  */
    LESS = 285,                    /* LESS  */
    AND = 286,                     /* AND  */
    OR = 287,                      /* OR  */
    NOT = 288,                     /* NOT  */
    RIGHTP = 289,                  /* RIGHTP  */
    LEFTP = 290,                   /* LEFTP  */
    COMMA = 291,                   /* COMMA  */
    LINEBREAK = 292,               /* LINEBREAK  */
    SAY = 293,                     /* SAY  */
    ATTACK = 294,                  /* ATTACK  */
    DELAY = 295,                   /* DELAY  */
    DAMAGE = 296,                  /* DAMAGE  */
    INT = 297,                     /* INT  */
    OTHER = 298                    /* OTHER  */
  };
  typedef enum yytokentype yytoken_kind_t;
#endif

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
union YYSTYPE
{
#line 9 "src/fight.y"

    char* input;
    char* identifier;
    int number;

#line 113 "src/fight.tab.h"

};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE yylval;


int yyparse (void);


#endif /* !YY_YY_SRC_FIGHT_TAB_H_INCLUDED  */
