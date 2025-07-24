import socket
import os

class Piece:
    # piece_type: M pour pion, K pour dame
    # piece_color: B pour noir, W pour blanc
    def __init__(self, piece_type, piece_color):
        self.piece_type = piece_type
        self.piece_color = piece_color

class Position:
    # row: ligne de la cellule
    # col: colonne de la cellule
    def __init__(self, row, col):
        self.row = row
        self.col = col

class Move:
    # start: Position de départ du coup
    # end: Position d'arrivée du coup
    def __init__(self, start, end):
        self.start = start
        self.end = end

    def __str__(self):
        return f"{self.start.row}{self.start.col},{self.end.row}{self.end.col};"

    def __repr__(self):
        return self.__str__()


def find_move(board, player_color):

    # TODO: Implémentez ici la logique pour trouver les coups à jouer et les retourner
    # Les coups doivent être retournés sous forme d'une liste d'objets Move,
    # Chaque objet Move représente un coup, avec une cellule de départ et une cellule d'arrivée
    # Les classes Position(row, column) et Move(start, end) sont fournies pour vous
    
    moves = [Move(Position(6, 1), Position(5, 0))]

    return moves


def main():
    board = [
        [None for _ in range(10)] for _ in range(10)
    ]

    # Lecture de la couleur du joueur depuis la console
#    player_color = input().strip()[0]
#
#    # Parsage du plateau de jeu depuis la console
#    for r in range(10):
#        line = input().strip()
#        row = line.split(",")
#        for c, piece_code in enumerate(row):
#            if piece_code:
#                board[r][c] = Piece(piece_code[0], piece_code[1])
#            else:
#                board[r][c] = None 

    board = ""
    player_color = ""
    # Appel de la fonction findMove pour trouver les coups à jouer
    moves = find_move(board, player_color)

    if not moves:
        raise Exception("No moves were returned.")
        return

    # Envoi des coups trouvés à la console
    move_out = ""
    for move in moves:
        #print(move, end="")
        move_out += move.__str__()
#    move_out += '\n'
    print(move_out)
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as client:
        client.connect(os.environ['SOCK'])
        client.send(move_out.encode("ascii"))
        client.close()
    print("done")


if __name__ == "__main__":
    main()
