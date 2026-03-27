from tree_sitter import Query, QueryCursor
from .base import BaseParser, ClassInfo


class JavaParser(BaseParser):
    LANG = "java"

    QUERY = """
        (method_declaration name: (identifier) @name) @method
    """

    CLASSES_QUERY = """
        [
          (class_declaration name: (identifier) @name)
          (interface_declaration name: (identifier) @name)
          (enum_declaration name: (identifier) @name)
        ] @class
    """

    CALLS_QUERY = """
        (method_invocation name: (identifier) @call)
    """

    CLASS_NODE_TYPES = ["class_declaration", "interface_declaration", "enum_declaration"]

    # Java annotations are part of the modifiers node, so _get_annotations does not
    # need to be overridden — annotations are included via _build_signature below.

    def _build_signature(self, node) -> str:
        parts = []
        for child in node.children:
            if child.type == "modifiers":
                parts.append(self._text(child))  # includes @Annotation, public, static, etc.
                break
        ret_type = node.child_by_field_name("type")
        name = node.child_by_field_name("name")
        params = node.child_by_field_name("parameters")
        if ret_type:
            parts.append(self._text(ret_type))
        if name:
            parts.append(self._text(name) + (self._text(params) if params else "()"))
        return " ".join(parts)

    def _build_field_signature(self, field_node) -> str:
        parts = []
        for child in field_node.children:
            if child.type == "modifiers":
                parts.append(self._text(child))  # includes field-level annotations like @Autowired
                break
        type_node = field_node.child_by_field_name("type")
        if type_node:
            parts.append(self._text(type_node))
        for child in field_node.children:
            if child.type == "variable_declarator":
                name = child.child_by_field_name("name")
                if name:
                    parts.append(self._text(name))
                break
        return " ".join(parts)

    def _build_constructor_signature(self, node) -> str:
        parts = []
        for child in node.children:
            if child.type == "modifiers":
                parts.append(self._text(child))
                break
        name = node.child_by_field_name("name")
        params = node.child_by_field_name("parameters")
        if name:
            parts.append(self._text(name) + (self._text(params) if params else "()"))
        return " ".join(parts)

    def get_class_skeleton(self, name: str) -> ClassInfo | None:
        query = Query(self._language, self.CLASSES_QUERY)
        captures = QueryCursor(query).captures(self._tree.root_node)

        if not all(k in captures for k in ("class", "name")):
            return None

        for cls_node, name_node in zip(captures["class"], captures["name"]):
            if self._text(name_node) != name:
                continue

            kind = {
                "class_declaration": "class",
                "interface_declaration": "interface",
                "enum_declaration": "enum",
            }.get(cls_node.type, cls_node.type)

            # extract class-level annotations from modifiers
            annotations = []
            for child in cls_node.children:
                if child.type == "modifiers":
                    for mod in child.children:
                        if mod.type in ("annotation", "marker_annotation"):
                            annotations.append(self._text(mod))
                    break

            # superclass
            superclass = None
            sc_node = cls_node.child_by_field_name("superclass")
            if sc_node:
                for child in sc_node.children:
                    if child.is_named:
                        superclass = self._text(child)
                        break

            # implemented interfaces
            interfaces = []
            ifaces_node = cls_node.child_by_field_name("interfaces")
            if ifaces_node:
                for child in ifaces_node.children:
                    if child.is_named and child.type != ",":
                        for t in child.children:
                            if t.is_named:
                                interfaces.append(self._text(t))

            # fields and methods from class body
            fields = []
            methods = []
            body = cls_node.child_by_field_name("body")
            if body:
                for child in body.children:
                    if child.type == "field_declaration":
                        fields.append(self._build_field_signature(child))
                    elif child.type == "method_declaration":
                        methods.append(self._build_signature(child))
                    elif child.type == "constructor_declaration":
                        methods.append(self._build_constructor_signature(child))

            return ClassInfo(
                name=name,
                kind=kind,
                annotations=annotations,
                superclass=superclass,
                interfaces=interfaces,
                fields=fields,
                methods=methods,
            )

        return None
