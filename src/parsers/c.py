from .base import BaseParser


class CParser(BaseParser):
    LANG = "c"
    QUERY = """
        (function_definition
          declarator: (function_declarator
            declarator: (identifier) @name)) @method
    """

    def _build_signature(self, node) -> str:
        # C signature: return_type function_declarator(params)
        # declarator field contains function_declarator -> identifier + parameter_list
        ret_type = node.child_by_field_name("type")
        declarator = node.child_by_field_name("declarator")  # function_declarator
        if declarator:
            name = declarator.child_by_field_name("declarator")
            params = declarator.child_by_field_name("parameters")
            name_text = self._text(name) if name else ""
            sig = name_text + (self._text(params) if params else "()")
        else:
            sig = ""
        type_text = self._text(ret_type) if ret_type else ""
        return f"{type_text} {sig}".strip()


class CppParser(BaseParser):
    LANG = "cpp"
    QUERY = """
        (function_definition
          declarator: [
            (function_declarator declarator: (identifier) @name)
            (function_declarator declarator: (qualified_identifier name: (identifier) @name))
          ]) @method
    """

    def _build_signature(self, node) -> str:
        ret_type = node.child_by_field_name("type")
        declarator = node.child_by_field_name("declarator")
        if declarator:
            name = declarator.child_by_field_name("declarator")
            params = declarator.child_by_field_name("parameters")
            name_text = self._text(name) if name else ""
            sig = name_text + (self._text(params) if params else "()")
        else:
            sig = ""
        type_text = self._text(ret_type) if ret_type else ""
        return f"{type_text} {sig}".strip()
