package s4s;

import java.io.FileWriter;
import java.io.IOException;
import java.util.List;
import java.util.Scanner;
import java.net.UnixDomainSocketAddress;
import java.nio.channels.SocketChannel;
import java.nio.ByteBuffer;
import java.nio.charset.Charset;
import s4s.Base;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        Base.Piece[][] board = new Base.Piece[10][10];

        // Lecture de la couleur du joueur depuis la console
        char playerColor = scanner.nextLine().charAt(0);

        // Parsage du plateau de jeu depuis la console
        for (int r = 0; r < 10; r++) {
            String line = scanner.nextLine();
            String[] row = line.split(",");
            Base.Piece[] pieceRow = new Base.Piece[row.length];
            for (int c = 0; c < row.length; c++) {
                String pieceCode = row[c];
                if (!pieceCode.isEmpty()) {
                    pieceRow[c] = new Base.Piece(pieceCode.charAt(0), pieceCode.charAt(1));
                } else {
                    pieceRow[c] = null;
                }
            }

            board[r] = pieceRow;
        }

        scanner.close();

        // Appel de la fonction findBase.Move pour trouver les coups à jouer
        List<Base.Move> moves = Base.findMove(board, playerColor);

        if (moves == null) {
            return;
        }

        // Format moves
        String moves_out = "";
        for (Base.Move move : moves) {
            moves_out +=
                move.from().row() + "" + move.from().column() + ","
                + move.to().row() + "" + move.to().column() + ";";
        }

        // Send moves
        SocketChannel socket;
        try {
            socket = SocketChannel.open(UnixDomainSocketAddress.of(System.getenv("SOCK")));
            socket.write(Charset.forName("UTF-8").encode(moves_out));
            socket.close();
        } catch (IOException e) {
            System.out.println(e);
        }
    }
}
