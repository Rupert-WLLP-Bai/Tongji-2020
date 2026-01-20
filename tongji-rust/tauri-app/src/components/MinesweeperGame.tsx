import { useState, useEffect } from 'react';

interface MinesweeperGameProps {
  onBack: () => void;
}

type Difficulty = 'easy' | 'medium' | 'hard';

interface CellState {
  hasMine: boolean;
  isRevealed: boolean;
  isFlagged: boolean;
  adjacentMines: number;
}

interface GameState {
  board: CellState[][];
  width: number;
  height: number;
  mineCount: number;
  revealedCount: number;
  flagCount: number;
  gameOver: boolean;
  gameWon: boolean;
  firstClick: boolean;
}

export default function MinesweeperGame({ onBack }: MinesweeperGameProps) {
  const [difficulty, setDifficulty] = useState<Difficulty>('easy');
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [autoPlaying, setAutoPlaying] = useState(false);
  const [moveHistory, setMoveHistory] = useState<string[]>([]);

  // Difficulty settings
  const getDifficultySettings = (diff: Difficulty) => {
    switch (diff) {
      case 'easy':
        return { width: 9, height: 9, mines: 10 };
      case 'medium':
        return { width: 16, height: 16, mines: 40 };
      case 'hard':
        return { width: 30, height: 16, mines: 99 };
    }
  };

  // Initialize game
  const initGame = () => {
    const { width, height, mines } = getDifficultySettings(difficulty);
    const board: CellState[][] = [];

    for (let y = 0; y < height; y++) {
      board[y] = [];
      for (let x = 0; x < width; x++) {
        board[y][x] = {
          hasMine: false,
          isRevealed: false,
          isFlagged: false,
          adjacentMines: 0,
        };
      }
    }

    setGameState({
      board,
      width,
      height,
      mineCount: mines,
      revealedCount: 0,
      flagCount: 0,
      gameOver: false,
      gameWon: false,
      firstClick: true,
    });
    setMoveHistory([]);
    setAutoPlaying(false);
  };

  // Initialize on mount and difficulty change
  useEffect(() => {
    initGame();
  }, [difficulty]);

  // Generate mines (called after first click)
  const generateMines = (state: GameState, safeX: number, safeY: number) => {
    const newBoard = state.board.map(row => row.map(cell => ({ ...cell })));
    let minesPlaced = 0;

    while (minesPlaced < state.mineCount) {
      const x = Math.floor(Math.random() * state.width);
      const y = Math.floor(Math.random() * state.height);

      // Skip if already has mine or in safe zone (3x3 around first click)
      if (newBoard[y][x].hasMine) continue;
      if (Math.abs(x - safeX) <= 1 && Math.abs(y - safeY) <= 1) continue;

      newBoard[y][x].hasMine = true;
      minesPlaced++;
    }

    // Calculate adjacent mine counts
    for (let y = 0; y < state.height; y++) {
      for (let x = 0; x < state.width; x++) {
        if (!newBoard[y][x].hasMine) {
          newBoard[y][x].adjacentMines = countAdjacentMines(newBoard, x, y, state.width, state.height);
        }
      }
    }

    return newBoard;
  };

  // Count adjacent mines
  const countAdjacentMines = (board: CellState[][], x: number, y: number, width: number, height: number): number => {
    let count = 0;
    for (let dy = -1; dy <= 1; dy++) {
      for (let dx = -1; dx <= 1; dx++) {
        if (dx === 0 && dy === 0) continue;
        const nx = x + dx;
        const ny = y + dy;
        if (nx >= 0 && nx < width && ny >= 0 && ny < height && board[ny][nx].hasMine) {
          count++;
        }
      }
    }
    return count;
  };

  // Flood fill reveal
  const floodFill = (board: CellState[][], x: number, y: number, width: number, height: number): number => {
    if (x < 0 || x >= width || y < 0 || y >= height) return 0;
    if (board[y][x].isRevealed || board[y][x].isFlagged || board[y][x].hasMine) return 0;

    board[y][x].isRevealed = true;
    let revealed = 1;

    // If no adjacent mines, recursively reveal neighbors
    if (board[y][x].adjacentMines === 0) {
      for (let dy = -1; dy <= 1; dy++) {
        for (let dx = -1; dx <= 1; dx++) {
          if (dx === 0 && dy === 0) continue;
          revealed += floodFill(board, x + dx, y + dy, width, height);
        }
      }
    }

    return revealed;
  };

  // Handle cell click
  const handleCellClick = (x: number, y: number) => {
    if (!gameState || gameState.gameOver || gameState.gameWon || autoPlaying) return;
    if (gameState.board[y][x].isRevealed || gameState.board[y][x].isFlagged) return;

    let newBoard = gameState.board.map(row => row.map(cell => ({ ...cell })));
    let newState = { ...gameState };

    // First click: generate mines
    if (gameState.firstClick) {
      newBoard = generateMines(gameState, x, y);
      newState.firstClick = false;
    }

    // Check if clicked on mine
    if (newBoard[y][x].hasMine) {
      newBoard[y][x].isRevealed = true;
      newState.board = newBoard;
      newState.gameOver = true;
      setGameState(newState);
      setMoveHistory(prev => [...prev, `💥 Hit mine at (${x}, ${y})`]);
      return;
    }

    // Reveal cell(s)
    const revealed = floodFill(newBoard, x, y, gameState.width, gameState.height);
    newState.board = newBoard;
    newState.revealedCount += revealed;

    // Check win condition
    const totalCells = gameState.width * gameState.height;
    if (newState.revealedCount === totalCells - gameState.mineCount) {
      newState.gameWon = true;
    }

    setGameState(newState);
    setMoveHistory(prev => [...prev, `✓ Revealed (${x}, ${y}) - ${revealed} cells`]);
  };

  // Handle right click (flag)
  const handleCellRightClick = (e: React.MouseEvent, x: number, y: number) => {
    e.preventDefault();
    if (!gameState || gameState.gameOver || gameState.gameWon || autoPlaying) return;
    if (gameState.board[y][x].isRevealed) return;

    const newBoard = gameState.board.map(row => row.map(cell => ({ ...cell })));
    const cell = newBoard[y][x];

    if (cell.isFlagged) {
      cell.isFlagged = false;
      setGameState({
        ...gameState,
        board: newBoard,
        flagCount: gameState.flagCount - 1,
      });
      setMoveHistory(prev => [...prev, `🚩 Unflagged (${x}, ${y})`]);
    } else {
      cell.isFlagged = true;
      setGameState({
        ...gameState,
        board: newBoard,
        flagCount: gameState.flagCount + 1,
      });
      setMoveHistory(prev => [...prev, `🚩 Flagged (${x}, ${y})`]);
    }
  };

  // Auto play using simple strategy
  const autoPlay = async () => {
    if (!gameState || autoPlaying) return;

    setAutoPlaying(true);
    let currentState = { ...gameState };

    // First click: click center
    if (currentState.firstClick) {
      const centerX = Math.floor(currentState.width / 2);
      const centerY = Math.floor(currentState.height / 2);

      let newBoard = generateMines(currentState, centerX, centerY);
      const revealed = floodFill(newBoard, centerX, centerY, currentState.width, currentState.height);

      currentState = {
        ...currentState,
        board: newBoard,
        firstClick: false,
        revealedCount: revealed,
      };

      setGameState(currentState);
      setMoveHistory(prev => [...prev, `🤖 Auto: First click (${centerX}, ${centerY})`]);
      await new Promise(resolve => setTimeout(resolve, 500));
    }

    // Simple strategy: reveal safe cells and flag obvious mines
    while (!currentState.gameOver && !currentState.gameWon) {
      let madeMove = false;

      // Strategy 1: Find cells with all neighbors flagged, reveal remaining
      for (let y = 0; y < currentState.height && !madeMove; y++) {
        for (let x = 0; x < currentState.width && !madeMove; x++) {
          const cell = currentState.board[y][x];
          if (!cell.isRevealed || cell.adjacentMines === 0) continue;

          // Count flagged and hidden neighbors
          let flaggedCount = 0;
          let hiddenCells: [number, number][] = [];

          for (let dy = -1; dy <= 1; dy++) {
            for (let dx = -1; dx <= 1; dx++) {
              if (dx === 0 && dy === 0) continue;
              const nx = x + dx;
              const ny = y + dy;
              if (nx >= 0 && nx < currentState.width && ny >= 0 && ny < currentState.height) {
                const neighbor = currentState.board[ny][nx];
                if (neighbor.isFlagged) flaggedCount++;
                else if (!neighbor.isRevealed) hiddenCells.push([nx, ny]);
              }
            }
          }

          // If all mines are flagged, reveal remaining hidden cells
          if (flaggedCount === cell.adjacentMines && hiddenCells.length > 0) {
            const [revealX, revealY] = hiddenCells[0];
            const newBoard = currentState.board.map(row => row.map(c => ({ ...c })));

            if (newBoard[revealY][revealX].hasMine) {
              newBoard[revealY][revealX].isRevealed = true;
              currentState = { ...currentState, board: newBoard, gameOver: true };
              setGameState(currentState);
              setMoveHistory(prev => [...prev, `💥 Auto: Hit mine at (${revealX}, ${revealY})`]);
              setAutoPlaying(false);
              return;
            }

            const revealed = floodFill(newBoard, revealX, revealY, currentState.width, currentState.height);
            currentState = {
              ...currentState,
              board: newBoard,
              revealedCount: currentState.revealedCount + revealed,
            };

            setGameState(currentState);
            setMoveHistory(prev => [...prev, `🤖 Auto: Revealed (${revealX}, ${revealY})`]);
            madeMove = true;
            await new Promise(resolve => setTimeout(resolve, 300));
          }

          // If hidden cells equal remaining mines, flag them
          if (hiddenCells.length > 0 && flaggedCount + hiddenCells.length === cell.adjacentMines) {
            const [flagX, flagY] = hiddenCells[0];
            const newBoard = currentState.board.map(row => row.map(c => ({ ...c })));
            newBoard[flagY][flagX].isFlagged = true;

            currentState = {
              ...currentState,
              board: newBoard,
              flagCount: currentState.flagCount + 1,
            };

            setGameState(currentState);
            setMoveHistory(prev => [...prev, `🤖 Auto: Flagged (${flagX}, ${flagY})`]);
            madeMove = true;
            await new Promise(resolve => setTimeout(resolve, 300));
          }
        }
      }

      // If no logical move found, make a random safe guess
      if (!madeMove) {
        const hiddenCells: [number, number][] = [];
        for (let y = 0; y < currentState.height; y++) {
          for (let x = 0; x < currentState.width; x++) {
            if (!currentState.board[y][x].isRevealed && !currentState.board[y][x].isFlagged) {
              hiddenCells.push([x, y]);
            }
          }
        }

        if (hiddenCells.length === 0) break;

        const [guessX, guessY] = hiddenCells[Math.floor(Math.random() * hiddenCells.length)];
        const newBoard = currentState.board.map(row => row.map(c => ({ ...c })));

        if (newBoard[guessY][guessX].hasMine) {
          newBoard[guessY][guessX].isRevealed = true;
          currentState = { ...currentState, board: newBoard, gameOver: true };
          setGameState(currentState);
          setMoveHistory(prev => [...prev, `💥 Auto: Hit mine at (${guessX}, ${guessY})`]);
          setAutoPlaying(false);
          return;
        }

        const revealed = floodFill(newBoard, guessX, guessY, currentState.width, currentState.height);
        currentState = {
          ...currentState,
          board: newBoard,
          revealedCount: currentState.revealedCount + revealed,
        };

        setGameState(currentState);
        setMoveHistory(prev => [...prev, `🤖 Auto: Guessed (${guessX}, ${guessY})`]);
        await new Promise(resolve => setTimeout(resolve, 300));
      }

      // Check win condition
      const totalCells = currentState.width * currentState.height;
      if (currentState.revealedCount === totalCells - currentState.mineCount) {
        currentState.gameWon = true;
        setGameState(currentState);
        break;
      }
    }

    setAutoPlaying(false);
  };

  // Get cell display
  const getCellDisplay = (cell: CellState) => {
    if (cell.isFlagged) return '🚩';
    if (!cell.isRevealed) return '';
    if (cell.hasMine) return '💣';
    if (cell.adjacentMines === 0) return '';
    return cell.adjacentMines.toString();
  };

  // Get cell color
  const getCellColor = (cell: CellState) => {
    if (!cell.isRevealed) return 'bg-gray-600 hover:bg-gray-500';
    if (cell.hasMine) return 'bg-red-600';
    if (cell.adjacentMines === 0) return 'bg-gray-800';

    const colors = [
      '', // 0
      'text-blue-400', // 1
      'text-green-400', // 2
      'text-red-400', // 3
      'text-purple-400', // 4
      'text-yellow-400', // 5
      'text-cyan-400', // 6
      'text-pink-400', // 7
      'text-gray-400', // 8
    ];
    return `bg-gray-800 ${colors[cell.adjacentMines]}`;
  };

  if (!gameState) return null;

  const cellSize = difficulty === 'hard' ? 20 : difficulty === 'medium' ? 25 : 30;

  return (
    <div className="flex flex-col h-full bg-gray-900 text-white">
      {/* Header */}
      <div className="p-4 bg-gray-800 border-b border-gray-700">
        <div className="flex items-center justify-between mb-4">
          <button
            onClick={onBack}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors"
          >
            ← Back
          </button>
          <h1 className="text-2xl font-bold">Minesweeper</h1>
          <div className="text-right">
            <div className="text-sm text-gray-400">Mines</div>
            <div className="text-2xl font-bold">{gameState.mineCount - gameState.flagCount}</div>
            <div className="text-xs text-gray-500">Revealed: {gameState.revealedCount}</div>
          </div>
        </div>

        {/* Controls */}
        <div className="flex gap-4 items-center flex-wrap">
          <div className="flex items-center gap-2">
            <label className="text-sm">Difficulty:</label>
            <select
              value={difficulty}
              onChange={(e) => setDifficulty(e.target.value as Difficulty)}
              disabled={autoPlaying}
              className="px-3 py-1 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="easy">Easy (9×9)</option>
              <option value="medium">Medium (16×16)</option>
              <option value="hard">Hard (30×16)</option>
            </select>
          </div>

          <button
            onClick={initGame}
            disabled={autoPlaying}
            className="px-4 py-1 bg-green-600 hover:bg-green-700 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Reset
          </button>

          <button
            onClick={autoPlay}
            disabled={autoPlaying || gameState.gameOver || gameState.gameWon}
            className="px-4 py-1 bg-purple-600 hover:bg-purple-700 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            {autoPlaying ? (
              <>
                <span className="animate-spin">⚙️</span>
                Auto Playing...
              </>
            ) : (
              '🤖 Auto Play'
            )}
          </button>
        </div>
      </div>

      {/* Game Board */}
      <div className="flex-1 flex items-stretch p-4 gap-4 overflow-auto">
        {/* Board Container */}
        <div className="flex-1 flex items-center justify-center">
          <div className="relative">
            <div
              className="inline-grid gap-0.5 bg-gray-700 p-1 rounded"
              style={{
                gridTemplateColumns: `repeat(${gameState.width}, ${cellSize}px)`,
              }}
            >
              {gameState.board.map((row, y) =>
                row.map((cell, x) => (
                  <button
                    key={`${x}-${y}`}
                    onClick={() => handleCellClick(x, y)}
                    onContextMenu={(e) => handleCellRightClick(e, x, y)}
                    disabled={autoPlaying}
                    className={`
                      ${getCellColor(cell)}
                      border border-gray-700
                      font-bold text-sm
                      transition-colors
                      disabled:cursor-not-allowed
                    `}
                    style={{
                      width: `${cellSize}px`,
                      height: `${cellSize}px`,
                    }}
                  >
                    {getCellDisplay(cell)}
                  </button>
                ))
              )}
            </div>

            {/* Game Over Overlay */}
            {(gameState.gameOver || gameState.gameWon) && (
              <div className="absolute inset-0 flex items-center justify-center bg-black bg-opacity-75 rounded-lg">
                <div className="text-center">
                  <h2 className={`text-4xl font-bold mb-4 ${gameState.gameWon ? 'text-green-400' : 'text-red-400'}`}>
                    {gameState.gameWon ? '🎉 You Won!' : '💥 Game Over!'}
                  </h2>
                  <p className="text-xl mb-2">Revealed: {gameState.revealedCount} cells</p>
                  <p className="text-sm text-gray-400 mb-4">
                    {gameState.gameWon ? 'All mines avoided!' : 'Hit a mine!'}
                  </p>
                  <button
                    onClick={initGame}
                    className="px-6 py-2 bg-green-600 hover:bg-green-700 rounded-lg transition-colors"
                  >
                    Play Again
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Move History Panel */}
        {moveHistory.length > 0 && (
          <div className="w-64 bg-gray-800 border border-gray-700 rounded-lg p-4 flex flex-col max-h-full">
            <h3 className="text-lg font-bold mb-3 text-gray-300">Move History</h3>
            <div className="flex-1 overflow-y-auto space-y-1 text-sm font-mono min-h-0">
              {moveHistory.map((move, index) => (
                <div
                  key={index}
                  className={`p-2 rounded ${
                    index === moveHistory.length - 1
                      ? 'bg-blue-900/50 text-blue-300 border border-blue-700'
                      : 'bg-gray-700/50 text-gray-400'
                  }`}
                >
                  {move}
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Instructions */}
      <div className="p-4 bg-gray-800 border-t border-gray-700">
        <div className="text-sm text-gray-400 text-center">
          <span>
            Left click to reveal, right click to flag. 🚩 = Flag, 💣 = Mine. Numbers show adjacent mines.
          </span>
        </div>
      </div>
    </div>
  );
}
