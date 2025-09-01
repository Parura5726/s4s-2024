package s4s;

import java.util.ArrayList;
import java.util.List;

public class Base {
    // pieceType: 'M' pour pion, 'K' pour dame
    // pieceColor: 'W' pour blanc, 'B' pour noir
    public record Piece(char pieceType, char pieceColor) {}

    // row: numéro de la ligne, column: numéro de la colonne
    public record Position(int row, int column) {}

    // from: position de départ, to: position d'arrivée
    public record Move(Position from, Position to) {}

    // Fonction pour trouver les coups à jouer
    public static List<Move> findMove(Piece[][] board, char playerColor, List<ArrayList<Move>> possibleMoves) {

        // TODO: Implémentez ici la logique pour trouver les coups à jouer et les retourner
        // Les coups doivent être retournés sous forme d'une liste d'objets Move,
        // Chaque objet Move représente un coup, avec une cellule de départ et une cellule d'arrivée
        // Les classes Position(row, column) et Move(from, to) sont fournies pour vous

        //System.out.println("You can print your debug values here!");

        return possibleMoves.get(0);
    }
}
