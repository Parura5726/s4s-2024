import socket
import os

import script

def main():
    board = [
        [None for _ in range(10)] for _ in range(10)
    ]

    # Lecture de la couleur du joueur depuis la console
    player_color = input().strip()[0]

    # Parsage du plateau de jeu depuis la console
    for r in range(10):
        line = input().strip()
        row = line.split(",")
        for c, piece_code in enumerate(row):
            if piece_code:
                board[r][c] = script.Piece(piece_code[0], piece_code[1])
            else:
                board[r][c] = None

    board = ""
    player_color = ""
    # Appel de la fonction findMove pour trouver les coups à jouer
    moves = script.find_move(board, player_color)

    if not moves:
        raise Exception("No moves were returned.")
        return

    # Envoi des coups trouvés à la console
    move_out = ""
    for move in moves:
        move_out += move.__str__()
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as client:
        client.connect(os.environ['SOCK'])
        client.send(move_out.encode("ascii"))
        client.close()


if __name__ == "__main__":
    main()
