import { Project } from '../types';

// All projects from the Rust migration
export const projects: Project[] = [
  // Tier 1 Projects (3-b*, 4-b*)
  { id: '3-b2', name: '3-b2', tier: 'tier1', description: 'Geometric calculations (circle, sphere, cylinder)', category: 'Mathematics' },
  { id: '3-b3', name: '3-b3', tier: 'tier1', description: 'Integer digit extraction (1-30000)', category: 'Fundamentals' },
  { id: '3-b4', name: '3-b4', tier: 'tier1', description: 'Large number digit extraction (0-10 billion)', category: 'Mathematics' },
  { id: '3-b5', name: '3-b5', tier: 'tier1', description: 'Triangle area calculation', category: 'Mathematics' },
  { id: '3-b6', name: '3-b6', tier: 'tier1', description: 'Basic I/O and conditionals', category: 'Fundamentals' },
  { id: '3-b7', name: '3-b7', tier: 'tier1', description: 'Control flow', category: 'Fundamentals' },
  { id: '3-b8', name: '3-b8', tier: 'tier1', description: 'Loops', category: 'Fundamentals' },
  { id: '3-b9', name: '3-b9', tier: 'tier1', description: 'Input validation', category: 'Fundamentals' },
  { id: '3-b10', name: '3-b10', tier: 'tier1', description: 'Basic calculations', category: 'Fundamentals' },
  { id: '3-b11', name: '3-b11', tier: 'tier1', description: 'String operations', category: 'Fundamentals' },
  { id: '3-b12', name: '3-b12', tier: 'tier1', description: 'Number processing', category: 'Fundamentals' },
  { id: '3-b13', name: '3-b13', tier: 'tier1', description: 'Pattern matching', category: 'Fundamentals' },

  // Tier 2 Projects (5-b*, 6-b*)
  { id: '5-b1', name: '5-b1', tier: 'tier2', description: 'Array operations', category: 'Data Structures' },
  { id: '5-b2', name: '5-b2', tier: 'tier2', description: 'Sorting algorithms', category: 'Algorithms' },
  { id: '6-b1', name: '6-b1', tier: 'tier2', description: 'Advanced algorithms', category: 'Algorithms' },
  { id: '6-b2', name: '6-b2', tier: 'tier2', description: 'Data processing', category: 'Data Structures' },

  // Tier 3 Projects (7-b*, 8-b*, 9-b*)
  { id: '7-b1', name: '7-b1', tier: 'tier3', description: 'Date/time calculations', category: 'Advanced' },
  { id: '7-b2', name: '7-b2', tier: 'tier3', description: 'Time handling', category: 'Advanced' },
  { id: '8-b1', name: '8-b1', tier: 'tier3', description: 'File I/O', category: 'File Operations' },
  { id: '8-b2', name: '8-b2', tier: 'tier3', description: 'File processing', category: 'File Operations' },
  { id: '9-b1', name: '9-b1', tier: 'tier3', description: 'Complex algorithms', category: 'Advanced' },

  // Games
  { id: 'hanoi', name: 'Hanoi Tower', tier: 'game', description: 'Classic Tower of Hanoi puzzle game', category: 'Games' },
  { id: 'minesweeper', name: 'Minesweeper', tier: 'game', description: 'Classic Minesweeper game', category: 'Games' },
];

export const getTierColor = (tier: Project['tier']): string => {
  switch (tier) {
    case 'tier1': return 'bg-blue-500';
    case 'tier2': return 'bg-green-500';
    case 'tier3': return 'bg-purple-500';
    case 'game': return 'bg-red-500';
    default: return 'bg-gray-500';
  }
};

export const getTierLabel = (tier: Project['tier']): string => {
  switch (tier) {
    case 'tier1': return 'Tier 1';
    case 'tier2': return 'Tier 2';
    case 'tier3': return 'Tier 3';
    case 'game': return 'Game';
    default: return 'Unknown';
  }
};
