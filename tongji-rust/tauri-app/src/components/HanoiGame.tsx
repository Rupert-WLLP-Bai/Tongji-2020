import { useState, useEffect, useRef } from 'react';
import { HanoiState } from '../types';

interface HanoiGameProps {
  onBack: () => void;
}

type Pillar = 'A' | 'B' | 'C';

export default function HanoiGame({ onBack }: HanoiGameProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [diskCount, setDiskCount] = useState(3);
  const [startPillar, setStartPillar] = useState<Pillar>('A');
  const [targetPillar, setTargetPillar] = useState<Pillar>('C');
  const [gameState, setGameState] = useState<HanoiState | null>(null);
  const [selectedDisk, setSelectedDisk] = useState<{ pillar: Pillar; disk: number } | null>(null);
  const [autoSolving, setAutoSolving] = useState(false);

  // Initialize game
  const initGame = () => {
    const stacks: number[][] = [[], [], []];
    const pillarIndex = startPillar === 'A' ? 0 : startPillar === 'B' ? 1 : 2;

    // Place disks on starting pillar (largest at bottom)
    for (let i = diskCount; i >= 1; i--) {
      stacks[pillarIndex].push(i);
    }

    setGameState({
      stacks,
      moveCount: 0,
      diskCount,
      startPillar,
      targetPillar,
    });
    setSelectedDisk(null);
    setAutoSolving(false);
  };

  // Reset game when parameters change
  useEffect(() => {
    initGame();
  }, [diskCount, startPillar, targetPillar]);

  // Draw the game on canvas
  useEffect(() => {
    if (!canvasRef.current || !gameState) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = '#1f2937';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    const pillarWidth = canvas.width / 3;
    const pillarHeight = canvas.height - 100;
    const baseY = canvas.height - 50;
    const diskHeight = 20;
    const maxDiskWidth = pillarWidth * 0.8;

    // Draw pillars
    ['A', 'B', 'C'].forEach((pillar, index) => {
      const x = pillarWidth * index + pillarWidth / 2;

      // Draw pillar base
      ctx.fillStyle = '#4b5563';
      ctx.fillRect(x - 5, baseY - pillarHeight, 10, pillarHeight);

      // Draw base platform
      ctx.fillStyle = '#6b7280';
      ctx.fillRect(x - pillarWidth / 3, baseY, pillarWidth * 2 / 3, 10);

      // Draw pillar label
      ctx.fillStyle = '#fff';
      ctx.font = 'bold 24px sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(pillar, x, baseY + 40);
    });

    // Draw disks
    gameState.stacks.forEach((stack, pillarIndex) => {
      const x = pillarWidth * pillarIndex + pillarWidth / 2;

      stack.forEach((disk, stackIndex) => {
        const diskWidth = (disk / diskCount) * maxDiskWidth;
        const y = baseY - (stackIndex + 1) * diskHeight;

        // Disk color based on size
        const hue = (disk / diskCount) * 360;
        const isSelected = selectedDisk?.pillar === ['A', 'B', 'C'][pillarIndex] &&
                          selectedDisk?.disk === disk;

        ctx.fillStyle = isSelected ? '#fbbf24' : `hsl(${hue}, 70%, 50%)`;
        ctx.fillRect(x - diskWidth / 2, y, diskWidth, diskHeight - 2);

        // Disk border
        ctx.strokeStyle = '#000';
        ctx.lineWidth = 2;
        ctx.strokeRect(x - diskWidth / 2, y, diskWidth, diskHeight - 2);

        // Disk number
        ctx.fillStyle = '#fff';
        ctx.font = 'bold 14px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText(disk.toString(), x, y + diskHeight / 2 + 5);
      });
    });

  }, [gameState, selectedDisk]);

  // Handle pillar click
  const handlePillarClick = (pillar: Pillar) => {
    if (!gameState || autoSolving) return;

    const pillarIndex = pillar === 'A' ? 0 : pillar === 'B' ? 1 : 2;
    const stack = gameState.stacks[pillarIndex];

    if (!selectedDisk) {
      // Select disk from this pillar
      if (stack.length > 0) {
        const topDisk = stack[stack.length - 1];
        setSelectedDisk({ pillar, disk: topDisk });
      }
    } else {
      // Try to move selected disk to this pillar
      if (selectedDisk.pillar === pillar) {
        // Deselect if clicking same pillar
        setSelectedDisk(null);
      } else {
        // Validate move
        const fromIndex = selectedDisk.pillar === 'A' ? 0 : selectedDisk.pillar === 'B' ? 1 : 2;
        const toIndex = pillarIndex;
        const toStack = gameState.stacks[toIndex];

        if (toStack.length === 0 || toStack[toStack.length - 1] > selectedDisk.disk) {
          // Valid move
          const newStacks = gameState.stacks.map(s => [...s]);
          newStacks[fromIndex].pop();
          newStacks[toIndex].push(selectedDisk.disk);

          setGameState({
            ...gameState,
            stacks: newStacks,
            moveCount: gameState.moveCount + 1,
          });
          setSelectedDisk(null);
        } else {
          // Invalid move - show error briefly
          alert('Cannot place larger disk on smaller disk!');
        }
      }
    }
  };

  // Check win condition
  const isWon = gameState &&
    gameState.stacks[targetPillar === 'A' ? 0 : targetPillar === 'B' ? 1 : 2].length === diskCount;

  // Calculate minimum moves
  const minMoves = Math.pow(2, diskCount) - 1;

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
          <h1 className="text-2xl font-bold">Hanoi Tower</h1>
          <div className="text-right">
            <div className="text-sm text-gray-400">Moves</div>
            <div className="text-2xl font-bold">{gameState?.moveCount || 0}</div>
            <div className="text-xs text-gray-500">Min: {minMoves}</div>
          </div>
        </div>

        {/* Controls */}
        <div className="flex gap-4 items-center flex-wrap">
          <div className="flex items-center gap-2">
            <label className="text-sm">Disks:</label>
            <select
              value={diskCount}
              onChange={(e) => setDiskCount(Number(e.target.value))}
              disabled={autoSolving}
              className="px-3 py-1 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {[1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map(n => (
                <option key={n} value={n}>{n}</option>
              ))}
            </select>
          </div>

          <div className="flex items-center gap-2">
            <label className="text-sm">Start:</label>
            <select
              value={startPillar}
              onChange={(e) => setStartPillar(e.target.value as Pillar)}
              disabled={autoSolving}
              className="px-3 py-1 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="A">A</option>
              <option value="B">B</option>
              <option value="C">C</option>
            </select>
          </div>

          <div className="flex items-center gap-2">
            <label className="text-sm">Target:</label>
            <select
              value={targetPillar}
              onChange={(e) => setTargetPillar(e.target.value as Pillar)}
              disabled={autoSolving}
              className="px-3 py-1 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="A">A</option>
              <option value="B">B</option>
              <option value="C">C</option>
            </select>
          </div>

          <button
            onClick={initGame}
            disabled={autoSolving}
            className="px-4 py-1 bg-green-600 hover:bg-green-700 rounded transition-colors disabled:opacity-50"
          >
            Reset
          </button>
        </div>
      </div>

      {/* Game Canvas */}
      <div className="flex-1 flex items-center justify-center p-4">
        <div className="relative">
          <canvas
            ref={canvasRef}
            width={800}
            height={400}
            className="border border-gray-700 rounded-lg cursor-pointer"
            onClick={(e) => {
              const rect = canvasRef.current?.getBoundingClientRect();
              if (!rect) return;

              const x = e.clientX - rect.left;
              const pillarWidth = 800 / 3;

              if (x < pillarWidth) handlePillarClick('A');
              else if (x < pillarWidth * 2) handlePillarClick('B');
              else handlePillarClick('C');
            }}
          />

          {isWon && (
            <div className="absolute inset-0 flex items-center justify-center bg-black bg-opacity-75 rounded-lg">
              <div className="text-center">
                <h2 className="text-4xl font-bold text-green-400 mb-4">🎉 You Won!</h2>
                <p className="text-xl mb-2">Moves: {gameState?.moveCount}</p>
                <p className="text-sm text-gray-400 mb-4">
                  {gameState?.moveCount === minMoves ? 'Perfect! Minimum moves!' : `Minimum was ${minMoves}`}
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

      {/* Instructions */}
      <div className="p-4 bg-gray-800 border-t border-gray-700">
        <div className="text-sm text-gray-400 text-center">
          {selectedDisk ? (
            <span className="text-yellow-400">
              Disk {selectedDisk.disk} selected from pillar {selectedDisk.pillar}. Click another pillar to move it.
            </span>
          ) : (
            <span>
              Click on a pillar to select the top disk, then click another pillar to move it.
              Goal: Move all disks from {startPillar} to {targetPillar}.
            </span>
          )}
        </div>
      </div>
    </div>
  );
}
