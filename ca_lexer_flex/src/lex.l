%{
int chars = 0;
int words = 0;
int lines = 0;
%}
%option reentrant
%%
[a-zA-Z]+ { words++; chars += strlen(yytext); }
\n        { chars++; lines++; }
.         { chars++; }
%%
