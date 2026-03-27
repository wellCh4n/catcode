from tree_sitter import Query, QueryCursor
from .base import BaseParser, ClassInfo


class GoParser(BaseParser):
    LANG = "go"

    QUERY = """
        [
          (function_declaration name: (identifier) @name)
          (method_declaration name: (field_identifier) @name)
        ] @method
    """

    CLASSES_QUERY = """
        (type_declaration
          (type_spec name: (type_identifier) @name
            type: (struct_type)) @class)
    """

    CALLS_QUERY = """
        (call_expression function: [
          (identifier) @call
          (selector_expression field: (field_identifier) @call)
        ])
    """

    CLASS_NODE_TYPES = ["type_spec"]

    def _build_signature(self, node) -> str:
        parts = ["func"]
        receiver = node.child_by_field_name("receiver")
        if receiver:
            parts.append(self._text(receiver))
        name = node.child_by_field_name("name")
        params = node.child_by_field_name("parameters")
        result = node.child_by_field_name("result")
        name_text = self._text(name) if name else ""
        parts.append(name_text + (self._text(params) if params else "()"))
        if result:
            parts.append(self._text(result))
        return " ".join(parts)

    def _enclosing_class(self, node) -> str | None:
        # Go methods are associated with a type via the receiver, not lexical nesting
        receiver = node.child_by_field_name("receiver")
        if receiver:
            for child in receiver.children:
                if child.type == "parameter_declaration":
                    type_node = child.child_by_field_name("type")
                    if type_node:
                        if type_node.type == "pointer_type":
                            for t in type_node.children:
                                if t.is_named:
                                    return self._text(t)
                        return self._text(type_node)
        return None

    def get_class_skeleton(self, name: str) -> ClassInfo | None:
        # find the struct definition
        query = Query(self._language, self.CLASSES_QUERY)
        captures = QueryCursor(query).captures(self._tree.root_node)

        if not all(k in captures for k in ("class", "name")):
            return None

        type_spec_node = None
        for cls_node, name_node in zip(captures["class"], captures["name"]):
            if self._text(name_node) == name:
                type_spec_node = cls_node  # type_spec node
                break

        if type_spec_node is None:
            return None

        # extract struct fields
        fields = []
        struct_type = type_spec_node.child_by_field_name("type")
        if struct_type and struct_type.type == "struct_type":
            for child in struct_type.children:
                if child.type == "field_declaration_list":
                    for decl in child.children:
                        if decl.type == "field_declaration":
                            fields.append(self._text(decl))

        # collect methods whose receiver type matches this struct
        method_captures = QueryCursor(self._query).captures(self._tree.root_node)
        methods = []
        if "method" in method_captures:
            for m in method_captures["method"]:
                if self._enclosing_class(m) == name:
                    methods.append(self._build_signature(m))

        return ClassInfo(
            name=name,
            kind="struct",
            annotations=[],
            superclass=None,
            interfaces=[],
            fields=fields,
            methods=methods,
        )
