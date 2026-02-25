import { create } from "zustand";
import type { Project, Chapter, Character } from "../types";

interface ProjectStore {
  // State
  projects: Project[];
  currentProject: Project | null;
  chapters: Chapter[];
  currentChapter: Chapter | null;
  characters: Character[];
  isLoading: boolean;

  // Actions
  setProjects: (projects: Project[]) => void;
  setCurrentProject: (project: Project | null) => void;
  setChapters: (chapters: Chapter[]) => void;
  setCurrentChapter: (chapter: Chapter | null) => void;
  setCharacters: (characters: Character[]) => void;
  setIsLoading: (isLoading: boolean) => void;
  addProject: (project: Project) => void;
  addChapter: (chapter: Chapter) => void;
  updateChapter: (chapterId: string, content: string) => void;
  removeChapter: (chapterId: string) => void;
  addCharacter: (character: Character) => void;
}

export const useProjectStore = create<ProjectStore>((set) => ({
  // Initial state
  projects: [],
  currentProject: null,
  chapters: [],
  currentChapter: null,
  characters: [],
  isLoading: false,

  // Actions
  setProjects: (projects) => set({ projects }),
  setCurrentProject: (project) => set({ currentProject: project }),
  setChapters: (chapters) => set({ chapters }),
  setCurrentChapter: (chapter) => set({ currentChapter: chapter }),
  setCharacters: (characters) => set({ characters }),
  setIsLoading: (isLoading) => set({ isLoading }),
  addProject: (project) => set((state) => ({ projects: [project, ...state.projects] })),
  addChapter: (chapter) => set((state) => ({ chapters: [...state.chapters, chapter] })),
  updateChapter: (chapterId, content) =>
    set((state) => ({
      chapters: state.chapters.map((ch) =>
        ch.id === chapterId ? { ...ch, content, word_count: content.length } : ch
      ),
      currentChapter:
        state.currentChapter?.id === chapterId
          ? { ...state.currentChapter, content, word_count: content.length }
          : state.currentChapter,
    })),
  removeChapter: (chapterId) =>
    set((state) => ({
      chapters: state.chapters.filter((ch) => ch.id !== chapterId),
      currentChapter: state.currentChapter?.id === chapterId ? null : state.currentChapter,
    })),
  addCharacter: (character) => set((state) => ({ characters: [...state.characters, character] })),
}));
