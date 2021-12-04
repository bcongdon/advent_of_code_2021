from itertools import chain


def board_wins(board, called):
    for i in range(len(board)):
        has_scored = True
        for j in range(len(board[i])):
            if board[i][j] not in called:
                has_scored = False
                break
        if has_scored:
            return True
    for j in range(len(board[0])):
        has_scored = True
        for i in range(len(board)):
            if board[i][j] not in called:
                has_scored = False
                break
        if has_scored:
            return True
    return False


def board_score(board, called):
    not_called = set(chain(*board)) - set(called)
    return sum(not_called) * called[-1]


if __name__ == "__main__":
    with open("4.txt") as f:
        lines = f.readlines()

    called_numbers = [int(i) for i in lines[0].split(",")]

    boards = []
    lines = lines[2:]
    for s_idx in range(0, len(lines), 6):
        board_lines = lines[s_idx : s_idx + 6][:-1]
        board = [[int(n) for n in line.strip().split()] for line in board_lines]
        assert len(board) == 5
        boards.append(board)

    scores = []
    for i in range(0, len(called_numbers)):
        non_winning = []
        for b in boards:
            if board_wins(b, called_numbers[:i]):
                scores.append(board_score(b, called_numbers[:i]))
            else:
                non_winning.append(b)
        boards = non_winning
    print("Part 1: {}".format(scores[0]))
    print("Part 2: {}".format(scores[-1]))
