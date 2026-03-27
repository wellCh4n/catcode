from .base import BaseParser


class RubyParser(BaseParser):
    LANG = "ruby"
    QUERY = """
        (method name: (identifier) @name) @method
    """
