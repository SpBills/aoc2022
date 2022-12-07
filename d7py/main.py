import math


TOTAL_SPACE = 70000000
NEEDED_SPACE = 30000000

def main():
    with open("inp.txt") as f:
        lines = iter(f.readlines())

        found = []
        all_found = []

        # current data
        dir_stack = []
        for line in lines:
            spl = iter(line.split(" "))
            # current sum
            match list(spl):
                case ["$", "cd", "..\n"]: 
                    (name, size) = dir_stack.pop()
                    (lname, lsize) = dir_stack[-1]
                    dir_stack[-1] = (lname, lsize + size)

                    if size <= 100_000:
                        found.append((name, size))

                    all_found.append((name, size))

                    print(f"left {name}")
                case ["$", "cd", x]:
                    dir_stack.append((x.strip(), 0))
                case ["$", "ls\n"]: pass
                case ["dir", x]: pass
                case [x, _]: 
                    x = int(x)
                    (lname, lsize) = dir_stack[-1]
                    dir_stack[-1] = (lname, lsize + x)
                case _: print("unreach")
        
        # The above algorithm doesn't catch anything that's left in the dir stack.

        s = 0
        for (name, sum) in found:
            s += sum

        print(f"part 1: {s}")
        # collapse the directory stack
        while dir_stack:
            (name, size) = dir_stack.pop()
            if dir_stack:
                (lname, lsize) = dir_stack[-1]
                final = (lname, lsize + size)
                dir_stack[-1] = final

        remaining = TOTAL_SPACE - final[1]
        print(f"Total space remaining: {remaining}")
        min = ("", 0)
        min_track = math.inf
        for (name, size) in all_found:
            if remaining + size >= NEEDED_SPACE and remaining + size < min_track:
                min_track = remaining + size
                min = size

        print(remaining + min)
        print(min)

if __name__ == "__main__":
    main()
