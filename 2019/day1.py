import math

from typing import Callable


def fuel_required(mass: int):
    return math.floor(mass / 3) - 2


def fuel_required_until_empty(fn: Callable[[int], int], mass: int):
    new_mass = mass
    fuel_needed = 0
    while True:
        new_mass = fn(new_mass)
        if new_mass < 0:
            break
        fuel_needed += new_mass

    return fuel_needed


def fuel_required_nofloor(mass: int):
    return (mass // 3) - 2


def main():
    with open("2019/day1_input.txt", "r") as f:
        lines = f.readlines()

    fuel_requirements = [
        fuel_required_until_empty(fuel_required_nofloor, int(x)) for x in lines
    ]

    total = 0
    for r in fuel_requirements:
        total += r

    print(total)


if __name__ == "__main__":
    main()
