def part1(nums):
    return sum(1 for (x, y) in zip(nums, nums[1:]) if y > x)

def part2(nums):
    windowed = [(x+y+z) for (x, y, z) in zip(nums, nums[1:], nums[2:])]
    return part1(windowed)

if __name__ == "__main__":
    with open("1.txt") as f:
        nums = [int(i) for i in f.readlines()]
    
    print(f"Part 1: {part1(nums)}")
    print(f"Part 2: {part2(nums)}")