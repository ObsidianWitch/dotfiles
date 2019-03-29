#!/usr/bin/env python3

import sys
import os
import argparse

class Xattr:
    @classmethod
    def list(cls, path): return [
        attr[5:] for attr in os.listxattr(path)
    ]

    @classmethod
    def rm(cls, path, attr):
        try: os.removexattr(path, "user." + attr)
        except OSError: print(f"Xattr.rm: {path} does not contain {attr}")

    @classmethod
    def set(cls, path, attr, val = ""): os.setxattr(
        path, "user." + attr, val.encode()
    )

class Tag:
    def __init__(self):
        parser = argparse.ArgumentParser(
            description = "Tagging tool using extended attributes (xattr)."
        )
        parser.add_argument("-d", "--debug",
            action = "store_true",
            help   = "display traceback"
        )
        subparsers = parser.add_subparsers(
            required = True,
            dest     = "command",
            metavar  = "command",
            help     = "{ls, search, add, rm, clean}",
        )

        # ls
        parser_ls = subparsers.add_parser("ls",
            help = "List tags for the given paths.",
        )
        parser_ls.add_argument("paths", nargs = "*", default = ["."])

        # search
        parser_search = subparsers.add_parser("search",
            help     = "Search files by evaluating the given expression.",
            add_help = False,
        )
        parser_search.add_argument("expression")
        parser_search.add_argument("directory", nargs = "?", default = ".")

        # add
        parser_add = subparsers.add_parser("add",
            help = "Add tag to the given files.",
        )
        parser_add.add_argument("tag")
        parser_add.add_argument("paths", nargs = "+")

        # rm
        parser_rm = subparsers.add_parser("rm",
            help = "Remove tag from the given files.",
        )
        parser_rm.add_argument("tag")
        parser_rm.add_argument("paths", nargs = "+")

        # clean
        parser_clean = subparsers.add_parser("clean",
            help = "Remove all tags from the given files.",
        )
        parser_clean.add_argument("paths", nargs = "+")

        # parse
        args = parser.parse_args().__dict__
        if not args.pop("debug"): sys.tracebacklimit = 0
        getattr(self, args.pop("command"))(**args)

    # Generator function returning the content of `path` if it is a directory,
    # or itself if it is a file. It can also return the content of
    # subdirectories recursively if `recursive` is `True`.
    @classmethod
    def walk(cls, path, recursive):
        yield path
        for dirpath, dirnames, filenames in os.walk(path):
            if not recursive: yield from (
                os.path.join(dirpath, d) for d in sorted(dirnames)
            )
            yield from ( os.path.join(dirpath, f) for f in sorted(filenames) )
            if not recursive: break

    @classmethod
    def ls(cls, paths = ["."]):
        for path1 in paths:
            for path2 in cls.walk(path1, recursive = False):
                xattrs = Xattr.list(path2)
                if not xattrs: continue
                strxattrs = str(xattrs).replace("'", "")
                print(f"{path2}: {strxattrs}")

    @classmethod
    def search(cls, expression, directory = "."):
        def satisfy_one(attrs, criterion):
            include = (criterion in attrs)
            exclude = (criterion[0] == "-") and (criterion[1:] not in attrs)
            return include or exclude

        def satisfy_all(attrs, criteria): return all(
            satisfy_one(attrs, criterion)
            for criterion in criteria
        )

        print(*filter(
            lambda filepath: satisfy_all(
                attrs    = Xattr.list(filepath),
                criteria = expression.split(),
            ),
            cls.walk(directory, recursive = True)
        ))

    @classmethod
    def add(cls, tag, paths):
        for path in paths:
            Xattr.set(path, tag)

    @classmethod
    def rm(cls, tag, paths):
        for path in paths:
            Xattr.rm(path, tag)

    @classmethod
    def clean(cls, paths):
        for path in paths:
            for tag in Xattr.list(path):
                Xattr.rm(path, tag)

if __name__ == "__main__": Tag()
