import { JSX } from "preact";

import Page from "../components/Page.tsx";
import GameManager from "../islands/GameManager.tsx";

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
      <GameManager />
    </Page>
  );
}
