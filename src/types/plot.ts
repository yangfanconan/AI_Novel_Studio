export interface PlotPoint {
  id: string;
  project_id: string;
  parent_id: string | null;
  title: string;
  description: string | null;
  note: string | null;
  chapter_id: string | null;
  status: string;
  sort_order: number;
  level: number;
  created_at: string;
  updated_at: string;
}

export interface CreatePlotPointRequest {
  project_id: string;
  parent_id?: string;
  title: string;
  description?: string;
  note?: string;
  chapter_id?: string;
  sort_order?: number;
}

export interface UpdatePlotPointRequest {
  id: string;
  title?: string;
  description?: string;
  note?: string;
  chapter_id?: string;
  status?: string;
  sort_order?: number;
  parent_id?: string;
}

export interface PlotPointNode extends PlotPoint {
  children: PlotPointNode[];
}
