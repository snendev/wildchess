import { IS_BROWSER } from "$fresh/runtime.ts";
import { VNode } from "preact/hooks";

import useWasmGame from "../game/useWasmGame.ts";

import Board from "./Board.tsx";
import Lobby from "./Lobby.tsx";

interface GameManagerProps {
  description: VNode,
}

export default function GameManager({ description }: GameManagerProps) {
  if (!IS_BROWSER) {
    return <Lobby requestGame={() => {}} />;
  }

  const {state, requestGame, ...gameState} = useWasmGame();

  switch (state) {
    case "not-connected":
    case "connected" : {
      return (
        <Lobby requestGame={requestGame} />
      );
    }
    case "awaiting-game" : {
      return <div>Finding game...</div>;
    }
    case "in-game": {
      return <Board {...gameState} size={600} />;
    }
    default: throw new Error("Unexpected network state: " + state);
  }
}
