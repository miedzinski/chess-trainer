import React from "react";
import Puzzle from "./Puzzle";

export default class App extends React.Component {
    constructor(props) {
        super(props);
        this.puzzle = {
            fen: "3R4/8/K7/pB2b3/1p6/1P2k3/3p4/8 w - - 4 58",
            moves: [
                { from: "a6", to: "a5" },
                { from: "e5", to: "c7" },
                { from: "a5", to: "b4" },
                { from: "c7", to: "d8" },
            ],
        };
    }

    render() {
        return (
            <div className="App">
                <Puzzle puzzle={this.puzzle} />
            </div>
        );
    }
}
