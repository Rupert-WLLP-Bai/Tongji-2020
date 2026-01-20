import { useState, useMemo } from 'react';
import { Project } from '../types';
import { projects, getTierColor, getTierLabel } from '../lib/projects';

interface ProjectListProps {
  onSelectProject: (project: Project) => void;
}

export default function ProjectList({ onSelectProject }: ProjectListProps) {
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedTier, setSelectedTier] = useState<string>('all');
  const [selectedCategory, setSelectedCategory] = useState<string>('all');

  const filteredProjects = useMemo(() => {
    return projects.filter(project => {
      const matchesSearch = project.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                          project.description.toLowerCase().includes(searchTerm.toLowerCase());
      const matchesTier = selectedTier === 'all' || project.tier === selectedTier;
      const matchesCategory = selectedCategory === 'all' || project.category === selectedCategory;

      return matchesSearch && matchesTier && matchesCategory;
    });
  }, [searchTerm, selectedTier, selectedCategory]);

  const categories = useMemo(() => {
    const cats = new Set(projects.map(p => p.category));
    return ['all', ...Array.from(cats)];
  }, []);

  return (
    <div className="flex flex-col h-full bg-white text-slate-900 font-sans">
      {/* Header */}
      <div className="py-20 px-10 bg-slate-50 border-b border-slate-200">
        <div className="max-w-6xl mx-auto">
          <div className="flex items-baseline gap-4 mb-4">
            <h1 className="text-6xl font-black tracking-tighter text-slate-900">TONGJI RUST</h1>
            <span className="text-blue-600 font-mono text-sm font-bold tracking-widest uppercase">Migration v0.1</span>
          </div>
          <p className="text-slate-500 font-medium mb-12 text-2xl leading-relaxed max-w-3xl border-l-4 border-slate-200 pl-8">
            A comprehensive migration of {projects.length} university projects to modern Rust, rebuilt with precision and performance.
          </p>

          <div className="flex flex-col md:flex-row border border-slate-200 bg-white">
            {/* Search */}
            <div className="relative flex-1 border-b md:border-b-0 md:border-r border-slate-200">
              <svg className="absolute left-6 top-1/2 -translate-y-1/2 text-slate-400" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
              <input
                type="text"
                placeholder="Search projects..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="w-full pl-16 pr-8 py-6 focus:outline-none focus:bg-slate-50 transition-all font-bold text-lg"
              />
            </div>

            {/* Filters */}
            <div className="flex">
              <select
                value={selectedTier}
                onChange={(e) => setSelectedTier(e.target.value)}
                className="px-10 py-6 border-r border-slate-200 focus:outline-none focus:bg-slate-50 transition-all font-black uppercase tracking-widest text-xs appearance-none cursor-pointer min-w-[160px] text-center"
              >
                <option value="all">All Tiers</option>
                <option value="tier1">Tier 1</option>
                <option value="tier2">Tier 2</option>
                <option value="tier3">Tier 3</option>
                <option value="game">Games</option>
              </select>

              <select
                value={selectedCategory}
                onChange={(e) => setSelectedCategory(e.target.value)}
                className="px-10 py-6 focus:outline-none focus:bg-slate-50 transition-all font-black uppercase tracking-widest text-xs appearance-none cursor-pointer min-w-[200px] text-center"
              >
                {categories.map(cat => (
                  <option key={cat} value={cat}>
                    {cat === 'all' ? 'All Categories' : cat}
                  </option>
                ))}
              </select>
            </div>
          </div>
        </div>
      </div>

      {/* Project Grid */}
      <div className="flex-1 overflow-y-auto py-20 px-10">
        <div className="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-px bg-slate-200 border border-slate-200 shadow-2xl shadow-slate-200/50">
          {filteredProjects.map(project => (
            <div
              key={project.id}
              onClick={() => onSelectProject(project)}
              className="group bg-white p-12 cursor-pointer hover:bg-slate-50 transition-all relative overflow-hidden active:bg-white"
            >
              <div className="flex items-start justify-between mb-8">
                <div>
                  <div className={`inline-block px-3 py-1 text-[10px] font-black uppercase tracking-[0.2em] ${getTierColor(project.tier)} text-white mb-4`}>
                    {getTierLabel(project.tier)}
                  </div>
                  <h3 className="text-3xl font-black text-slate-900 group-hover:text-blue-600 transition-colors leading-none tracking-tighter">{project.name}</h3>
                </div>
                <div className="text-slate-200 group-hover:text-blue-500 transition-all -translate-y-2 group-hover:translate-y-0 opacity-0 group-hover:opacity-100">
                  <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>
                </div>
              </div>
              <p className="text-slate-500 font-medium text-lg leading-relaxed mb-10 line-clamp-2">{project.description}</p>
              <div className="flex items-center justify-between mt-auto pt-8 border-t border-slate-100">
                <span className="text-xs font-black uppercase tracking-[0.3em] text-slate-300 group-hover:text-slate-400 transition-colors">{project.category}</span>
                <span className="text-[10px] font-mono text-slate-300 font-bold">RUST_PROJECT_{project.id.toUpperCase()}</span>
              </div>
            </div>
          ))}
        </div>

        {filteredProjects.length === 0 && (
          <div className="text-center py-40 border-2 border-dashed border-slate-200">
            <div className="text-8xl mb-8 grayscale opacity-20">📂</div>
            <p className="text-slate-400 font-black text-3xl tracking-tighter">NO MATCHING ENTRIES</p>
            <button 
              onClick={() => { setSearchTerm(''); setSelectedTier('all'); setSelectedCategory('all'); }}
              className="mt-8 px-8 py-3 bg-slate-900 text-white font-black text-sm uppercase tracking-widest hover:bg-black transition-all"
            >
              Reset Filters
            </button>
          </div>
        )}
      </div>

      <div className="py-12 px-10 border-t border-slate-200 bg-slate-50">
        <div className="max-w-6xl mx-auto flex justify-between items-center">
          <div className="flex gap-20">
            <div>
              <div className="text-5xl font-black text-blue-600 tabular-nums leading-none mb-2 tracking-tighter">{filteredProjects.length}</div>
              <div className="text-[10px] font-black uppercase tracking-[0.3em] text-slate-400">Total Entries</div>
            </div>
            <div>
              <div className="text-5xl font-black text-slate-900 tabular-nums leading-none mb-2 tracking-tighter">836</div>
              <div className="text-[10px] font-black uppercase tracking-[0.3em] text-slate-400">Unit Tests</div>
            </div>
          </div>
          <div className="text-right">
            <div className="text-2xl font-black text-slate-900 tracking-tighter">100%</div>
            <div className="text-[10px] font-black uppercase tracking-[0.3em] text-slate-400">Test Coverage</div>
          </div>
        </div>
      </div>
    </div>
  );
}


