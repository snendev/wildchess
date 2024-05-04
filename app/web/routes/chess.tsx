import { JSX } from "preact";

import Page from "../components/Page.tsx";

import Game from "../islands/Game.tsx";

interface ChessBoardProps {
    size?: [number, number];
    dimensions?: [number, number];
}

export default function ChessBoard({
    size = [1200, 1200],
    dimensions = [8, 8],
}: ChessBoardProps) {
    return (
        <Page>
            <Game name="chess_app_web" />
        </Page>
    );
}
