import { useState } from "react";
import type { ProjectSummary } from "../lib/tauri";

interface ProjectListProps {
  root: string;
  projects: ProjectSummary[];
  onOpen: (project: ProjectSummary) => void;
  onCreate: (name: string) => Promise<void>;
  onChangeFolder: () => void;
}

function formatModified(epochSeconds: number): string {
  if (!epochSeconds) return "just now";
  return new Date(epochSeconds * 1000).toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

export function ProjectList({
  root,
  projects,
  onOpen,
  onCreate,
  onChangeFolder,
}: ProjectListProps) {
  const [creating, setCreating] = useState(false);
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!name.trim()) return;
    setSubmitting(true);
    setError(null);
    try {
      await onCreate(name.trim());
      setName("");
      setCreating(false);
    } catch (err) {
      setError(String(err));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <main className="screen">
      <header className="screen-header">
        <div>
          <h1>Projects</h1>
          <p className="root-path" title={root}>
            {root}
          </p>
        </div>
        <div className="screen-header-actions">
          <button type="button" className="btn btn-ghost" onClick={onChangeFolder}>
            Change folder
          </button>
          <button
            type="button"
            className="btn btn-primary"
            onClick={() => setCreating(true)}
          >
            + New project
          </button>
        </div>
      </header>

      {creating && (
        <form className="create-form" onSubmit={handleSubmit}>
          <input
            autoFocus
            placeholder="Project name"
            value={name}
            onChange={(e) => setName(e.currentTarget.value)}
            disabled={submitting}
          />
          <button type="submit" className="btn btn-primary" disabled={submitting}>
            Create
          </button>
          <button
            type="button"
            className="btn btn-ghost"
            onClick={() => {
              setCreating(false);
              setError(null);
            }}
            disabled={submitting}
          >
            Cancel
          </button>
          {error && <p className="form-error">{error}</p>}
        </form>
      )}

      {projects.length === 0 ? (
        <p className="empty-state">
          No projects yet in this folder. Create one to open its board.
        </p>
      ) : (
        <ul className="project-grid">
          {projects.map((project) => (
            <li key={project.id}>
              <button
                type="button"
                className="project-card"
                onClick={() => onOpen(project)}
              >
                <span className="project-card-name">{project.name}</span>
                <span className="project-card-meta">
                  Edited {formatModified(project.modifiedAt)}
                </span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
