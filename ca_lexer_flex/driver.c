
#include "lex.h"
#include <stdio.h>
int main() {
  yyscan_t scanner;
  YY_BUFFER_STATE buf;
  yylex_init(&scanner);
//   yyset_in(fopen("test.z", "r"), scanner);
//   buf = yy_scan_string("replace me with the string youd like to scan", scanner);
//   printf("12312312312313\n\n\n\n");
//   yy_switch_to_buffer(buf, scanner);
  yylex(scanner);
//   printf("12312312312313\n\n\n\n");
//   yy_delete_buffer(buf, scanner);
//   yylex_destroy(scanner);
  return 0;
}
int yywrap(yyscan_t scanner) { return 0; }