from .base import BaseParser


class KotlinParser(BaseParser):
    LANG = "kotlin"
    QUERY = """
        (function_declaration (simple_identifier) @name) @method
    """
