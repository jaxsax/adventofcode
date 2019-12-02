import os
import unittest


OPCODE_1 = 1
OPCODE_2 = 2
OPCODE_99 = 99


def calculate_once(contents: [int], next_index=0) -> (int, [int]):
    opcode = contents[next_index]

    if opcode == OPCODE_1:
        a = contents[contents[next_index + 1]]
        b = contents[contents[next_index + 2]]
        pos = contents[next_index + 3]
        contents[pos] = a + b
        next_index += 4
    elif opcode == OPCODE_2:
        a = contents[contents[next_index + 1]]
        b = contents[contents[next_index + 2]]
        pos = contents[next_index + 3]
        contents[pos] = a * b
        next_index += 4
    elif opcode == OPCODE_99:
        next_index = -1

    return (
        next_index,
        contents,
    )


def calculate_until_end(contents: [int]) -> [int]:
    next_index = 0
    while True:
        next_index, contents = calculate_once(contents, next_index)
        if next_index == -1:
            return contents


def to_all_ints(a: str) -> [int]:
    return [int(x) for x in a.split(",")]


def main():
    with open("2019/day2_input.txt") as f:
        contents = f.read()

    contents = to_all_ints(contents)

    target = 19690720
    for i in range(99):
        for j in range(99):
            contents_copy = contents.copy()
            contents_copy[1] = i
            contents_copy[2] = j

            answer = calculate_until_end(contents_copy)
            if answer[0] == target:
                print(f"i={i}, j={j}, contents={answer}")
                print(f"100*{i}+{j}={100 * i + j}")
                break

            diff = answer[0] - target
            within = diff >= -1000 and diff <= 1000
            if within:
                print(i, j, answer[0])


class Day2TestCase(unittest.TestCase):
    def test_demo_case(self):
        contents = to_all_ints("1,9,10,3,2,3,11,0,99,30,40,50")
        g_next_index, got = calculate_once(contents)
        w_next_index, want = 4, to_all_ints("1,9,10,70,2,3,11,0,99,30,40,50")

        self.assertEqual(w_next_index, g_next_index, "expected same index")
        self.assertEqual(want, got, "expected same contents")

        g1_next_index, got1 = calculate_once(got, g_next_index)
        w1_next_index, want1 = 8, to_all_ints("3500,9,10,70,2,3,11,0,99,30,40,50")

        self.assertEqual(w1_next_index, g1_next_index)
        self.assertEqual(want1, got1)

        g2_next_index, got2 = calculate_once(got1, g1_next_index)
        w2_next_index, want2 = -1, to_all_ints("3500,9,10,70,2,3,11,0,99,30,40,50")

        self.assertEqual(w2_next_index, g2_next_index)
        self.assertEqual(want2, got2)

    def test_more_demos(self):
        contents = to_all_ints("1,1,1,4,99,5,6,0,99")
        got = calculate_until_end(contents)

        self.assertEqual(to_all_ints("30,1,1,4,2,5,6,0,99"), got)


if __name__ == "__main__":
    if os.environ.get("TEST", 0):
        unittest.main()
    else:
        main()
