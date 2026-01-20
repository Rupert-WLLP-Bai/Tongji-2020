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
    <div className="flex flex-col h-full bg-gray-900 text-white">
      {/* Header */}
      <div className="p-6 border-b border-gray-700">
        <h1 className="text-3xl font-bold mb-4">Tongji Rust Projects</h1>
        <p className="text-gray-400 mb-4">
          Browse all {projects.length} migrated projects from C/C++ to Rust
        </p>

        {/* Search */}
        <input
          type="text"
          placeholder="Search projects..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        />

        {/* Filters */}
        <div className="flex gap-4 mt-4">
          <select
            value={selectedTier}
            onChange={(e) => setSelectedTier(e.target.value)}
            className="px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
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
            className="px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {categories.map(cat => (
              <option key={cat} value={cat}>
                {cat === 'all' ? 'All Categories' : cat}
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Project Grid */}
      <div className="flex-1 overflow-y-auto p-6">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {filteredProjects.map(project => (
            <div
              key={project.id}
              onClick={() => onSelectProject(project)}
              className="bg-gray-800 rounded-lg p-4 cursor-pointer hover:bg-gray-700 transition-colors border border-gray-700 hover:border-blue-500"
            >
              <div className="flex items-center justify-between mb-2">
                <h3 className="text-lg font-semibold">{project.name}</h3>
                <span className={`px-2 py-1 text-xs rounded ${getTierColor(project.tier)} text-white`}>
                  {getTierLabel(project.tier)}
                </span>
              </div>
              <p className="text-sm text-gray-400 mb-2">{project.description}</p>
              <span className="text-xs text-gray-500">{project.category}</span>
            </div>
          ))}
        </div>

        {filteredProjects.length === 0 && (
          <div className="text-center text-gray-500 mt-8">
            No projects found matching your criteria
          </div>
        )}
      </div>

      {/* Footer Stats */}
      <div className="p-4 border-t border-gray-700 bg-gray-800">
        <div className="flex justify-around text-sm">
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-400">{filteredProjects.length}</div>
            <div className="text-gray-400">Projects</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-green-400">836</div>
            <div className="text-gray-400">Tests Passing</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-purple-400">100%</div>
            <div className="text-gray-400">Coverage</div>
          </div>
        </div>
      </div>
    </div>
  );
}
