#include "base.hpp"

#include <iostream>
#include <string>
#include <sstream>
#include <cstring>

#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

int main() {
    std::vector<std::vector<std::optional<Piece>>> board(10, std::vector<std::optional<Piece>>(10));
    char playerColor;

    // Lecture de la couleur du joueur depuis la console
    std::cin >> playerColor;
    std::cin.ignore(); // Ignore the newline character after reading playerColor

    // Parsage du plateau de jeu depuis la console
    for (int r = 0; r < 10; r++) {
        std::string line;
        std::getline(std::cin, line);
        std::stringstream ss(line);
        std::string pieceCode;
        int c = 0;

        while (std::getline(ss, pieceCode, ',')) {
            if (!pieceCode.empty()) {
                board[r][c] = Piece(pieceCode[0], pieceCode[1]);
            } else {
                board[r][c].reset();
            }
            c++;
        }
    }

    // Appel de la fonction findMove pour trouver les coups à jouer
    auto moves = findMove(board, playerColor);

    if (moves.empty()) {
        std::cerr << "No moves were returned." << std::endl;
        return 0;
    }

    // Envoi des coups trouvés à la console
    std::stringstream moves_out_ss;
    for (const auto& pos : moves) {
        moves_out_ss
            << pos.from.row << pos.from.column << "," 
            << pos.to.row << pos.to.column << ";";
    }
    std::string moves_out = moves_out_ss.str();

    // Create socket
    int sock;
    struct sockaddr_un server_addr;

    // 1. Create a socket
    sock = socket(AF_UNIX, SOCK_STREAM, 0);
    if (sock == -1) {
        perror("socket creation failed");
        exit(EXIT_FAILURE);
    }

    // 2. Set up the server address structure
    memset(&server_addr, 0, sizeof(struct sockaddr_un));
    server_addr.sun_family = AF_UNIX;
    strncpy(server_addr.sun_path, std::getenv("SOCK"), sizeof(server_addr.sun_path) - 1);

    // 3. Connect to the server
    if (connect(sock, (struct sockaddr *)&server_addr, sizeof(struct sockaddr_un)) == -1) {
        perror("connection failed");
        close(sock);
        exit(EXIT_FAILURE);
    }

    // 4. Send data to the server
    if (send(sock, moves_out.c_str(), moves_out.size(), 0) == -1) {
        perror("sending data failed");
    }


    return 0;
}
