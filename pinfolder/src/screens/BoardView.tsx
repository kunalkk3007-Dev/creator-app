import type { BoardDocument, ProjectSummary } from "../lib/tauri";

interface BoardViewProps {
  project: ProjectSummary;
  board: BoardDocument;
  onBack: () => void;
}

export function BoardView({ project, board, onBack }: BoardViewProps) {
  return (
    <main className="screen board-screen">
      <header className="board-header">
        <button type="button" className="breadcrumb" onClick={onBack}>
          ‹ Projects
        </button>
        <h1>{project.name}</h1>
      </header>

      <div className="canvas-placeholder">
        <div className="canvas-placeholder-note">
          <p>
            <strong>{board.elements.length}</strong> element
            {board.elements.length === 1 ? "" : "s"} on this board.
          </p>
          <p className="canvas-placeholder-sub">
            This board already reads and writes{" "}
            <code>{project.name}/board.json</code> on disk. Notes, images,
            and the rest of the canvas tools land in Phase 1.
          </p>
        </div>
      </div>
    </main>
  );
}
