from .base import BaseParser


class RustParser(BaseParser):
    LANG = "rust"

    QUERY = """
        (function_item name: (identifier) @name) @method
    """

    CLASSES_QUERY = """
        [
          (struct_item name: (type_identifier) @name)
          (impl_item type: (type_identifier) @name)
        ] @class
    """

    CALLS_QUERY = """
        (call_expression function: [
          (identifier) @call
          (field_expression field: (field_identifier) @call)
          (scoped_identifier name: (identifier) @call)
        ])
    """

    CLASS_NODE_TYPES = ["impl_item"]

    def _get_annotations(self, node) -> list[str]:
        """#[attr] 是函数节点的前置兄弟节点 attribute_item"""
        if not node.parent:
            return []
        attrs = []
        for child in node.parent.children:
            if child == node:
                break
            if child.type == "attribute_item":
                attrs.append(self._text(child))
            else:
                attrs.clear()  # 只取紧邻的
        return attrs

    def _build_signature(self, node) -> str:
        parts = []
        for child in node.children:
            if child.type == "visibility_modifier":
                parts.append(self._text(child))
                break
        parts.append("fn")
        name = node.child_by_field_name("name")
        params = node.child_by_field_name("parameters")
        return_type = node.child_by_field_name("return_type")
        name_text = self._text(name) if name else ""
        parts.append(name_text + (self._text(params) if params else "()"))
        if return_type:
            parts.append(f"-> {self._text(return_type)}")

        sig = " ".join(parts)
        attrs = self._get_annotations(node)
        if attrs:
            return " ".join(attrs) + " " + sig
        return sig

    def _enclosing_class(self, node) -> str | None:
        parent = node.parent
        while parent:
            if parent.type == "impl_item":
                type_node = parent.child_by_field_name("type")
                if type_node:
                    return self._text(type_node)
            parent = parent.parent
        return None
