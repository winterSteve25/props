#include <tree_sitter/parser.h>

enum TokenType {
  INDENT,
  DEDENT,
  NEWLINE
};

void* tree_sitter_props_external_scanner_create() {
	return NULL;
}

void tree_sitter_props_external_scanner_destroy(void* payload) {
}

unsigned tree_sitter_my_language_external_scanner_serialize(
  void *payload,
  char *buffer
) {
}

void tree_sitter_my_language_external_scanner_deserialize(
  void *payload,
  const char *buffer,
  unsigned length
) {
}

bool tree_sitter_my_language_external_scanner_scan(
  void *payload,
  TSLexer *lexer,
  const bool *valid_symbols
) {
}
