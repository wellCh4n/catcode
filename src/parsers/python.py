from .base import BaseParser


class PythonParser(BaseParser):
    LANG = "python"

    QUERY = """
        (function_definition name: (identifier) @name) @method
    """

    CLASSES_QUERY = """
        (class_definition name: (identifier) @name) @class
    """

    CALLS_QUERY = """
        (call function: [
          (identifier) @call
          (attribute attribute: (identifier) @call)
        ])
    """

    CLASS_NODE_TYPES = ["class_definition"]

    def _get_annotations(self, node) -> list[str]:
        """装饰器在 decorated_definition 父节点的 decorator 子节点中"""
        if node.parent and node.parent.type == "decorated_definition":
            return [
                self._text(child)
                for child in node.parent.children
                if child.type == "decorator"
            ]
        return []

    def _build_signature(self, node) -> str:
        name = node.child_by_field_name("name")
        params = node.child_by_field_name("parameters")
        return_type = node.child_by_field_name("return_type")

        sig = f"def {self._text(name)}{self._text(params) if params else '()'}"
        if return_type:
            sig += f" -> {self._text(return_type)}"

        decorators = self._get_annotations(node)
        if decorators:
            return " ".join(decorators) + " " + sig
        return sig
