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

    ctx.fillStyle = '#f8fafc';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    const pillarWidth = canvas.width / 3;
    const pillarHeight = canvas.height - 100;
    const baseY = canvas.height - 50;
    const diskHeight = 24;
    const maxDiskWidth = pillarWidth * 0.8;

    // Draw pillars
    ['A', 'B', 'C'].forEach((pillar, index) => {
      const x = pillarWidth * index + pillarWidth / 2;

      // Draw pillar base
      ctx.fillStyle = '#e2e8f0';
      ctx.fillRect(x - 4, baseY - pillarHeight, 8, pillarHeight);

      // Draw base platform
      ctx.fillStyle = '#94a3b8';
      ctx.fillRect(x - pillarWidth / 3, baseY, pillarWidth * 2 / 3, 8);

      // Draw pillar label
      ctx.fillStyle = '#475569';
      ctx.font = 'black 20px sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText(pillar, x, baseY + 40);
    });

    // Helper function to draw a disk
    const drawDisk = (disk: number, x: number, y: number, isSelected: boolean, isAnimating: boolean) => {
      const diskWidth = (disk / diskCount) * maxDiskWidth;
      
      const diskColors = [
        '#3b82f6',
        '#10b981',
        '#f43f5e',
        '#8b5cf6',
        '#f59e0b',
        '#06b6d4',
        '#ec4899',
        '#6366f1',
        '#14b8a6',
        '#f97316',
      ];
      
      const color = diskColors[(disk - 1) % diskColors.length];

      ctx.fillStyle = isSelected ? '#000' : color;
      ctx.fillRect(x - diskWidth / 2, y, diskWidth, diskHeight - 4);

      if (isSelected) {
        ctx.strokeStyle = '#000';
        ctx.lineWidth = 2;
        ctx.strokeRect(x - diskWidth / 2 - 2, y - 2, diskWidth + 4, diskHeight);
      }

      // Disk number
      ctx.fillStyle = '#fff';
      ctx.font = 'bold 12px monospace';
      ctx.textAlign = 'center';
      ctx.fillText(disk.toString(), x, y + diskHeight / 2 + 2);
    };

    // Draw static disks (not animating)
    gameState.stacks.forEach((stack, pillarIndex) => {
      const x = pillarWidth * pillarIndex + pillarWidth / 2;

      stack.forEach((disk, stackIndex) => {
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

      const fromStack = gameState.stacks[fromIndex];
      const toStack = gameState.stacks[toIndex];

      const fromY = baseY - (fromStack.length + 1) * diskHeight;
      const toY = baseY - (toStack.length + 1) * diskHeight;

      let currentX = fromX;
      let currentY = fromY;
      const topY = 50;

      if (animatingDisk.progress < 0.33) {
        const phase1Progress = animatingDisk.progress / 0.33;
        currentX = fromX;
        currentY = fromY + (topY - fromY) * phase1Progress;
      } else if (animatingDisk.progress < 0.67) {
        const phase2Progress = (animatingDisk.progress - 0.33) / 0.34;
        currentX = fromX + (toX - fromX) * phase2Progress;
        currentY = topY;
      } else {
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
      if (stack.length > 0) {
        const topDisk = stack[stack.length - 1];
        setSelectedDisk({ pillar, disk: topDisk });
      }
    } else {
      if (selectedDisk.pillar === pillar) {
        setSelectedDisk(null);
      } else {
        const fromIndex = selectedDisk.pillar === 'A' ? 0 : selectedDisk.pillar === 'B' ? 1 : 2;
        const toIndex = pillarIndex;
        const toStack = gameState.stacks[toIndex];

        if (toStack.length === 0 || toStack[toStack.length - 1] > selectedDisk.disk) {
          const newStacks = gameState.stacks.map(s => [...s]);
          newStacks[fromIndex].pop();
          newStacks[toIndex].push(selectedDisk.disk);

          const moveCount = gameState.moveCount + 1;
          const stepMsg = `MOVE: ${selectedDisk.pillar} -> ${pillar}`;

          setGameState({
            ...gameState,
            stacks: newStacks,
            moveCount,
          });
          setSelectedDisk(null);
          setCurrentStep(stepMsg);
          setMoveHistory(prev => [stepMsg, ...prev]);
        }
      }
    }
  };

  // Animate a single move
  const animateMove = async (from: Pillar, to: Pillar, disk: number) => {
    const animationDuration = 500;
    const fps = 60;
    const frames = (animationDuration / 1000) * fps;

    for (let frame = 0; frame <= frames; frame++) {
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
  };

  const autoSolve = async () => {
    if (!gameState || autoSolving) return;

    setAutoSolving(true);
    setMoveHistory([]);
    setCurrentStep('');

    const stacks: number[][] = [[], [], []];
    const pillarIndex = startPillar === 'A' ? 0 : startPillar === 'B' ? 1 : 2;

    for (let i = diskCount; i >= 1; i--) {
      stacks[pillarIndex].push(i);
    }

    const initialState = {
      stacks,
      moveCount: 0,
      diskCount,
      startPillar,
      targetPillar,
    };

    setGameState(initialState);
    setSelectedDisk(null);
    setAnimatingDisk(null);

    await new Promise(resolve => setTimeout(resolve, 500));

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

    const pillars: Pillar[] = ['A', 'B', 'C'];
    const auxPillar = pillars.find(p => p !== startPillar && p !== targetPillar)!;

    hanoi(diskCount, startPillar, targetPillar, auxPillar);

    let currentState = initialState;
    for (let i = 0; i < moves.length; i++) {
      const { from, to } = moves[i];
      const fromIndex = from === 'A' ? 0 : from === 'B' ? 1 : 2;
      const toIndex = to === 'A' ? 0 : to === 'B' ? 1 : 2;

      const disk = currentState.stacks[fromIndex][currentState.stacks[fromIndex].length - 1];
      const moveCount = i + 1;
      const stepMsg = `AUTO: ${from} -> ${to}`;

      setCurrentStep(stepMsg);
      setMoveHistory(prev => [stepMsg, ...prev]);

      await animateMove(from, to, disk);

      const newStacks = currentState.stacks.map(s => [...s]);
      newStacks[fromIndex].pop();
      newStacks[toIndex].push(disk);

      currentState = {
        ...currentState,
        stacks: newStacks,
        moveCount,
      };

      setGameState(currentState);
      await new Promise(resolve => setTimeout(resolve, 100));
    }

    setAutoSolving(false);
  };

  // Check win condition
  const isWon = gameState &&
    gameState.stacks[targetPillar === 'A' ? 0 : targetPillar === 'B' ? 1 : 2].length === diskCount;

  const minMoves = Math.pow(2, diskCount) - 1;

  return (
    <div className="flex flex-col h-full bg-white text-slate-900 font-sans">
      {/* Header */}
      <div className="py-8 px-10 bg-slate-50 border-b border-slate-200">
        <div className="max-w-5xl mx-auto flex items-center justify-between">
          <div className="flex items-center gap-6">
            <button
              onClick={onBack}
              className="p-3 hover:bg-white rounded-none transition-all text-slate-400 hover:text-blue-600 border border-transparent hover:border-slate-200"
              title="Back"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="m15 18-6-6 6-6"/></svg>
            </button>
            <div>
              <h1 className="text-3xl font-black tracking-tight text-slate-900">HANOI TOWER</h1>
              <p className="text-slate-400 text-sm font-bold uppercase tracking-widest">Mathematical Puzzle</p>
            </div>
          </div>
          
          <div className="flex items-center gap-10">
            <div className="text-center">
              <div className="text-[10px] uppercase tracking-[0.3em] font-black text-slate-400 mb-1">Moves</div>
              <div className="text-3xl font-black text-slate-900 tabular-nums leading-none">
                {String(gameState?.moveCount || 0).padStart(2, '0')}
              </div>
            </div>
            <div className="text-center">
              <div className="text-[10px] uppercase tracking-[0.3em] font-black text-slate-400 mb-1">Target</div>
              <div className="text-3xl font-black text-blue-600 tabular-nums leading-none">
                {String(minMoves).padStart(2, '0')}
              </div>
            </div>
          </div>
        </div>

        {/* Controls */}
        <div className="mt-10 flex gap-0 items-center justify-center max-w-5xl mx-auto">
          <div className="flex border border-slate-200">
            <div className="px-6 py-4 bg-white border-r border-slate-200 flex items-center gap-3">
              <span className="text-[10px] font-black uppercase tracking-widest text-slate-400">Disks</span>
              <select
                value={diskCount}
                onChange={(e) => setDiskCount(Number(e.target.value))}
                disabled={autoSolving}
                className="bg-transparent font-bold focus:outline-none cursor-pointer"
              >
                {[3, 4, 5, 6, 7, 8].map(n => (
                  <option key={n} value={n}>{n}</option>
                ))}
              </select>
            </div>
            
            <button
              onClick={initGame}
              disabled={autoSolving}
              className="px-8 py-4 bg-white hover:bg-slate-50 font-black uppercase tracking-widest text-xs border-r border-slate-200 transition-all disabled:opacity-30"
            >
              Reset
            </button>

            <button
              onClick={autoSolve}
              disabled={autoSolving || isWon}
              className="px-8 py-4 bg-slate-900 text-white hover:bg-black font-black uppercase tracking-widest text-xs transition-all disabled:opacity-30 flex items-center gap-3"
            >
              {autoSolving ? 'Solving...' : 'Auto Solve'}
            </button>
          </div>
        </div>
      </div>

      <div className="flex-1 flex flex-col md:flex-row items-stretch p-10 gap-10 overflow-hidden bg-white">
        <div className="flex-1 flex items-center justify-center">
          <div className="relative border border-slate-200 p-8 bg-slate-50 shadow-2xl shadow-slate-100">
            <canvas
              ref={canvasRef}
              width={700}
              height={400}
              className="cursor-pointer"
              onClick={(e) => {
                const rect = canvasRef.current?.getBoundingClientRect();
                if (!rect) return;
                const x = e.clientX - rect.left;
                const pillarWidth = 700 / 3;
                if (x < pillarWidth) handlePillarClick('A');
                else if (x < pillarWidth * 2) handlePillarClick('B');
                else handlePillarClick('C');
              }}
            />

            {isWon && !autoSolving && (
              <div className="absolute inset-0 flex items-center justify-center bg-white/80 backdrop-blur-sm animate-in fade-in duration-500">
                <div className="bg-white p-12 border border-slate-900 shadow-2xl text-center max-w-sm">
                  <div className="text-6xl mb-6">🏆</div>
                  <h2 className="text-4xl font-black text-slate-900 mb-2 tracking-tighter uppercase">Puzzle Solved</h2>
                  <p className="text-slate-500 font-bold mb-8 uppercase tracking-widest text-sm">
                    Completed in {gameState?.moveCount} moves
                  </p>
                  <button
                    onClick={initGame}
                    className="w-full py-4 bg-slate-900 text-white font-black uppercase tracking-widest text-sm hover:bg-black transition-all"
                  >
                    Start Over
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Move History */}
        <div className="w-80 border border-slate-200 bg-slate-50 flex flex-col">
          <div className="p-6 border-b border-slate-200 bg-white">
            <h3 className="text-xs font-black uppercase tracking-[0.3em] text-slate-400">Activity Log</h3>
          </div>
          <div className="flex-1 overflow-y-auto p-4 space-y-2 font-mono text-[10px] font-bold">
            {moveHistory.length === 0 && (
              <div className="text-slate-300 py-4 text-center italic">No activity recorded</div>
            )}
            {moveHistory.map((move, index) => (
              <div
                key={index}
                className={`p-3 border ${
                  index === 0 ? 'bg-blue-600 border-blue-600 text-white' : 'bg-white border-slate-200 text-slate-400'
                }`}
              >
                {move}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

