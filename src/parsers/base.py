from dataclasses import dataclass, field
from tree_sitter import Query, QueryCursor
from tree_sitter_language_pack import get_language, get_parser


@dataclass
class MethodInfo:
    body: str
    lines: str
    class_name: str | None
    calls: list[str]


@dataclass
class ClassInfo:
    name: str
    kind: str                    # class / interface / struct / ...
    annotations: list[str]       # 类级别的注解/装饰器
    superclass: str | None
    interfaces: list[str]
    fields: list[str]            # 已格式化的签名字符串（含注解）
    methods: list[str]           # 已格式化的签名字符串（含注解）


class BaseParser:
    LANG: str = ""
    QUERY: str = ""              # 捕获 @method / @name
    CLASSES_QUERY: str = ""      # 捕获 @class / @name
    CALLS_QUERY: str = ""        # 在方法节点上运行，捕获 @call
    CLASS_NODE_TYPES: list[str] = []

    def __init__(self, file_path: str):
        self._language = get_language(self.LANG)
        parser = get_parser(self.LANG)
        with open(file_path, "rb") as f:
            self._source = f.read()
        self._tree = parser.parse(self._source)
        self._query = Query(self._language, self.QUERY)

    def _text(self, node) -> str:
        return node.text.decode("utf-8")

    def _captures(self):
        return QueryCursor(self._query).captures(self._tree.root_node)

    # ── 注解/装饰器（子类覆写） ───────────────────────────────────────────────

    def _get_annotations(self, node) -> list[str]:
        """返回该节点的注解/装饰器字符串列表，子类按语言覆写"""
        return []

    # ── 找到节点所在的类 ──────────────────────────────────────────────────────

    def _enclosing_class(self, node) -> str | None:
        parent = node.parent
        while parent:
            if parent.type in self.CLASS_NODE_TYPES:
                name_node = parent.child_by_field_name("name")
                if name_node:
                    return self._text(name_node)
            parent = parent.parent
        return None

    # ── 方法签名（子类覆写） ──────────────────────────────────────────────────

    def _build_signature(self, node) -> str:
        annotations = self._get_annotations(node)
        name = node.child_by_field_name("name")
        sig = self._text(name) if name else self._text(node)
        if annotations:
            return " ".join(annotations) + " " + sig
        return sig

    # ── 方法列表 ─────────────────────────────────────────────────────────────

    def list_methods(self) -> list[tuple[str | None, str]]:
        captures = self._captures()
        if "method" not in captures:
            return []
        return [
            (self._enclosing_class(m), self._build_signature(m))
            for m in captures["method"]
        ]

    # ── 方法详情 ─────────────────────────────────────────────────────────────

    def _get_calls(self, method_node) -> list[str]:
        if not self.CALLS_QUERY:
            return []
        query = Query(self._language, self.CALLS_QUERY)
        captures = QueryCursor(query).captures(method_node)
        if "call" not in captures:
            return []
        seen: set[str] = set()
        result = []
        for n in captures["call"]:
            t = self._text(n)
            if t not in seen:
                seen.add(t)
                result.append(t)
        return result

    def _method_name(self, node) -> str | None:
        """从方法节点取名字，子类可覆写处理特殊节点类型"""
        name_node = node.child_by_field_name("name")
        return self._text(name_node) if name_node else None

    def get_method(self, name: str) -> MethodInfo | None:
        captures = self._captures()
        if "method" not in captures:
            return None
        for m in captures["method"]:
            if self._method_name(m) != name:
                continue
            return MethodInfo(
                body=self._text(m),
                lines=f"L{m.start_point[0] + 1}-{m.end_point[0] + 1}",
                class_name=self._enclosing_class(m),
                calls=self._get_calls(m),
            )
        return None

    # ── 类列表 ───────────────────────────────────────────────────────────────

    def list_classes(self) -> list[str]:
        if not self.CLASSES_QUERY:
            return []
        query = Query(self._language, self.CLASSES_QUERY)
        captures = QueryCursor(query).captures(self._tree.root_node)
        if "name" not in captures:
            return []
        return [self._text(n) for n in captures["name"]]

    # ── 类骨架（子类覆写） ────────────────────────────────────────────────────

    def get_class_skeleton(self, name: str) -> ClassInfo | None:
        return None
