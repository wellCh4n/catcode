import sys
from pathlib import Path
from src.parsers import EXT_TO_PARSER


def print_method_list(methods: list[tuple[str | None, str]]) -> None:
    if not methods:
        print("(no methods found)")
        return
    # 按 class 分组
    groups: dict[str, list[str]] = {}
    for class_name, sig in methods:
        key = class_name or "(top-level)"
        groups.setdefault(key, []).append(sig)

    for class_name, sigs in groups.items():
        print(f"**{class_name}**")
        for sig in sigs:
            print(f"- `{sig}`")
        print()


def print_method_detail(args_method: str, info, ext: str) -> None:
    print(f"> {info.lines}")
    if info.class_name:
        print(f"\n**class** `{info.class_name}`")
    if info.calls:
        calls_str = ", ".join(f"`{c}`" for c in info.calls)
        print(f"\n**calls** {calls_str}")
    print(f"\n```{ext}\n{info.body}\n```")


def print_class_list(classes: list[str]) -> None:
    if not classes:
        print("(no classes found)")
        return
    for name in classes:
        print(f"- `{name}`")


def print_class_skeleton(info) -> None:
    if info.annotations:
        print(" ".join(f"`{a}`" for a in info.annotations))
    header = f"## {info.kind} `{info.name}`"
    if info.superclass:
        header += f" extends `{info.superclass}`"
    if info.interfaces:
        ifaces = ", ".join(f"`{i}`" for i in info.interfaces)
        header += f" implements {ifaces}"
    print(header)

    if info.fields:
        print("\n**Fields**")
        for f in info.fields:
            print(f"- `{f}`")

    if info.methods:
        print("\n**Methods**")
        for m in info.methods:
            print(f"- `{m}`")


def main():
    import argparse

    parser = argparse.ArgumentParser(prog="catcode")
    parser.add_argument("-f", "--file", required=True, help="源文件路径")
    parser.add_argument(
        "-m", "--method", nargs="?", const="__list__", default=None,
        metavar="METHOD_NAME",
        help="不带参数时列出所有方法；带方法名时显示方法详情",
    )
    parser.add_argument(
        "-c", "--class", dest="cls", nargs="?", const="__list__", default=None,
        metavar="CLASS_NAME",
        help="不带参数时列出所有类；带类名时显示类骨架",
    )
    args = parser.parse_args()

    if args.method is None and args.cls is None:
        args.method = "__list__"

    ext = Path(args.file).suffix.lower()
    parser_cls = EXT_TO_PARSER.get(ext)
    if parser_cls is None:
        supported = ", ".join(sorted(EXT_TO_PARSER.keys()))
        print(f"unsupported file extension '{ext}'. supported: {supported}", file=sys.stderr)
        sys.exit(1)

    cp = parser_cls(args.file)
    file_ext = ext.lstrip(".")

    if args.method == "__list__":
        print_method_list(cp.list_methods())

    elif args.method is not None:
        info = cp.get_method(args.method)
        if info is None:
            print(f"method '{args.method}' not found", file=sys.stderr)
            sys.exit(1)
        print_method_detail(args.method, info, file_ext)

    elif args.cls == "__list__":
        print_class_list(cp.list_classes())

    elif args.cls is not None:
        info = cp.get_class_skeleton(args.cls)
        if info is None:
            print(f"class '{args.cls}' not found", file=sys.stderr)
            sys.exit(1)
        print_class_skeleton(info)
