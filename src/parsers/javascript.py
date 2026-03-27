from .base import BaseParser


def _preceding_decorators(node, text_fn) -> list[str]:
    """收集紧邻该节点之前的 decorator 兄弟节点"""
    if not node.parent:
        return []
    decorators = []
    for child in node.parent.children:
        if child == node:
            break
        if child.type == "decorator":
            decorators.append(text_fn(child))
        else:
            decorators.clear()  # 只取紧邻的，中间有其他节点就重置
    return decorators


class JavaScriptParser(BaseParser):
    LANG = "javascript"

    QUERY = """
        [
          (function_declaration name: (identifier) @name)
          (method_definition name: (property_identifier) @name)
        ] @method
    """

    CLASSES_QUERY = """
        (class_declaration name: (identifier) @name) @class
    """

    CALLS_QUERY = """
        (call_expression function: [
          (identifier) @call
          (member_expression property: (property_identifier) @call)
        ])
    """

    CLASS_NODE_TYPES = ["class_declaration"]

    def _build_signature(self, node) -> str:
        name = node.child_by_field_name("name")
        params = node.child_by_field_name("parameters")
        sig = (self._text(name) if name else "") + (self._text(params) if params else "()")
        return sig


class TypeScriptParser(BaseParser):
    LANG = "typescript"

    QUERY = """
        [
          (function_declaration name: (identifier) @name)
          (method_definition name: (property_identifier) @name)
        ] @method
    """

    CLASSES_QUERY = """
        (class_declaration name: (type_identifier) @name) @class
    """

    CALLS_QUERY = """
        (call_expression function: [
          (identifier) @call
          (member_expression property: (property_identifier) @call)
        ])
    """

    CLASS_NODE_TYPES = ["class_declaration"]

    def _get_annotations(self, node) -> list[str]:
        return _preceding_decorators(node, self._text)

    def _build_signature(self, node) -> str:
        name = node.child_by_field_name("name")
        params = node.child_by_field_name("parameters")
        return_type = node.child_by_field_name("return_type")

        sig = (self._text(name) if name else "") + (self._text(params) if params else "()")
        if return_type:
            sig += f": {self._text(return_type)}"

        decorators = self._get_annotations(node)
        if decorators:
            return " ".join(decorators) + " " + sig
        return sig
