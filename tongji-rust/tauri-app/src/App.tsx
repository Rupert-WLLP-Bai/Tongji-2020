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
    <div className="w-full h-screen bg-white">
      {!selectedProject ? (
        <ProjectList onSelectProject={handleSelectProject} />
      ) : selectedProject.id === 'hanoi' ? (
        <HanoiGame onBack={handleBack} />
      ) : selectedProject.id === 'minesweeper' ? (
        <MinesweeperGame onBack={handleBack} />
      ) : (
        <div className="flex flex-col h-full bg-white relative">
          {/* Back Button - Fixed Position */}
          <button
            onClick={handleBack}
            className="absolute top-6 left-6 z-10 px-6 py-3 bg-slate-900 text-white font-black uppercase tracking-widest text-xs hover:bg-black transition-all active:scale-95 border border-slate-900 shadow-lg"
          >
            ← Back
          </button>

          {/* Header */}
          <div className="p-10 bg-slate-50 border-b border-slate-200">
            <div className="mt-8">
              <h1 className="text-4xl font-black text-slate-900 tracking-tighter uppercase">{selectedProject.name}</h1>
              <span className="text-slate-400 font-mono text-xs font-bold tracking-widest">{selectedProject.description}</span>
            </div>
          </div>

          {/* Content */}
          <div className="flex-1 overflow-auto p-20 bg-white">
            <div className="max-w-4xl">
              {/* Rust Improvements Section */}
              {selectedProject.description.includes('Rust改进:') && (
                <div className="border border-slate-200 p-16 bg-slate-50 relative overflow-hidden mb-12">
                  <div className="absolute top-0 right-0 w-32 h-32 bg-slate-100 -mr-16 -mt-16 rotate-45" />

                  <p className="text-3xl font-black text-slate-900 mb-6 tracking-tight">
                    RUST 改进
                  </p>
                  <div className="text-slate-600 text-lg leading-relaxed mb-8 max-w-2xl">
                    {selectedProject.description.split('| Rust改进: ')[1]?.split('|')[0].split(', ').map((improvement, idx) => (
                      <div key={idx} className="flex items-start gap-3 mb-3">
                        <span className="text-blue-600 font-bold mt-1">✓</span>
                        <span>{improvement.trim()}</span>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Command Section */}
              <div className="border border-slate-200 p-16 bg-slate-50 relative overflow-hidden">
                <div className="absolute top-0 right-0 w-32 h-32 bg-slate-100 -mr-16 -mt-16 rotate-45" />

                <p className="text-3xl font-black text-slate-900 mb-6 tracking-tight">
                  运行命令
                </p>

                <div className="bg-slate-900 p-8 font-mono text-sm text-blue-400 inline-block border-l-8 border-blue-600 shadow-2xl">
                  <span className="text-slate-500 mr-4">$</span> cargo run --bin {selectedProject.id}
                </div>
              </div>

              <div className="mt-20 grid grid-cols-3 gap-12 border-t border-slate-100 pt-12">
                <div>
                  <div className="text-[10px] font-black uppercase tracking-[0.3em] text-slate-300 mb-2">Language</div>
                  <div className="text-lg font-bold text-slate-800">Rust 1.75+</div>
                </div>
                <div>
                  <div className="text-[10px] font-black uppercase tracking-[0.3em] text-slate-300 mb-2">Original</div>
                  <div className="text-lg font-bold text-slate-800">C++ / Tongji</div>
                </div>
                <div>
                  <div className="text-[10px] font-black uppercase tracking-[0.3em] text-slate-300 mb-2">Status</div>
                  <div className="text-lg font-bold text-green-600">Verified</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
