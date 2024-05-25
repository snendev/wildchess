import { useState, useMemo, useCallback, } from "preact/hooks";

export type NetworkState = "not-connected" | "connected" | "awaiting-game" | "in-game"
export type GameVariant = "featured-1" | "featured-2" | "featured-3" | "wild"
export type GameClock = "classical" | "rapid" | "blitz" | "bullet"

export type RecvMessage =
  | { kind: 'network-state', state: NetworkState }
  | { kind: 'piece-icons', icons: Record<string, string> }
  | { kind: 'require-promotion', icons: string[] }
  | { kind: 'position', position: Record<string, string>, lastMove: [string, string] | null | undefined }
  | { kind: 'targets', source: string, targets?: string[] }
  | { kind: 'player-count', count: number }
  | { kind: 'orientation', orientation: 'white' | 'black'}
  | { kind: 'gameover', winningTeam: 'white' | 'black' }
  | { kind: 'clocks', clocks: { white: string, black: string }}

export type SendMessage =
  | { kind: 'request-game', variant: GameVariant | null, clock: GameClock | null }
  | { kind: 'play-move', source: string, target: string }
  | { kind: 'select-promotion', promotionIndex: number }
  | { kind: 'request-targets', source: string }
  | { kind: 'remove-board' }
  | { kind: 'leave-game' }

export interface GameState {
  position: Record<string, string> | null
  clocks: {white: string, black: string} | null
  icons:  Record<string, string> | null
  targetSquares: string[] | null
  lastMoveSquares: [string, string] | null
  orientation: "white" | "black"
}

export interface GameActions {
  requestTargets: (source: string) => void
  resetTargets: () => void
  playMove: (source: string, target: string) => void
}

export interface GameMenuState {
  netState: NetworkState
  promotionIcons: string[] | null
  winner: "white" | "black" | null
}

export interface GameMenuActions {
  requestGame: (variant: GameVariant | null, clock: GameClock | null) => void
  leaveGame: () => void
  selectPromotion: (promotionIndex: number) => void
}

export interface WasmGameData {
  boardState: GameState,
  boardActions: GameActions,
  menuState: GameMenuState,
  menuActions: GameMenuActions,
}

function sendMessage(worker: Worker, message: SendMessage) {
  worker.postMessage(message);
}

export default function useWasmGame(): WasmGameData {
  const [netState, setNetState] = useState<NetworkState>("not-connected");
  const [position, setPosition] = useState<Record<string, string> | null>(null);
  const [clocks, setClocks] = useState<{white: string, black: string} | null>(null);
  const [orientation, setOrientation] = useState<"white" | "black">("white");
  const [icons, setIcons] = useState<Record<string, string> | null>(null);
  const [targetSquares, setTargetSquares] = useState<string[] | null>(null);
  const [lastMoveSquares, setLastMoveSquares] = useState<[string, string] | null>(null);
  const [promotionIcons, setPromotionIcons] = useState<string[] | null>(null);
  const [winner, setWinner] = useState<"white" | "black" | null>(null);

  const worker = useMemo(() => {
    const worker = new Worker(
      new URL("/js/workers/wasm.js", import.meta.url).href,
    );

    worker.onmessage = (event: MessageEvent<RecvMessage>) => {
      switch (event.data.kind) {
        case "network-state": {
          setNetState(event.data.state);
          return;
        }
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
        case "orientation": {
          setOrientation(event.data.orientation);
          return;
        }
        case "require-promotion": {
          setPromotionIcons(event.data.icons);
          return;
        }
        case "gameover": {
          setWinner(event.data.winningTeam);
          return;
        }
        case "clocks": {
          setClocks(event.data.clocks);
          return;
        }
        default: {
          assertNever(event.data);
        }
      }
    };
    return worker;
  }, []);
    
  const requestGame = useCallback((variant: GameVariant | null, clock: GameClock | null) => {
    sendMessage(worker, {kind: 'request-game', variant, clock});
  }, [worker]);

  const leaveGame = useCallback(() => {
    sendMessage(worker, {kind: 'leave-game'});
  }, []);

  const requestTargets = useCallback((source: string) => {
    sendMessage(worker, {kind: 'request-targets', source});
  }, [worker])

  const resetTargets = useCallback(() => {
    setTargetSquares(null);
  }, [worker])

  const playMove = useCallback((source: string, target: string) => {
    sendMessage(worker, {kind: 'play-move', source, target});
  }, [worker]);

  const selectPromotion = useCallback((promotionIndex: number) => {
    sendMessage(worker, {kind: 'select-promotion', promotionIndex});
  }, [worker]);

  return {
    boardState: {
      position, icons, targetSquares, lastMoveSquares, orientation,
      clocks,
    },
    boardActions: {
      requestTargets, resetTargets, playMove,
    },
    menuState: {
      netState,
      promotionIcons,
      winner,
    },
    menuActions: { requestGame, leaveGame, selectPromotion }
  }
}

function assertNever<T>(flag: never): never {
  throw new Error(`Unexpected value found: ${JSON.stringify(flag)}`);
}
