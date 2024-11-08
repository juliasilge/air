use crate::kind_src::KindsSrc;

pub const R_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (";", "SEMICOLON"),
        (",", "COMMA"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("?", "WAT"),
        ("~", "TILDE"),
        ("<-", "ASSIGN"),
        ("<<-", "SUPER_ASSIGN"),
        (":=", "WALRUS"),
        ("->", "ASSIGN_RIGHT"),
        ("->>", "SUPER_ASSIGN_RIGHT"),
        ("=", "EQUAL"),
        ("|", "OR"),
        ("&", "AND"),
        ("||", "OR2"),
        ("&&", "AND2"),
        ("<", "LESS_THAN"),
        ("<=", "LESS_THAN_OR_EQUAL_TO"),
        (">", "GREATER_THAN"),
        (">=", "GREATER_THAN_OR_EQUAL_TO"),
        ("==", "EQUAL2"),
        ("!=", "NOT_EQUAL"),
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("*", "MULTIPLY"),
        ("/", "DIVIDE"),
        ("^", "EXPONENTIATE"),
        ("**", "EXPONENTIATE2"),
        ("|>", "PIPE"),
        (":", "COLON"),
        ("::", "COLON2"),
        (":::", "COLON3"),
        ("$", "DOLLAR"),
        ("@", "AT"),
        ("!", "BANG"),
        ("...", "DOTS"),
    ],
    keywords: &[
        "function",
        "for",
        "in",
        "while",
        "repeat",
        "if",
        "else",
        "return",
        "next",
        "break",
        "true",
        "false",
        "null",
        "inf",
        "nan",
        "na_logical",
        "na_integer",
        "na_double",
        "na_complex",
        "na_character",
    ],
    literals: &[
        "R_INTEGER_LITERAL",
        "R_DOUBLE_LITERAL",
        "R_COMPLEX_LITERAL",
        "R_STRING_LITERAL",
    ],
    tokens: &[
        "NEWLINE",
        "WHITESPACE",
        "IDENT",
        "COMMENT",
        "BACKSLASH",
        "DOTDOTI",
        "SPECIAL",
    ],
    nodes: &[
        "R_ROOT",
        "R_IDENTIFIER",
        "R_DOTS",
        "R_DOT_DOT_I",
        "R_UNARY_EXPRESSION",
        "R_BINARY_EXPRESSION",
        "R_EXTRACT_EXPRESSION",
        "R_NAMESPACE_EXPRESSION",
        "R_FUNCTION_DEFINITION",
        "R_PARAMETERS",
        "R_PARAMETER_LIST",
        "R_IDENTIFIER_PARAMETER",
        "R_DOTS_PARAMETER",
        "R_DEFAULT_PARAMETER",
        "R_IF_STATEMENT",
        "R_ELSE_CLAUSE",
        "R_FOR_STATEMENT",
        "R_WHILE_STATEMENT",
        "R_REPEAT_STATEMENT",
        "R_BRACED_EXPRESSIONS",
        "R_PARENTHESIZED_EXPRESSION",
        "R_CALL",
        "R_CALL_ARGUMENTS",
        "R_ARGUMENT_LIST",
        "R_NAMED_ARGUMENT",
        "R_UNNAMED_ARGUMENT",
        "R_DOTS_ARGUMENT",
        "R_HOLE_ARGUMENT",
        "R_EXPRESSION_LIST",
        "R_INTEGER_VALUE",
        "R_DOUBLE_VALUE",
        "R_COMPLEX_VALUE",
        "R_STRING_VALUE",
        "R_RETURN_EXPRESSION",
        "R_NEXT_EXPRESSION",
        "R_BREAK_EXPRESSION",
        "R_TRUE_EXPRESSION",
        "R_FALSE_EXPRESSION",
        "R_NULL_EXPRESSION",
        "R_INF_EXPRESSION",
        "R_NAN_EXPRESSION",
        "R_NA_EXPRESSION",
        // Bogus nodes
        "R_BOGUS",
        "R_BOGUS_VALUE",
        "R_BOGUS_EXPRESSION",
        "R_BOGUS_PARAMETER",
        "R_BOGUS_ARGUMENT",
    ],
};
