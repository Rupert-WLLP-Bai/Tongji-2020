import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface MinesweeperGameProps {
  onBack: () => void;
}

type Difficulty = 'Easy' | 'Medium' | 'Hard';

interface CellInfo {
  hasMine: boolean;
  isRevealed: boolean;
  isFlagged: boolean;
  adjacentMines: number;
}

interface GameStateInfo {
  board: CellInfo[][];
  width: number;
  height: number;
  mineCount: number;
  flagCount: number;
  gameOver: boolean;
  gameWon: boolean;
}

export default function MinesweeperGame({ onBack }: MinesweeperGameProps) {
  const [difficulty, setDifficulty] = useState<Difficulty>('Easy');
  const [gameState, setGameState] = useState<GameStateInfo | null>(null);

  const [autoSolving, setAutoSolving] = useState(false);

  const initGame = async () => {
    setAutoSolving(false);
    try {
      const state = await invoke<GameStateInfo>('init_minesweeper', { difficulty });
      setGameState(state);
    } catch (error) {
      console.error('Failed to init game:', error);
    }
  };

  useEffect(() => {
    initGame();
  }, [difficulty]);

  useEffect(() => {
    let timer: number;
    if (autoSolving && gameState && !gameState.gameOver && !gameState.gameWon) {
      timer = window.setTimeout(async () => {
        try {
          const state = await invoke<GameStateInfo>('auto_solve_step');
          setGameState(state);
        } catch (error) {
          console.error('Auto solve error:', error);
          setAutoSolving(false);
        }
      }, 200);
    } else if (gameState?.gameOver || gameState?.gameWon) {
      setAutoSolving(false);
    }
    return () => clearTimeout(timer);
  }, [autoSolving, gameState]);

  const handleCellClick = async (x: number, y: number) => {
    if (!gameState || gameState.gameOver || gameState.gameWon || autoSolving) return;
    try {
      const state = await invoke<GameStateInfo>('handle_click', { x, y });
      setGameState(state);
    } catch (error) {
      console.error('Failed to handle click:', error);
    }
  };

  const handleCellRightClick = async (e: React.MouseEvent, x: number, y: number) => {
    e.preventDefault();
    if (!gameState || gameState.gameOver || gameState.gameWon || autoSolving) return;
    try {
      const state = await invoke<GameStateInfo>('handle_flag', { x, y });
      setGameState(state);
    } catch (error) {
      console.error('Failed to handle flag:', error);
    }
  };

  const handleAutoSolve = () => {
    if (!gameState || gameState.gameOver || gameState.gameWon) return;
    setAutoSolving(!autoSolving);
  };

  const getCellDisplay = (cell: CellInfo) => {
    if (cell.isRevealed && cell.hasMine) return '💣';
    if (cell.isFlagged) return 'F';
    if (!cell.isRevealed) return '';
    if (cell.adjacentMines === 0) return '';
    return cell.adjacentMines.toString();
  };

  const getCellStyles = (cell: CellInfo) => {
    const isGameEnded = gameState?.gameOver || gameState?.gameWon;

    // At game end, show incorrectly flagged cells in yellow
    if (isGameEnded && cell.isFlagged && !cell.hasMine) {
      return 'bg-yellow-500 text-white shadow-inner';
    }

    if (!cell.isRevealed) {
      // If flagged, show the flag with visible text
      if (cell.isFlagged) {
        return 'bg-slate-200 hover:bg-slate-300 text-red-600 font-black shadow-[inset_-3px_-3px_0px_rgba(0,0,0,0.08),inset_3px_3px_0px_rgba(255,255,255,0.6)]';
      }
      return 'bg-slate-200 hover:bg-slate-300 text-transparent shadow-[inset_-3px_-3px_0px_rgba(0,0,0,0.08),inset_3px_3px_0px_rgba(255,255,255,0.6)]';
    }

    // At game end, distinguish between correctly flagged and unflagged mines
    if (cell.hasMine && cell.isFlagged) {
      return 'bg-green-500 text-white shadow-inner';
    }
    if (cell.hasMine) {
      return 'bg-red-500 text-white shadow-inner';
    }

    const colors = [
      'text-transparent',
      'text-[#0000FF]',
      'text-[#008000]',
      'text-[#FF0000]',
      'text-[#000080]',
      'text-[#800000]',
      'text-[#008080]',
      'text-[#000000]',
      'text-[#808080]',
    ];

    if (cell.adjacentMines === 0) {
      return `bg-slate-50 ${colors[0]} shadow-inner`;
    }

    return `bg-white ${colors[cell.adjacentMines]} shadow-sm`;
  };

  if (!gameState) return null;

  const cellSize = difficulty === 'Hard' ? 24 : difficulty === 'Medium' ? 32 : 40;
  const fontSize = difficulty === 'Hard' ? 'text-2xl' : difficulty === 'Medium' ? 'text-3xl' : 'text-4xl';

  return (
    <div className="flex flex-col h-full bg-slate-50 text-slate-900 font-sans">
      <div className="py-8 px-10 bg-white border-b border-slate-200">
        <div className="max-w-5xl mx-auto flex items-center justify-between">
          <div className="flex items-center gap-6">
            <button
              onClick={onBack}
              className="p-3 hover:bg-slate-50 rounded-none transition-all text-slate-400 hover:text-blue-600 border border-transparent hover:border-slate-100"
              title="Back"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="m15 18-6-6 6-6"/></svg>
            </button>
            <div>
              <h1 className="text-3xl font-black tracking-tight text-slate-900 uppercase">Minesweeper</h1>
              <p className="text-slate-400 text-sm font-bold uppercase tracking-widest">Backend Logic Powered</p>
            </div>
          </div>
          
          <div className="flex items-center gap-3 bg-slate-900 px-8 py-4 shadow-xl shadow-slate-900/10 border border-slate-900">
            <div className="text-center">
              <div className="text-[10px] uppercase tracking-[0.2em] font-black text-slate-500 mb-1">Mines Left</div>
              <div className="text-3xl font-black text-white tabular-nums leading-none">
                {String(Math.max(0, gameState.mineCount - gameState.flagCount)).padStart(2, '0')}
              </div>
            </div>
          </div>
        </div>

        <div className="mt-10 flex gap-0 items-center justify-center max-w-5xl mx-auto">
          <select
            value={difficulty}
            onChange={(e) => setDifficulty(e.target.value as Difficulty)}
            className="px-8 py-4 bg-slate-50 border border-slate-200 text-base font-bold focus:ring-2 focus:ring-blue-500 focus:outline-none transition-all shadow-sm cursor-pointer appearance-none min-w-[160px] text-center"
          >
            <option value="Easy">Beginner</option>
            <option value="Medium">Intermediate</option>
            <option value="Hard">Expert</option>
          </select>

          <button
            onClick={initGame}
            className="px-10 py-4 bg-white hover:bg-slate-50 text-slate-900 text-base font-black shadow-lg transition-all active:scale-95 flex items-center gap-3 border border-slate-200"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/></svg>
            New Game
          </button>

          <button
            onClick={handleAutoSolve}
            className="px-10 py-4 bg-slate-900 hover:bg-black text-white text-base font-black shadow-lg shadow-blue-600/20 transition-all active:scale-95 flex items-center gap-3 border border-slate-900"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="M12 2v4"/><path d="m16.2 7.8 2.9-2.9"/><path d="M18 12h4"/><path d="m16.2 16.2 2.9 2.9"/><path d="M12 18v4"/><path d="m4.9 19.1 2.9-2.9"/><path d="M2 12h4"/><path d="m4.9 4.9 2.9 2.9"/></svg>
            Auto Solve {autoSolving ? '(Running...)' : ''}
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-auto py-16 px-10 flex items-center justify-center bg-slate-50">
        <div className="relative group">
          <div 
            className="bg-white p-6 shadow-2xl border border-slate-200"
            style={{ display: 'inline-block' }}
          >
            <div
              className="grid gap-0 bg-slate-300 border border-slate-300"
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
                    className={`
                      ${getCellStyles(cell)}
                      ${fontSize}
                      flex items-center justify-center
                      transition-all duration-75
                      select-none
                      border border-slate-800
                      active:scale-90
                      font-[Arial,sans-serif] font-[900]
                    `}
                    style={{
                      width: `${cellSize}px`,
                      height: `${cellSize}px`,
                      WebkitTextStroke: '0',
                      letterSpacing: '-0.02em',
                      lineHeight: 1,
                      fontSize: difficulty === 'Hard' ? '18px' : difficulty === 'Medium' ? '24px' : '30px'
                    }}
                  >
                    {getCellDisplay(cell)}
                  </button>
                ))
              )}
            </div>

          </div>

          {(gameState.gameOver || gameState.gameWon) && (
            <div className="absolute inset-0 z-10 flex items-center justify-center animate-in fade-in zoom-in duration-500">
              <div className="absolute inset-0 bg-white/40 backdrop-blur-md" />
              <div className="relative bg-white p-12 shadow-2xl border border-slate-900 text-center max-w-sm mx-auto overflow-hidden">
                <div className="absolute top-0 left-0 w-full h-2 bg-blue-600" />
                <div className="text-7xl mb-6 scale-125">
                  {gameState.gameWon ? '🏆' : '💥'}
                </div>
                <h2 className={`text-4xl font-black mb-3 tracking-tight ${gameState.gameWon ? 'text-green-600' : 'text-red-600'}`}>
                  {gameState.gameWon ? 'VICTORY' : 'GAME OVER'}
                </h2>
                <p className="text-slate-500 font-bold text-lg mb-10 leading-relaxed uppercase tracking-widest">
                  {gameState.gameWon ? 'Operation Successful' : 'System compromised'}
                </p>
                <button
                  onClick={initGame}
                  className="w-full py-5 bg-slate-900 hover:bg-black text-white font-black text-lg transition-all active:scale-95 shadow-xl border border-slate-900 uppercase tracking-[0.2em]"
                >
                  Initialize New Session
                </button>
              </div>
            </div>
          )}
        </div>
      </div>

      <div className="py-8 bg-white border-t border-slate-100 text-center">
        <div className="text-[10px] font-black text-slate-300 uppercase tracking-[0.3em] flex items-center justify-center gap-12">
          <span className="flex items-center gap-3">
            <span className="w-6 h-6 bg-slate-200 shadow-[inset_-2px_-2px_0px_rgba(0,0,0,0.05)] border border-slate-300" /> Left Click to Reveal
          </span>
          <span className="flex items-center gap-3">
            <span className="w-6 h-6 bg-slate-200 flex items-center justify-center text-[10px] shadow-[inset_-2px_-2px_0px_rgba(0,0,0,0.05)] border border-slate-300">🚩</span> Right Click to Flag
          </span>
        </div>
      </div>
    </div>
  );
}
