def day(name):
    native.py_binary(
        name = name,
        srcs = [name + ".py"],
        data = [name + "_input.txt"],
    )

    native.py_test(
        name = name + "_test",
        srcs = [name + "_test.py"],
        deps = [
            ":" + name
        ],
        size = "small"
    )