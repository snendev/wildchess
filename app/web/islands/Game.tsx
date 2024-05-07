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
  | { kind: 'targets', source: string, targets: string[] }

type SendMessage =
  | { kind: 'setup-board' }
  | { kind: 'remove-board' }
  | { kind: 'play-move', source: string, target: string }
  | { kind: 'request-targets', source: string }

function sendMessage(worker: Worker, message: SendMessage) {
  worker.postMessage(message);
}

function useWasmGame(game_name: string) {
  const [position, setPosition] = useState<Record<string, string> | null>(null);
  const [icons, setIcons] = useState<Record<string, string> | null>(null);
  const [targetSquares, setTargetSquares] = useState<string[] | null>(null);
  const [lastMoveSquares, setLastMoveSquares] = useState<[string, string] | null>(null);

  const worker = useMemo(() => {
    const worker = new Worker(
      new URL("/js/workers/wasm.js", import.meta.url).href,
    );
    setTimeout(() => {
      sendMessage(worker, {kind: "setup-board"});
    }, 1000);
    worker.onmessage = (event: MessageEvent<RecvMessage>) => {
      switch (event.data.kind) {
        case "piece-icons": {
          setIcons(event.data.icons);
          return;
        }
        case "position": {
            console.log(event.data.position);
          setPosition(event.data.position);
          if (event.data.lastMove !== undefined) setLastMoveSquares(event.data.lastMove ?? null);
          return;
        }
        case "targets": {
          setTargetSquares(event.data.targets);
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
    sendMessage(worker, {kind: 'setup-board'});
  }, [worker]);

  const requestTargets = useCallback((source: string) => {
    sendMessage(worker, {kind: 'request-targets', source});
  }, [worker])

  const resetTargets = useCallback(() => {
    setTargetSquares(null);
  }, [worker])

  const playMove = useCallback((source: string, target: string) => {
    sendMessage(worker, {kind: 'play-move', source, target});
  }, [worker]);

  return { position, icons, targetSquares, lastMoveSquares, setupBoard, requestTargets, resetTargets, playMove };
}
