import { VNode } from "preact";
import { IS_BROWSER } from "$fresh/runtime.ts";

import useWasmGame from "../game/useWasmGame.ts";

import Board from "./Board.tsx";
import Lobby from "./Lobby.tsx";
import GameSidebar from "./GameSidebar.tsx";
import PromotionPieces from "./PromotionPieces.tsx";

interface GameManagerProps {
  description: VNode,
}

export default function GameManager({ description }: GameManagerProps) {
  if (!IS_BROWSER) {
    return <Lobby requestGame={() => {}} />;
  }

  const { boardState, boardActions, menuState, menuActions } = useWasmGame();
  const { netState, promotionIcons } = menuState;
  const { requestGame, selectPromotion } =  menuActions;

  switch (netState) {
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
      return (
        <div class="flex flex-row gap-2">
          <Board {...boardState} {...boardActions} size={600} />
          <PromotionPieces icons={promotionIcons} selectIcon={selectPromotion} />
          <GameSidebar {...boardState} />
        </div>
      );
    }
    default: throw new Error("Unexpected network state: " + netState);
  }
}
