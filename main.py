from dataclasses import dataclass
from tree_sitter import Query, QueryCursor
from tree_sitter_language_pack import get_language, get_parser


@dataclass
class MethodInfo:
    body: str
    lines: str 


class JavaParser:
    _METHODS_QUERY = """
    (method_declaration
      name: (identifier) @name)
    """

    _METHOD_DETAIL_QUERY = """
    (method_declaration
      name: (identifier) @name) @method
    """

    def __init__(self, file_path: str):
        language = get_language("java")
        parser = get_parser("java")
        with open(file_path, "rb") as f:
            self._source = f.read()
        self._tree = parser.parse(self._source)
        self._language = language

    def _text(self, node) -> str:
        return node.text.decode("utf-8")

    def list_methods(self) -> list[dict]:
        """返回所有方法的名称和行号范围"""
        query = Query(self._language, self._METHODS_QUERY)
        captures = QueryCursor(query).captures(self._tree.root_node)
        results = []
        if "name" in captures:
            for n_node in captures["name"]:
                results.append(self._text(n_node))
        return results

    def get_method(self, name: str) -> MethodInfo | None:
        """返回指定方法的方法体、起始行、行数"""
        query = Query(self._language, self._METHOD_DETAIL_QUERY)
        captures = QueryCursor(query).captures(self._tree.root_node)

        if not all(k in captures for k in ("method", "name")):
            return None

        for m, nm in zip(captures["method"], captures["name"]):
            if self._text(nm) != name:
                continue
            return MethodInfo(
                body=self._text(m),
                lines=f"L{m.start_point[0] + 1}-{m.end_point[0] + 1}",
            )
        return None


# ── 示例用法 ──────────────────────────────────────────────────────────────────
if __name__ == "__main__":
    FILE = "/Users/chenweihao/Codes/oops/src/main/java/com/github/wellch4n/oops/service/ApplicationService.java"
    jp = JavaParser(FILE)

    print("# Methods")
    for name in jp.list_methods():
        print(f"  - `{name}`")

    info = jp.get_method("getApplication")
    if info:
        print(f"# Method: getApplication")
        print(f"// {info.lines}\n{info.body} ")
    else:
        print("  (not found)")
