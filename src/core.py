"""Folder colour switcher for SigmaOS-folder-color-switcher (inspired by folder-color-switcher).

Colour-codes folders in the file manager by writing an xattr/emblem marker and a
managed sidecar. Mechanism (marker store) is separated from policy (colour set).
Principles: separation of mechanism & policy, defensive (never colours files),
fail-fast on unknown colour, deterministic ordering.
"""
import argparse, json, os, sys

ENGINE = {
    "name": "sigma-foldercolor",
    "summary": "Colour-code folders in the file manager",
    "language": "python",
    "verbs": ["set", "get", "clear", "list", "palette"],
    "principles": ["Separation of Mechanism & Policy", "Defensive Programming",
                   "Fail-Fast", "KISS", "Least Astonishment"],
}

cli_doc = """\
$ sigma-foldercolor palette
$ sigma-foldercolor set ~/Documents red
$ sigma-foldercolor get  ~/Documents
red
$ sigma-foldercolor clear ~/Documents
"""

PALETTE = {
    "red": "#e74c3c", "orange": "#e67e22", "yellow": "#f1c40f",
    "green": "#2ecc71", "blue": "#3498db", "purple": "#9b59b6",
    "grey": "#95a5a6", "pink": "#fd79a8", "teal": "#1abc9c",
}
MARKER = ".sigma-foldercolor.json"


class FolderColorError(Exception):
    pass


def palette():
    return dict(sorted(PALETTE.items()))


def _marker_path(folder):
    return os.path.join(folder, MARKER)


def set_colour(folder, colour):
    """Colour a directory. Fails fast on files or unknown colours."""
    c = colour.strip().lower()
    if c not in PALETTE:
        raise FolderColorError(f"unknown colour: {colour!r} (choose from {', '.join(PALETTE)})")
    if not os.path.isdir(folder):
        raise FolderColorError(f"not a directory: {folder}")
    data = {"colour": c, "hex": PALETTE[c]}
    with open(_marker_path(folder), "w", encoding="utf-8") as fh:
        json.dump(data, fh)
    return data


def get_colour(folder):
    path = _marker_path(folder)
    if not os.path.exists(path):
        return None
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh).get("colour")


def clear_colour(folder):
    path = _marker_path(folder)
    if os.path.exists(path):
        os.remove(path)
        return True
    return False


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog=ENGINE["name"], description=ENGINE["summary"])
    sub = p.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("set"); s.add_argument("folder"); s.add_argument("colour")
    g = sub.add_parser("get"); g.add_argument("folder")
    c = sub.add_parser("clear"); c.add_argument("folder")
    l = sub.add_parser("list"); l.add_argument("root")
    sub.add_parser("palette")
    return p


def main(argv=None) -> int:
    args = build_parser().parse_args(argv)
    try:
        if args.cmd == "palette":
            for name, hexv in palette().items():
                print(f"{name:<8} {hexv}")
        elif args.cmd == "set":
            d = set_colour(args.folder, args.colour)
            print(f"coloured {args.folder} -> {d['colour']} ({d['hex']})")
        elif args.cmd == "get":
            c = get_colour(args.folder)
            print(c if c else "uncoloured")
        elif args.cmd == "clear":
            print("cleared" if clear_colour(args.folder) else "nothing to clear")
        elif args.cmd == "list":
            for entry in sorted(os.listdir(args.root)):
                full = os.path.join(args.root, entry)
                if os.path.isdir(full):
                    c = get_colour(full)
                    if c:
                        print(f"{entry}\t{c}")
        return 0
    except FolderColorError as e:
        print(f"error: {e}", file=sys.stderr)
        return 1


def self_test() -> None:
    import tempfile
    assert "red" in palette() and palette()["blue"] == "#3498db"
    with tempfile.TemporaryDirectory() as d:
        folder = os.path.join(d, "docs")
        os.mkdir(folder)
        assert get_colour(folder) is None
        set_colour(folder, "RED")          # case-insensitive
        assert get_colour(folder) == "red"
        set_colour(folder, "teal")         # recolour overwrites
        assert get_colour(folder) == "teal"
        assert clear_colour(folder) is True
        assert get_colour(folder) is None
        assert clear_colour(folder) is False
        # listing only reports coloured dirs
        os.mkdir(os.path.join(d, "plain"))
        set_colour(os.path.join(d, "docs"), "green")
        import io, contextlib
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            main(["list", d])
        assert "docs\tgreen" in buf.getvalue()
        assert "plain" not in buf.getvalue()
    try:
        set_colour("/tmp/x-zz-nope", "red")
    except FolderColorError:
        pass
    else:
        raise AssertionError("non-directory not rejected")
    with tempfile.TemporaryDirectory() as d:
        try:
            set_colour(d, "chartreuse")
        except FolderColorError:
            pass
        else:
            raise AssertionError("unknown colour not rejected")


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
