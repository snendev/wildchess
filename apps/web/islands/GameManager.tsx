import { VNode } from "preact";
import { IS_BROWSER } from "$fresh/runtime.ts";

import Clock from "../components/Clock.tsx"
import useWasmGame from "../game/useWasmGame.ts";

import Board from "./Board.tsx";
import Lobby from "./Lobby.tsx";
import GameSidebar from "./GameSidebar.tsx";
import PromotionPieces from "./PromotionPieces.tsx";

interface GameManagerProps {
  description: VNode,
  useDev?: boolean
}

export default function GameManager({ description, useDev }: GameManagerProps) {
  if (!IS_BROWSER) {
    return <Lobby requestGame={() => {}} />;
  }

  const { boardState, boardActions, menuState, menuActions } = useWasmGame(useDev);
  const { clocks, orientation } = boardState;
  const { netState, promotionIcons } = menuState;
  const { requestGame, selectPromotion } =  menuActions;

  switch (netState) {
    case "not-connected":
    case "connected" : {
      return (
        <Lobby {...menuState} {...menuActions} />
      );
    }
    case "awaiting-game" : {
      return <div>Finding game...</div>;
    }
    case "in-game": {
      return (
        <div class="flex flex-col gap-2 lg:flex-row">
          <div class="flex flex-col items-end gap-2">
            <div>
                {boardState.orientation !== "any" && boardState.currentTurn === boardState.orientation && (
                    <h2 class="text-lg text-bold">MY TURN</h2>
                )}
            </div>
            {clocks && (
              <Clock time={clocks[orientation === "white" ? "black" : "white"]} />
            )}
            <Board {...boardState} {...boardActions} size={600} />
            {clocks && (
              <Clock time={clocks[orientation === "any" ? boardState.currentTurn : orientation]} />
            )}
          </div>
          <PromotionPieces icons={promotionIcons} selectIcon={selectPromotion} />
          <GameSidebar {...boardState} {...menuState} {...menuActions} />
        </div>
      );
    }
    default: throw new Error("Unexpected network state: " + netState);
  }
}
