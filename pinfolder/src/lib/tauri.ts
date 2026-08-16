import { invoke } from "@tauri-apps/api/core";

export interface ProjectSummary {
  id: string;
  name: string;
  /** Absolute path to the project folder on disk. */
  path: string;
  /** Unix epoch seconds. */
  modifiedAt: number;
}

export interface BoardDocument {
  version: number;
  id: string;
  elements: unknown[];
  connectors: unknown[];
  frames: unknown[];
}

/** Opens the native folder picker. Resolves to `null` if the user cancels. */
export function pickRootFolder(): Promise<string | null> {
  return invoke("pick_root_folder");
}

/** The root folder chosen on a previous launch, if any. */
export function getLastRoot(): Promise<string | null> {
  return invoke("get_last_root");
}

export function listProjects(root: string): Promise<ProjectSummary[]> {
  return invoke("list_projects", { root });
}

export function createProject(
  root: string,
  name: string,
): Promise<ProjectSummary> {
  return invoke("create_project", { root, name });
}

export function readBoard(path: string): Promise<BoardDocument> {
  return invoke("read_board", { path });
}

export function writeBoard(path: string, doc: BoardDocument): Promise<void> {
  return invoke("write_board", { path, doc });
}
