from .base import BaseParser


class SwiftParser(BaseParser):
    LANG = "swift"
    QUERY = """
        (function_declaration name: (simple_identifier) @name) @method
    """
