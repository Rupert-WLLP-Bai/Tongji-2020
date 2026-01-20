import { useState } from 'react';
import ProjectList from './components/ProjectList';
import HanoiGame from './components/HanoiGame';
import MinesweeperGame from './components/MinesweeperGame';
import { Project } from './types';
import './index.css';

function App() {
  const [selectedProject, setSelectedProject] = useState<Project | null>(null);

  const handleSelectProject = (project: Project) => {
    setSelectedProject(project);
    console.log('Selected project:', project);
  };

  const handleBack = () => {
    setSelectedProject(null);
  };

  return (
    <div className="w-full h-screen bg-gray-900">
      {!selectedProject ? (
        <ProjectList onSelectProject={handleSelectProject} />
      ) : selectedProject.id === 'hanoi' ? (
        <HanoiGame onBack={handleBack} />
      ) : selectedProject.id === 'minesweeper' ? (
        <MinesweeperGame onBack={handleBack} />
      ) : (
        <div className="flex flex-col h-full">
          {/* Header */}
          <div className="p-4 bg-gray-800 border-b border-gray-700 flex items-center gap-4">
            <button
              onClick={handleBack}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
            >
              ← Back
            </button>
            <h1 className="text-2xl font-bold text-white">{selectedProject.name}</h1>
            <span className="text-gray-400">{selectedProject.description}</span>
          </div>

          {/* Content */}
          <div className="flex-1 overflow-auto p-6 text-white">
            <div className="text-center">
              <h2 className="text-3xl font-bold mb-4">{selectedProject.name}</h2>
              <p className="text-gray-400 mb-8">{selectedProject.description}</p>
              <div className="bg-gray-800 rounded-lg p-8 max-w-2xl mx-auto">
                <p className="text-lg text-gray-300">
                  This project has been successfully migrated from C/C++ to Rust.
                </p>
                <p className="text-sm text-gray-500 mt-4">
                  Terminal execution available via: <code className="bg-gray-900 px-2 py-1 rounded">cargo run --bin {selectedProject.id}</code>
                </p>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
