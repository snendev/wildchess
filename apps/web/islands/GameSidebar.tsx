import { JSX } from "preact";

import type { GameState,  GameMenuState, GameMenuActions } from "../game/useWasmGame.ts";

function History() {
  return (
    <div>
      <h2 class="text-base font-medium underline">Moves</h2>
      <p>WIP</p>
    </div>
  );
}

function ActionBar({ leaveGame }: Pick<GameMenuActions, 'leaveGame'>) {
  return (
    <div class="flex flex-row pr-2 justify-between items-center">
      {/* Board actions */}
      <div class="flex flex-row gap-1">
        {/* Resign */}
        <button
          type="button"
          class="shadow rounded p-4 bg-[#fdfbe8] cursor-not-allowed"
          disabled
        />
        {/* Draw offer*/}
        <button
          type="button"
          class="shadow rounded p-4 bg-[#fdfbe8] cursor-not-allowed"
          disabled
        />
      </div>

      {/* Move history actions */}
      <div class="flex flex-row gap-1">
        {/* Resign */}
        <button
          type="button"
          class="shadow rounded p-4 bg-[#fdfbe8] cursor-not-allowed"
          disabled
        />
        {/* Draw offer*/}
        <button
          type="button"
          class="shadow rounded p-4 bg-[#fdfbe8] cursor-not-allowed"
          disabled
        />
      </div>

      {/* TODO: only reveal when game is complete */}
      <button
        type="button"
        class="shadow rounded p-2 bg-[#ffefef]"
        onClick={() => leaveGame}
      >
        Leave
      </button>
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
export default function GameSidebar({ orientation, winner, leaveGame }: GameState & GameMenuState & GameMenuActions ): JSX.Element {
  if (winner) console.log(winner);
  const playerTurnMessage = orientation === "any" ? null : `You are playing ${orientation[0].toUpperCase()}${orientation.slice(1)}.`;
  return (
    <div class="w-[350px] h-min p-4 flex flex-col gap-3 text-sm bg-[#f3edd9] border-2 border-black">
      <History />
      <hr class="border-black" />
      {playerTurnMessage !== null && (
        <div>
            <h3 class="text-md">{playerTurnMessage}</h3>
        </div>
      )}
      {winner != null && (<div>
        <h3 class="text-lg">WINNER: {winner}</h3>
      </div>)}
      {winner != null && (<hr class="border-black" />)}
      <ActionBar leaveGame={leaveGame} />
      <hr class="border-black" />
      <Legend />
    </div>
  )
}
