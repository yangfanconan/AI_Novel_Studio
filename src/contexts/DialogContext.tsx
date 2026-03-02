import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';

interface DialogState {
  isCreateProjectDialogOpen: boolean;
  isChapterNameDialogOpen: boolean;
  isProjectRenameDialogOpen: boolean;
  isChapterRenameDialogOpen: boolean;
  isCharacterDialogOpen: boolean;
  isModelSettingsDialogOpen: boolean;
  isExportDialogOpen: boolean;
  isImportDialogOpen: boolean;
  isPluginManagerOpen: boolean;
  isPromptTemplateOpen: boolean;
  isMultimediaSettingsOpen: boolean;
  isOutlineOpen: boolean;
  isBatchGeneratorOpen: boolean;
  isReverseAnalysisOpen: boolean;
  isCharacterBibleOpen: boolean;
  isBatchProductionOpen: boolean;
  isSceneEditorOpen: boolean;
  isComfyUIPanelOpen: boolean;
  isWorkflowEditorOpen: boolean;
  isSeedancePanelOpen: boolean;
  isStoryboardEditorOpen: boolean;
  isChapterVersionPanelOpen: boolean;
  isChapterOptimizerOpen: boolean;
  isBlueprintEditorOpen: boolean;
  isChapterMissionPanelOpen: boolean;
  isVersionComparisonOpen: boolean;
  isTaskProgressOpen: boolean;
  isPlotPointEditorOpen: boolean;
  isWorldViewEditorOpen: boolean;
}

type DialogKey = keyof DialogState;

interface DialogContextType {
  dialogs: DialogState;
  openDialog: (key: DialogKey) => void;
  closeDialog: (key: DialogKey) => void;
  toggleDialog: (key: DialogKey) => void;
  closeAllDialogs: () => void;
}

const initialDialogState: DialogState = {
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

const DialogContext = createContext<DialogContextType | undefined>(undefined);

export function DialogProvider({ children }: { children: ReactNode }) {
  const [dialogs, setDialogs] = useState<DialogState>(initialDialogState);

  const openDialog = useCallback((key: DialogKey) => {
    setDialogs(prev => ({ ...prev, [key]: true }));
  }, []);

  const closeDialog = useCallback((key: DialogKey) => {
    setDialogs(prev => ({ ...prev, [key]: false }));
  }, []);

  const toggleDialog = useCallback((key: DialogKey) => {
    setDialogs(prev => ({ ...prev, [key]: !prev[key] }));
  }, []);

  const closeAllDialogs = useCallback(() => {
    setDialogs(initialDialogState);
  }, []);

  return (
    <DialogContext.Provider value={{ dialogs, openDialog, closeDialog, toggleDialog, closeAllDialogs }}>
      {children}
    </DialogContext.Provider>
  );
}

export function useDialog() {
  const context = useContext(DialogContext);
  if (context === undefined) {
    throw new Error('useDialog must be used within a DialogProvider');
  }
  return context;
}

export type { DialogKey, DialogState };
