// Project types
export interface Project {
  id: string;
  name: string;
  tier: 'tier1' | 'tier2' | 'tier3' | 'game';
  description: string;
  category: string;
}

// Hanoi Tower types
export interface HanoiState {
  stacks: number[][];
  moveCount: number;
  diskCount: number;
  startPillar: 'A' | 'B' | 'C';
  targetPillar: 'A' | 'B' | 'C';
}

// Minesweeper types
export type Difficulty = 'Easy' | 'Medium' | 'Hard';

export interface MinesweeperCell {
  revealed: boolean;
  flagged: boolean;
  hasMine: boolean;
  adjacentMines: number;
}

export interface MinesweeperState {
  board: MinesweeperCell[][];
  width: number;
  height: number;
  mineCount: number;
  flagCount: number;
  gameOver: boolean;
  won: boolean;
}
