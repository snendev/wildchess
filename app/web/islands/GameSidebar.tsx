import { JSX } from "preact";

import type { GameState } from "../game/useWasmGame.ts";

function History() {
  return (
    <div>
      <h2 class="text-base font-medium underline">Moves</h2>
      <p>WIP</p>
    </div>
  );
}

function ActionBar() {
  return (
    <div>
    </div>
  );
}

function Legend() {
  return (
    <div>
      <h2 class="text-base font-medium underline">About the Icons</h2>
      <p>Piece icons are uniquely generated to provide a clue for how the piece moves.</p>
      <ul class="p-2 list-disc list-inside">
        <li>Dots are arranged to hint at the location of an available target square.</li>
        <li>Arrows imply that a direction can be followed for any range.</li>
        <li>
          Black symbols imply that the target square can be used as a typical piece movement,
          either to move an empty square or capture an enemy piece on that square.
        </li>
        <li>Blue implies that the move cannot capture.</li>
        <li>Red implies that it can <i>only</i> be used to capture.</li>
        <li>
          Finally, an X implies an <i>en passant</i> target, meaning it will capture any target
          that passed through that square on the previous turn.
        </li>
      </ul>
    </div>
  );
}
export default function GameSidebar({ }: GameState): JSX.Element {
  return (
    <div class="w-[350px] h-min p-2 flex flex-col gap-3 text-sm bg-[#FFFBD4] border-2 border-black">
      <History />
      <hr />
      <ActionBar />
      <hr />
      <Legend />
    </div>
  )
}
