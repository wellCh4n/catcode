from .base import BaseParser


class PhpParser(BaseParser):
    LANG = "php"
    QUERY = """
        (method_declaration name: (name) @name) @method
    """
