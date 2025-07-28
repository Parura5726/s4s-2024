#include <vector>
#include <optional>

struct Piece {
    char pieceType; // pieceType: 'M' pour pion, 'K' pour dame
    char pieceColor; // pieceColor: 'W' pour blanc, 'B' pour noir
    Piece(char type, char color) : pieceType(type), pieceColor(color) {}
};

struct Position {
    int row; // row: ligne de la cellule
    int column; // column: colonne de la cellule
};

struct Move {
    Position from; // from: cellule de départ
    Position to; // to: cellule d'arrivée
};

// Fonction pour trouver les coups à jouer
std::vector<Move> findMove(const std::vector<std::vector<std::optional<Piece>>>& board, char playerColor) {

    // TODO: Implémentez ici la logique pour trouver les coups à jouer et les retourner
    // Les coups doivent être retournés sous forme d'une liste d'objets Move,
    // Chaque objet Move représente un coup, avec une cellule de départ et une cellule d'arrivée
    // Les classes Position(row, column) et Move(from, to) sont fournies pour vous

    Position start = {6, 1};
    Position end = {5, 0};
    Move move = {start, end};

    std::vector<Move> moves;
    moves.push_back(move);

    return moves;
}

