from .base import BaseParser, MethodInfo
from .java import JavaParser
from .python import PythonParser
from .javascript import JavaScriptParser, TypeScriptParser
from .go import GoParser
from .rust import RustParser
from .c import CParser, CppParser
from .ruby import RubyParser
from .kotlin import KotlinParser
from .swift import SwiftParser
from .csharp import CSharpParser
from .php import PhpParser

EXT_TO_PARSER: dict[str, type[BaseParser]] = {
    ".java": JavaParser,
    ".py":   PythonParser,
    ".js":   JavaScriptParser,
    ".mjs":  JavaScriptParser,
    ".cjs":  JavaScriptParser,
    ".ts":   TypeScriptParser,
    ".tsx":  TypeScriptParser,
    ".go":   GoParser,
    ".rs":   RustParser,
    ".c":    CParser,
    ".h":    CParser,
    ".cpp":  CppParser,
    ".cc":   CppParser,
    ".cxx":  CppParser,
    ".hpp":  CppParser,
    ".rb":   RubyParser,
    ".kt":   KotlinParser,
    ".kts":  KotlinParser,
    ".swift": SwiftParser,
    ".cs":   CSharpParser,
    ".php":  PhpParser,
}
