import { useEffect, useState } from "react";
import "./App.css";
import {
  type BoardDocument,
  type ProjectSummary,
  createProject,
  getLastRoot,
  listProjects,
  pickRootFolder,
  readBoard,
} from "./lib/tauri";
import { BoardView } from "./screens/BoardView";
import { ProjectList } from "./screens/ProjectList";
import { Welcome } from "./screens/Welcome";

type Screen = "loading" | "welcome" | "projects" | "board";

function App() {
  const [screen, setScreen] = useState<Screen>("loading");
  const [root, setRoot] = useState<string | null>(null);
  const [projects, setProjects] = useState<ProjectSummary[]>([]);
  const [activeProject, setActiveProject] = useState<ProjectSummary | null>(
    null,
  );
  const [board, setBoard] = useState<BoardDocument | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [pickerBusy, setPickerBusy] = useState(false);

  useEffect(() => {
    getLastRoot()
      .then((lastRoot) => {
        if (lastRoot) {
          openRoot(lastRoot);
        } else {
          setScreen("welcome");
        }
      })
      .catch((err) => {
        setError(String(err));
        setScreen("welcome");
      });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function openRoot(nextRoot: string) {
    try {
      const list = await listProjects(nextRoot);
      setRoot(nextRoot);
      setProjects(list);
      setError(null);
      setScreen("projects");
    } catch (err) {
      setError(String(err));
      setScreen("welcome");
    }
  }

  async function handleChooseFolder() {
    setPickerBusy(true);
    try {
      const picked = await pickRootFolder();
      if (picked) {
        await openRoot(picked);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setPickerBusy(false);
    }
  }

  async function handleCreate(name: string) {
    if (!root) return;
    const project = await createProject(root, name);
    setProjects((current) => [project, ...current]);
  }

  async function handleOpen(project: ProjectSummary) {
    try {
      const doc = await readBoard(project.path);
      setActiveProject(project);
      setBoard(doc);
      setScreen("board");
    } catch (err) {
      setError(String(err));
    }
  }

  function handleBack() {
    setActiveProject(null);
    setBoard(null);
    setScreen("projects");
    if (root) {
      listProjects(root)
        .then(setProjects)
        .catch((err) => setError(String(err)));
    }
  }

  return (
    <>
      {error && (
        <div className="global-error" role="alert">
          {error}
          <button type="button" onClick={() => setError(null)} aria-label="Dismiss">
            ×
          </button>
        </div>
      )}

      {screen === "loading" && <main className="screen screen-center" />}

      {screen === "welcome" && (
        <Welcome onChooseFolder={handleChooseFolder} busy={pickerBusy} />
      )}

      {screen === "projects" && root && (
        <ProjectList
          root={root}
          projects={projects}
          onOpen={handleOpen}
          onCreate={handleCreate}
          onChangeFolder={handleChooseFolder}
        />
      )}

      {screen === "board" && activeProject && board && (
        <BoardView project={activeProject} board={board} onBack={handleBack} />
      )}
    </>
  );
}

export default App;
