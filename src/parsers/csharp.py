from .base import BaseParser


class CSharpParser(BaseParser):
    LANG = "csharp"
    QUERY = """
        (method_declaration name: (identifier) @name) @method
    """
