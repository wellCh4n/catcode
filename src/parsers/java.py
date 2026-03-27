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

    # ── 注解：Java 注解在 modifiers 里，_build_signature 直接 capture modifiers 文本即可
    # _get_annotations 不需要覆写，注解已包含在下面的签名构建中

    # ── 方法签名 ──────────────────────────────────────────────────────────────

    def _build_signature(self, node) -> str:
        parts = []
        for child in node.children:
            if child.type == "modifiers":
                parts.append(self._text(child))  # 包含 @Annotation 和 public/static 等
                break
        ret_type = node.child_by_field_name("type")
        name = node.child_by_field_name("name")
        params = node.child_by_field_name("parameters")
        if ret_type:
            parts.append(self._text(ret_type))
        if name:
            parts.append(self._text(name) + (self._text(params) if params else "()"))
        return " ".join(parts)

    # ── 字段签名 ──────────────────────────────────────────────────────────────

    def _build_field_signature(self, field_node) -> str:
        parts = []
        for child in field_node.children:
            if child.type == "modifiers":
                parts.append(self._text(child))  # 含 @Autowired 等字段注解
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

    # ── 构造器签名 ────────────────────────────────────────────────────────────

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

    # ── 类骨架 ────────────────────────────────────────────────────────────────

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

            # 类级别注解（从 modifiers 中提取 annotation 子节点）
            annotations = []
            for child in cls_node.children:
                if child.type == "modifiers":
                    for mod in child.children:
                        if mod.type in ("annotation", "marker_annotation"):
                            annotations.append(self._text(mod))
                    break

            # 父类
            superclass = None
            sc_node = cls_node.child_by_field_name("superclass")
            if sc_node:
                for child in sc_node.children:
                    if child.is_named:
                        superclass = self._text(child)
                        break

            # 接口
            interfaces = []
            ifaces_node = cls_node.child_by_field_name("interfaces")
            if ifaces_node:
                for child in ifaces_node.children:
                    if child.is_named and child.type != ",":
                        for t in child.children:
                            if t.is_named:
                                interfaces.append(self._text(t))

            # 字段 & 方法
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
