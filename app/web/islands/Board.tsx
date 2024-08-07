import { JSX } from "preact";
import { useEffect, useMemo, useLayoutEffect, useCallback, useRef, useState } from "preact/hooks";
import { GameActions, GameState } from "../game/useWasmGame.ts";

interface ChessBoardControls {
  boardRef: {current: Element | null},
  // map from algebraic square name to piece name (e.g. 'wK', 'bN', 'bP')
  position: Record<string, string> | null
  icons: Record<string, string> | null
  orientation: "white" | "black",
  playMove: (source: string, target: string) => boolean,
  requestTargets: (source: string) => void,
  resetTargets: () => void,
}

function useChessBoard({
  boardRef,
  position,
  icons,
  orientation,
  requestTargets,
  resetTargets,
  playMove,
}: ChessBoardControls): unknown | null {
  const [board, setBoard] = useState(null);

  const handleClickEmptySquare = useCallback(() => {
    resetTargets();
  }, [resetTargets]);

  const handleDragStart = useCallback((source: string) => {
    requestTargets(source);
  }, [requestTargets]);

  const handleDrop = useCallback((source: string, target: string) => {
    if (playMove(source, target)) {
      resetTargets();
    }
  }, [playMove, resetTargets]);

  const config = useMemo(() => ({
    position: position ?? 'start',
    orientation,
    dropOffBoard: "snapback",
    draggable: true,
    onDragStart: handleDragStart,
    onDrop: handleDrop,
    onClickEmptySquare: handleClickEmptySquare,
    pieceTheme: icons ? (piece: string) => icons[piece] : DEFAULT_PIECE_THEME,
  }), [
    position,
    handleDrop,
    icons,
    orientation,
    handleClickEmptySquare,
  ]);

  useEffect(() => {
    async function initBoard() {
      const { Chessboard } = await import("chessboardjs");
      setBoard(new Chessboard(boardRef.current, config));
    }
    initBoard();
  }, [config]);

  return board
}

interface ChessBoardProps {
  size?: number
  dimensions?: [number, number]
}

export default function Board({
  size,
  dimensions = [8, 8],
  position,
  icons,
  orientation,
  targetSquares,
  lastMoveSquares,
  playMove,
  requestTargets,
  resetTargets,
}: ChessBoardProps & GameState & GameActions): JSX.Element {
  const boardRef = useRef<Element>(null);
  const [selectedSquare, setSelectedSquare] = useState<string | null>(null);
  const selectPiece = useCallback((square: string) => {
    setSelectedSquare(square);
    requestTargets(square);
  }, [])

  const board = useChessBoard({
    boardRef,
    position,
    icons,
    orientation: orientation as "white" | "black",
    playMove,
    requestTargets: selectPiece,
    resetTargets,
  });

  // manage highlights on the selected square
  useHighlighter(boardRef, board, 'state', useMemo(() => selectedSquare ? [selectedSquare] : null, [selectedSquare]));
  // and any targeted squares
  useHighlighter(boardRef, board, 'state', targetSquares);
  // and the last move squares
  useHighlighter(boardRef, board, 'state', lastMoveSquares);
  // and finally the target squares
  // TODO: dots for moves and circles for attacks, instead of backgrounds
  // could use more-transparent circles for unavailable attack squares
  useHighlighter(boardRef, board, 'targets', targetSquares);

  const width = size ? `${size}px` : "100%";

  return (
    <div
      ref={boardRef}
      class="flex flex-row justify-end"
      style={`width: ${width}`}
      {...(position == null ? ({}) : ({["data-testid"]: "chessboard"}))}
    />
  );
}

// probably should implement our own light or dark square checker
// but this is an easy hack for now since chessboardjs classnames are static
const LIGHT_SQUARE_CLASSNAME = "white-1e1d7";
const DARK_SQUARE_CLASSNAME = "black-3c85d";
function isSquareLightOrDark(node: Element) {
  if (node.classList.contains(DARK_SQUARE_CLASSNAME)) return "dark"
  if (node.classList.contains(LIGHT_SQUARE_CLASSNAME)) return "light"
  throw new Error("Node is neither light nor dark square");
}

const LIGHT_SQUARE_HIGHLIGHT_CLASS = "bg-amber-200";
const DARK_SQUARE_HIGHLIGHT_CLASS = "bg-amber-300/85";
const LIGHT_SQUARE_TARGET_CLASS = "bg-blue-200";

type Color = "light" | "dark";
type HighlightKind = "targets" | "state";
function getHighlightClass(highlight: HighlightKind, square_color: Color): string {
  switch (highlight) {
    case 'state': {
      switch (square_color) {
        case 'light': return LIGHT_SQUARE_HIGHLIGHT_CLASS
        case 'dark': return DARK_SQUARE_HIGHLIGHT_CLASS
        default: throw new Error("Getting a bad highlight class config for 'state': " + square_color);
      }
    }
    case 'targets': {
      return LIGHT_SQUARE_TARGET_CLASS
    }
    default: throw new Error("Getting a bad highlight class config: " + highlight);
  }
}

function addHighlight(element: Element, highlight: HighlightKind, square_color: Color) {
  element.className = [
    getHighlightClass(highlight, square_color),
    ...element.classList,
  ].join(" ");
  if (highlight === 'targets') {
    element.setAttribute('data-gamestate', 'target');
  }
}

function removeHighlight(element: Element, highlight: HighlightKind, square_color: Color) {
  const classSet = new Set(element.classList);
  classSet.delete(getHighlightClass(highlight, square_color));
  element.className = Array.from(classSet).join(" ");
  if (highlight === 'targets') {
    element.removeAttribute('data-gamestate');
  }
}

function getSquareNode(rootElement: Element, square: string): Element | null {
  return rootElement.querySelector(`[data-square="${square}"]`) ?? null;
}

// A useLayoutEffect that manages adding and removing highlight classes for square nodes.
// It"s not generally good to manage state in this effectful way, but we cannot control the board
// nodes directly, so this works fine.
function useHighlighter(
  // TODO: get the preact Ref type
  rootRef: { current: Element | null },
  board: unknown | null,
  highlight: HighlightKind,
  squaresToHighlight: string[] | null,
) {
  useLayoutEffect(() => {
    // useLayoutEffect runs after refs are assigned
    if (!squaresToHighlight || !rootRef.current!) {
      return undefined;
    }
    // only do this operation after the chessboardjs instance exists
    if (!board) {
      return undefined;
    }
    const rootElement = rootRef.current;
    for (const square of squaresToHighlight) {
      const node = getSquareNode(rootElement, square);
      if (node) {
        const squareColor = isSquareLightOrDark(node);
        addHighlight(node, highlight, squareColor);
      }
    }
    return () => {
      for (const square of squaresToHighlight) {
        const node = getSquareNode(rootElement, square);
        if (node) {
          const squareColor = isSquareLightOrDark(node);
          removeHighlight(node, highlight, squareColor);
        }
      }
    }
  }, [highlight, squaresToHighlight, board]);
}

const DEFAULT_PIECE_THEME = `<img src="https://raw.githubusercontent.com/oakmac/chessboardjs/master/website/img/chesspieces/wikipedia/{piece}.png" />`
