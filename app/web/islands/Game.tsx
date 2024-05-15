import { IS_BROWSER } from "$fresh/runtime.ts";
import { useState, useMemo, useCallback, useRef, useEffect, VNode } from "preact/hooks";

import Board from "./Board.tsx";

interface WasmGameProps {
  name: string
  description: VNode,
}

export default function WasmGame({ name, description }: WasmGameProps) {
  if (!IS_BROWSER) {
    return <Board />;
  }

  return (
    <Board {...useWasmGame(name)} />
  );
}

type RecvMessage =
  | { kind: 'piece-icons', icons: Record<string, string> }
  | { kind: 'position', position: Record<string, string>, lastMove: [string, string] | null | undefined }
  | { kind: 'targets', source: string, targets?: string[] }
  | { kind: 'player-count', count: number }
  | { kind: 'orientation', orientation: 'white' | 'black'}

type SendMessage =
  | { kind: 'start-game' }
  | { kind: 'remove-board' }
  | { kind: 'play-move', source: string, target: string }
  | { kind: 'request-targets', source: string }

function sendMessage(worker: Worker, message: SendMessage) {
  worker.postMessage(message);
}

function useWasmGame(game_name: string) {
  const [position, setPosition] = useState<Record<string, string> | null>(null);
  const [orientation, setOrientation] = useState<string>('white');
  const [icons, setIcons] = useState<Record<string, string> | null>(null);
  const [targetSquares, setTargetSquares] = useState<string[] | null>(null);
  const [lastMoveSquares, setLastMoveSquares] = useState<[string, string] | null>(null);

  const worker = useMemo(() => {
    const worker = new Worker(
      new URL("/js/workers/wasm.js", import.meta.url).href,
    );
    setTimeout(() => {
      console.log("sending start signal")
      sendMessage(worker, {kind: "start-game"});
    }, 2000);
    worker.onmessage = (event: MessageEvent<RecvMessage>) => {
      switch (event.data.kind) {
        case "piece-icons": {
          setIcons(event.data.icons);
          return;
        }
        case "position": {
          setPosition(event.data.position);
          if (event.data.lastMove !== undefined) setLastMoveSquares(event.data.lastMove ?? null);
          return;
        }
        case "targets": {
          setTargetSquares(event.data.targets ?? null);
          return;
        }
        case "player-count": {
          console.log("player count: " + event.data.count);
          return;
        }
        case 'orientation': {
          setOrientation(event.data.orientation);
          return;
        }
        default: {
          throw new Error(`Unexpected message received from worker: ${JSON.stringify(event.data)}`);
        }
      }
    };
    return worker;
  }, []);
    
  const setupBoard = useCallback(() => {
    sendMessage(worker, {kind: 'start-game'});
  }, [worker]);

  const requestTargets = useCallback((source: string) => {
    sendMessage(worker, {kind: 'request-targets', source});
  }, [worker])

  const resetTargets = useCallback(() => {
    setTargetSquares(null);
  }, [worker])

  const playMove = useCallback((source: string, target: string) => {
    console.log({source, target});
    sendMessage(worker, {kind: 'play-move', source, target});
  }, [worker]);

  return { position, icons, targetSquares, lastMoveSquares, orientation, setupBoard, requestTargets, resetTargets, playMove };
}
