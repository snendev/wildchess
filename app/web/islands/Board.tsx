import { JSX } from "preact";
import { useEffect, useMemo, useLayoutEffect, useCallback, useRef, useState } from "preact/hooks";

function useChessBoard({
  boardRef,
  selectSquare,
  movePiece,
  getTargets,
  getPosition,
  getIcons,
  tick,
}): Object | null {
  const [board, setBoard] = useState(null);
  const [selectedSquare, setSelectedSquare] = useState(null);

  const targetedSquares = useMemo(
    () => selectedSquare ? getTargets(selectedSquare) : null,
    [getTargets, selectedSquare, tick],
  );

  const handleClickEmptySquare = useCallback((square) => {
    // TODO: need to incorporate selecting a targeted square to execute moves
    // but it would be bad for this to take stateful dependencies because
    // we would reinitialize the board every time
    selectSquare(null);
  }, [selectSquare]);

  const handleDragStart = useCallback((source) => {
    selectSquare(source);
  }, [selectSquare]);

  const handleDrop = useCallback((source, target, piece, newPos, oldPos, orientation) => {
    const result = movePiece(source, target) ? "drop" : "snapback";
    if (result === "drop") selectSquare(null);
    return result;
  }, [movePiece, selectSquare]);

  // TODO:
  const icons = useMemo(() => getIcons(), [getIcons]);

  const config = useMemo(() => ({
    position: getPosition() ?? 'start',
    dropOffBoard: "snapback",
    draggable: true,
    onDragStart: handleDragStart,
    onDrop: handleDrop,
    onClickEmptySquare: handleClickEmptySquare,
    pieceTheme: icons ? (piece) => icons[piece] : undefined,
  }), [
    getPosition,
    handleDrop,
    icons,
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
  size: number
  dimensions?: [number, number]
  // map from algebraic square name to piece name (e.g. 'wK', 'bN', 'bP')
  getPosition: () => Record<string, string>
  getIcons: () => Record<string, string>
  getTargets: () => string[],
  lastMoveSquares: [string, string] | null,
  movePiece: (source: string, target: string) => "success" | "failure"
  tick: number
}

export default function Board({
  size = 800,
  dimensions = [8, 8],
  getIcons,
  getTargets,
  getPosition,
  lastMoveSquares,
  movePiece,
  tick,
}: ChessBoardProps): JSX.Element {
  const boardRef = useRef(null);
  const [selectedSquare, setSelectedSquare] = useState(null);
  const board = useChessBoard({
    boardRef,
    selectSquare: setSelectedSquare,
    movePiece,
    getTargets,
    getPosition,
    getIcons,
    tick,
  });

  const targetedSquares = useMemo(
    () => selectedSquare ? getTargets(selectedSquare) : null,
    [selectedSquare],
  );

  // manage highlights on the selected square
  useHighlighter(boardRef, board, 'state', useMemo(() => selectedSquare ? [selectedSquare] : null, [selectedSquare]));
  // and any targeted squares
  useHighlighter(boardRef, board, 'state', targetedSquares);
  // and the last move squares
  useHighlighter(boardRef, board, 'state', lastMoveSquares);
  // and finally the target squares
  // TODO: dots for moves and circles for attacks, instead of backgrounds
  // could use more-transparent circles for unavailable attack squares
  useHighlighter(boardRef, board, 'targets', targetedSquares);

  return <div ref={boardRef} style={`width: ${size}px`} />;
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
}

function removeHighlight(element: Element, highlight: HighlightKind, square_color: Color) {
  const classSet = new Set(element.classList);
  classSet.delete(getHighlightClass(highlight, square_color));
  element.className = Array.from(classSet).join(" ");
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
  board: Object | null,
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