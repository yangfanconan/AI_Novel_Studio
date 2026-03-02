import { create } from 'zustand';

type DialogKey = 
  | 'isCreateProjectDialogOpen'
  | 'isChapterNameDialogOpen'
  | 'isProjectRenameDialogOpen'
  | 'isChapterRenameDialogOpen'
  | 'isCharacterDialogOpen'
  | 'isModelSettingsDialogOpen'
  | 'isExportDialogOpen'
  | 'isImportDialogOpen'
  | 'isPluginManagerOpen'
  | 'isPromptTemplateOpen'
  | 'isMultimediaSettingsOpen'
  | 'isOutlineOpen'
  | 'isBatchGeneratorOpen'
  | 'isReverseAnalysisOpen'
  | 'isCharacterBibleOpen'
  | 'isBatchProductionOpen'
  | 'isSceneEditorOpen'
  | 'isComfyUIPanelOpen'
  | 'isWorkflowEditorOpen'
  | 'isSeedancePanelOpen'
  | 'isStoryboardEditorOpen'
  | 'isChapterVersionPanelOpen'
  | 'isChapterOptimizerOpen'
  | 'isBlueprintEditorOpen'
  | 'isChapterMissionPanelOpen'
  | 'isVersionComparisonOpen'
  | 'isTaskProgressOpen'
  | 'isPlotPointEditorOpen'
  | 'isWorldViewEditorOpen';

interface DialogState {
  dialogs: Record<DialogKey, boolean>;
  openDialog: (key: DialogKey) => void;
  closeDialog: (key: DialogKey) => void;
  toggleDialog: (key: DialogKey) => void;
  closeAllDialogs: () => void;
}

const initialDialogs: Record<DialogKey, boolean> = {
  isCreateProjectDialogOpen: false,
  isChapterNameDialogOpen: false,
  isProjectRenameDialogOpen: false,
  isChapterRenameDialogOpen: false,
  isCharacterDialogOpen: false,
  isModelSettingsDialogOpen: false,
  isExportDialogOpen: false,
  isImportDialogOpen: false,
  isPluginManagerOpen: false,
  isPromptTemplateOpen: false,
  isMultimediaSettingsOpen: false,
  isOutlineOpen: false,
  isBatchGeneratorOpen: false,
  isReverseAnalysisOpen: false,
  isCharacterBibleOpen: false,
  isBatchProductionOpen: false,
  isSceneEditorOpen: false,
  isComfyUIPanelOpen: false,
  isWorkflowEditorOpen: false,
  isSeedancePanelOpen: false,
  isStoryboardEditorOpen: false,
  isChapterVersionPanelOpen: false,
  isChapterOptimizerOpen: false,
  isBlueprintEditorOpen: false,
  isChapterMissionPanelOpen: false,
  isVersionComparisonOpen: false,
  isTaskProgressOpen: false,
  isPlotPointEditorOpen: false,
  isWorldViewEditorOpen: false,
};

export const useDialogStore = create<DialogState>((set) => ({
  dialogs: initialDialogs,
  openDialog: (key) => set((state) => ({ dialogs: { ...state.dialogs, [key]: true } })),
  closeDialog: (key) => set((state) => ({ dialogs: { ...state.dialogs, [key]: false } })),
  toggleDialog: (key) => set((state) => ({ dialogs: { ...state.dialogs, [key]: !state.dialogs[key] } })),
  closeAllDialogs: () => set({ dialogs: initialDialogs }),
}));

export type { DialogKey };
