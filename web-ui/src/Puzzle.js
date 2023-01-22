import { Chess, SQUARES } from "chess.js"
import { Chessground } from "chessground";
import React from "react";
import "chessground/assets/chessground.base.css";
import "chessground/assets/chessground.brown.css";
import "chessground/assets/chessground.cburnett.css";
import "./Puzzle.css";

const Color = {
    White: "white",
    Black: "black",
};

export default class Puzzle extends React.Component {
    constructor(props) {
        super(props);
        this.boardRef = React.createRef();
    }

    updateBoard() {
        this.board.set({
            check: this.game.inCheck() ? this.currentColor() : false,
            turnColor: this.currentColor(),
            movable: {
                color: this.currentColor(),
                dests: this.legalMoves(),
            },
        });
    }

    currentColor() {
        return this.game.turn() === "w" ? Color.White : Color.Black;
    }

    legalMoves() {
        const allMoves = new Map();
        SQUARES.forEach((square) => {
            const moves = this.game.moves({ square, verbose: true });
            if (moves.length) {
                allMoves.set(
                    square,
                    moves.map((move) => move.to),
                );
            }
        });
        return allMoves;
    }

    handleMove(from, to) {
        this.game.move({ from, to });

        const expected = this.props.puzzle.moves[this.movesPlayed];
        const reply = this.props.puzzle.moves[this.movesPlayed + 1];

        if (from !== expected.from || to !== expected.to) {
            this.undo();
            return
        }

        this.movesPlayed += 2

        if (reply) {
            this.playReply(reply)
        }
    }

    undo() {
        this.game.undo();
        const lastMove = this.props.puzzle.moves[this.movesPlayed - 1]
        this.board.set({
            fen: this.game.fen(),
            lastMove: [lastMove.from, lastMove.to],
        });
        this.updateBoard()
    }

    playReply(move) {
        this.game.move(move)
        this.board.move(move.from, move.to)
        this.updateBoard();
    }

    componentDidMount() {
        this.game = new Chess(this.props.puzzle.fen);
        const initialMove = this.props.puzzle.moves[0]
        this.game.move(Object.assign(initialMove, { promotion: 'q' }))
        this.board = Chessground(this.boardRef.current, {
            fen: this.props.puzzle.fen,
            turnColor: this.currentColor(),
            orientation: this.currentColor(),
            movable: {
                free: false,
                events: {
                    after: this.handleMove.bind(this),
                },
            },
            premovable: {
                enabled: false,
            },
        });
        this.board.move(initialMove.from, initialMove.to);
        this.movesPlayed = 1
        this.updateBoard();
    }

    render() {
        return (
            <div className="puzzle">
                <div className="board" ref={this.boardRef}></div>
            </div>
        );
    }
}
