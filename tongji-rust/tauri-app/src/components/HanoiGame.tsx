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
  const [moveHistory, setMoveHistory] = useState<string[]>([]);
  const [currentStep, setCurrentStep] = useState<string>('');
  const [animatingDisk, setAnimatingDisk] = useState<{
    disk: number;
    fromPillar: Pillar;
    toPillar: Pillar;
    progress: number; // 0-1, animation progress
  } | null>(null);
  const [shouldStop, setShouldStop] = useState(false);

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
    setMoveHistory([]);
    setCurrentStep('');
    setAnimatingDisk(null);
    setShouldStop(false);
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

    // Helper function to draw a disk
    const drawDisk = (disk: number, x: number, y: number, isSelected: boolean, isAnimating: boolean) => {
      const diskWidth = (disk / diskCount) * maxDiskWidth;
      const hue = (disk / diskCount) * 360;

      ctx.fillStyle = isSelected || isAnimating ? '#fbbf24' : `hsl(${hue}, 70%, 50%)`;
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
    };

    // Draw static disks (not animating)
    gameState.stacks.forEach((stack, pillarIndex) => {
      const x = pillarWidth * pillarIndex + pillarWidth / 2;

      stack.forEach((disk, stackIndex) => {
        // Skip if this disk is currently animating
        if (animatingDisk && animatingDisk.disk === disk) {
          return;
        }

        const y = baseY - (stackIndex + 1) * diskHeight;
        const isSelected = selectedDisk?.pillar === ['A', 'B', 'C'][pillarIndex] &&
                          selectedDisk?.disk === disk;

        drawDisk(disk, x, y, isSelected, false);
      });
    });

    // Draw animating disk
    if (animatingDisk) {
      const fromIndex = animatingDisk.fromPillar === 'A' ? 0 : animatingDisk.fromPillar === 'B' ? 1 : 2;
      const toIndex = animatingDisk.toPillar === 'A' ? 0 : animatingDisk.toPillar === 'B' ? 1 : 2;

      const fromX = pillarWidth * fromIndex + pillarWidth / 2;
      const toX = pillarWidth * toIndex + pillarWidth / 2;

      // Calculate source and destination heights
      const fromStack = gameState.stacks[fromIndex];
      const toStack = gameState.stacks[toIndex];

      // Source Y (top of source stack before removal)
      const fromY = baseY - (fromStack.length + 1) * diskHeight;
      // Destination Y (top of destination stack after placement)
      const toY = baseY - (toStack.length + 1) * diskHeight;

      // Animation has 3 phases:
      // Phase 1 (0-0.33): Move up from source
      // Phase 2 (0.33-0.67): Move horizontally
      // Phase 3 (0.67-1.0): Move down to destination

      let currentX = fromX;
      let currentY = fromY;
      const topY = 50; // Height to lift disk to

      if (animatingDisk.progress < 0.33) {
        // Phase 1: Moving up
        const phase1Progress = animatingDisk.progress / 0.33;
        currentX = fromX;
        currentY = fromY + (topY - fromY) * phase1Progress;
      } else if (animatingDisk.progress < 0.67) {
        // Phase 2: Moving horizontally
        const phase2Progress = (animatingDisk.progress - 0.33) / 0.34;
        currentX = fromX + (toX - fromX) * phase2Progress;
        currentY = topY;
      } else {
        // Phase 3: Moving down
        const phase3Progress = (animatingDisk.progress - 0.67) / 0.33;
        currentX = toX;
        currentY = topY + (toY - topY) * phase3Progress;
      }

      drawDisk(animatingDisk.disk, currentX, currentY, false, true);
    }

  }, [gameState, selectedDisk, animatingDisk, diskCount]);

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

          const moveCount = gameState.moveCount + 1;
          const stepMsg = `Step ${moveCount}: ${selectedDisk.pillar} → ${pillar}`;

          setGameState({
            ...gameState,
            stacks: newStacks,
            moveCount,
          });
          setSelectedDisk(null);
          setCurrentStep(stepMsg);
          setMoveHistory(prev => [...prev, stepMsg]);
        } else {
          // Invalid move - show error briefly
          alert('Cannot place larger disk on smaller disk!');
        }
      }
    }
  };

  // Animate a single move
  const animateMove = async (from: Pillar, to: Pillar, disk: number) => {
    const animationDuration = 800; // Total animation time in ms
    const fps = 60;
    const frames = (animationDuration / 1000) * fps;

    for (let frame = 0; frame <= frames; frame++) {
      if (shouldStop) {
        setAnimatingDisk(null);
        return false; // Animation was stopped
      }

      const progress = frame / frames;
      setAnimatingDisk({
        disk,
        fromPillar: from,
        toPillar: to,
        progress,
      });
      await new Promise(resolve => setTimeout(resolve, 1000 / fps));
    }

    setAnimatingDisk(null);
    return true; // Animation completed
  };

  // Stop auto-solve
  const stopAutoSolve = () => {
    setShouldStop(true);
  };

  // Auto solve function using recursive algorithm
  const autoSolve = async () => {
    if (!gameState || autoSolving) return;

    setAutoSolving(true);
    setShouldStop(false);
    setMoveHistory([]);
    setCurrentStep('');

    // Reset to initial state
    initGame();

    // Wait a bit for reset to complete
    await new Promise(resolve => setTimeout(resolve, 500));

    if (shouldStop) {
      setAutoSolving(false);
      return;
    }

    // Generate move sequence
    const moves: { from: Pillar; to: Pillar }[] = [];

    const hanoi = (n: number, from: Pillar, to: Pillar, aux: Pillar) => {
      if (n === 1) {
        moves.push({ from, to });
      } else {
        hanoi(n - 1, from, aux, to);
        moves.push({ from, to });
        hanoi(n - 1, aux, to, from);
      }
    };

    // Get auxiliary pillar
    const pillars: Pillar[] = ['A', 'B', 'C'];
    const auxPillar = pillars.find(p => p !== startPillar && p !== targetPillar)!;

    hanoi(diskCount, startPillar, targetPillar, auxPillar);

    // Execute moves with animation
    let currentState = gameState;
    for (let i = 0; i < moves.length; i++) {
      if (shouldStop) {
        break;
      }

      const { from, to } = moves[i];
      const fromIndex = from === 'A' ? 0 : from === 'B' ? 1 : 2;
      const toIndex = to === 'A' ? 0 : to === 'B' ? 1 : 2;

      const disk = currentState.stacks[fromIndex][currentState.stacks[fromIndex].length - 1];
      const moveCount = i + 1;
      const stepMsg = `Step ${moveCount}: ${from} → ${to}`;

      // Update step info before animation
      setCurrentStep(stepMsg);
      setMoveHistory(prev => [...prev, stepMsg]);

      // Animate the move
      const completed = await animateMove(from, to, disk);
      if (!completed) {
        break; // Animation was stopped
      }

      // Update game state after animation
      const newStacks = currentState.stacks.map(s => [...s]);
      newStacks[fromIndex].pop();
      newStacks[toIndex].push(disk);

      currentState = {
        ...currentState,
        stacks: newStacks,
        moveCount,
      };

      setGameState(currentState);

      // Small delay between moves
      await new Promise(resolve => setTimeout(resolve, 200));
    }

    setAutoSolving(false);
    setShouldStop(false);
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

          {autoSolving ? (
            <button
              onClick={stopAutoSolve}
              className="px-4 py-1 bg-red-600 hover:bg-red-700 rounded transition-colors flex items-center gap-2"
            >
              ⏹️ Stop
            </button>
          ) : (
            <button
              onClick={autoSolve}
              className="px-4 py-1 bg-purple-600 hover:bg-purple-700 rounded transition-colors flex items-center gap-2"
            >
              🤖 Auto Solve
            </button>
          )}
        </div>

        {/* Current Step Display */}
        {currentStep && (
          <div className="mt-3 p-2 bg-blue-900/50 border border-blue-700 rounded text-center">
            <span className="text-blue-300 font-mono">{currentStep}</span>
          </div>
        )}
      </div>

      {/* Game Canvas */}
      <div className="flex-1 flex items-stretch p-4 gap-4">
        {/* Canvas Container */}
        <div className="flex-1 flex items-center justify-center">
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
