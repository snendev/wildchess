import { JSX } from "preact";
import { useState } from "preact/hooks";

import AboutBlurb from "../components/AboutBlurb.tsx";
import { GameClock, GameMenuActions, GameMenuState, GameVariant } from "../game/useWasmGame.ts";

import Board from "./Board.tsx";

export default function Lobby({
  netState,
  requestGame,
}: GameMenuState & GameMenuActions): JSX.Element {
  const [selectedVariant, setSelectedVariant] = useState<GameVariant | null>(null);
  const [selectedClock, setSelectedClock] = useState<GameClock | null>(null);

  const isNetDisabled = netState !== 'connected';
  // todo types for empty board
  const EmptyBoard = Board as any;
  return (
    <div class="flex flex-row gap-x-[5%]">
      <div class="min-w-[40%]">
        <EmptyBoard />
        <p class="mt-2 text-sm italic">(For now, pretend there's some wacky position on the board here!)</p>
      </div>
      <div class="min-w-[200px] h-min p-2 flex flex-col gap-3 bg-[#FFFBD4] border-2 border-black">
        <div class="w-full">
          <button
            role="button"
            class="w-full h-[80px] text-3xl shadow-lg bg-[#6fa6ff] rounded-2xl disabled:opacity-50 disabled:cursor-not-allowed"
            disabled={isNetDisabled}
            onClick={() => requestGame('online', selectedVariant, selectedClock)}
          >
            Play Online
          </button>
          {isNetDisabled && (
            <p class="w-full pt-1 pr-1 text-red-500 text-xs text-right">
              Failed to connect to server.
            </p>
          )}
        </div>
        <button
          role="button"
          class="w-full h-[80px] text-3xl shadow-lg bg-[#6fa6ff] rounded-2xl"
          onClick={() => requestGame('local', selectedVariant, selectedClock)}
        >
          Play Local
        </button>
        <div class="border-2 border-black">
          <div class="text-sm italic m-2 border-b-[1px] border-black">
            <h4>Play featured positions</h4>
          </div>
          <div class="flex flex-col gap-2 px-2 pb-2">
            <button
              class={`text-lg shadow rounded-lg py-1 ${selectedClassName(selectedVariant, "featured-1") ?? DEFAULT_BUTTON_BG}`}
              onClick={() => setSelectedVariant(setOrToggle<GameVariant>("featured-1"))}
            >
              Position 1
            </button>
            <button
              class={`text-lg shadow rounded-lg py-1 ${selectedClassName(selectedVariant, "featured-2") ?? DEFAULT_BUTTON_BG}`}
              onClick={() => setSelectedVariant(setOrToggle<GameVariant>("featured-2"))}
            >
              Position 2
            </button>
            <button
              class={`text-lg shadow rounded-lg py-1 ${selectedClassName(selectedVariant, "featured-3") ?? DEFAULT_BUTTON_BG}`}
              onClick={() => setSelectedVariant(setOrToggle<GameVariant>("featured-3"))}
            >
              Position 3
            </button>
            <button
              class={`text-lg shadow rounded-lg py-1 ${selectedClassName(selectedVariant, "wild") ?? DEFAULT_BUTTON_BG}`}
              onClick={() => setSelectedVariant(setOrToggle<GameVariant>("wild"))}
            >
              Wild Position
            </button>
          </div>
        </div>
        <div class="border-2 border-black">
          <div class="text-sm italic m-2 border-b-[1px] border-black">
            <h4>Time control</h4>
          </div>
          <div class="flex flex-col gap-2 px-2 pb-2">
            <button
              class={`text-lg shadow rounded-lg py-1 ${selectedClassName(selectedClock, "rapid") ?? DEFAULT_BUTTON_BG}`}
              onClick={() => setSelectedClock(setOrToggle<GameClock>("rapid"))}
            >
              Rapid
            </button>
            <button
              class={`text-lg shadow rounded-lg py-1 ${selectedClassName(selectedClock, "blitz") ?? DEFAULT_BUTTON_BG}`}
              onClick={() => setSelectedClock(setOrToggle<GameClock>("blitz"))}
            >
              Blitz
            </button>
          </div>
        </div>
      </div>
      <AboutBlurb />
    </div>
  )
}

function selectedClassName<T>(value: T, expected: T): string | undefined {
  return value === expected ? "bg-emerald-500" : undefined;
}

function setOrToggle<T>(value: T | null): (prevState: T | null) => T | null {
  return (prevState) => {
    if (prevState === value) {
      return null;
    } else {
      return value;
    }
  };
}

const DEFAULT_BUTTON_BG = "bg-[#a2ddcc]";
