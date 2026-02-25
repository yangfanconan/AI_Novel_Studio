use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storyboard {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: String,
    pub scenes: Vec<StoryboardScene>,
    pub visual_style: VisualStyle,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryboardScene {
    pub id: String,
    pub storyboard_id: String,
    pub scene_number: usize,
    pub location: String,
    pub time_of_day: TimeOfDay,
    pub shots: Vec<StoryboardShot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryboardShot {
    pub id: String,
    pub scene_id: String,
    pub shot_number: usize,
    pub shot_type: ShotType,
    pub camera_angle: CameraAngle,
    pub camera_movement: CameraMovement,
    pub subject: String,
    pub action: String,
    pub dialogue: Option<String>,
    pub duration: f32,
    pub description: String,
    pub visual_reference: Option<String>,
    pub video_reference: Option<String>,
    pub audio_reference: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualStyle {
    Realistic,
    Anime2D,
    Anime3D,
    StopMotion,
    Watercolor,
    OilPainting,
    Sketch,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeOfDay {
    Dawn,
    Morning,
    Noon,
    Afternoon,
    Evening,
    Night,
    Midnight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShotType {
    ExtremeCloseUp,
    CloseUp,
    MediumCloseUp,
    MediumShot,
    MediumFullShot,
    FullShot,
    WideShot,
    ExtremeWideShot,
    TwoShot,
    OverTheShoulder,
    PointOfView,
    Establishing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CameraAngle {
    EyeLevel,
    LowAngle,
    HighAngle,
    DutchAngle,
    BirdEye,
    WormEye,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CameraMovement {
    Static,
    PanLeft,
    PanRight,
    TiltUp,
    TiltDown,
    ZoomIn,
    ZoomOut,
    DollyIn,
    DollyOut,
    TruckLeft,
    TruckRight,
    PedestalUp,
    PedestalDown,
    Arc,
    Crane,
    Handheld,
    Steadicam,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStoryboardRequest {
    pub project_id: String,
    pub name: String,
    pub description: String,
    pub visual_style: VisualStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSceneRequest {
    pub storyboard_id: String,
    pub scene_number: usize,
    pub location: String,
    pub time_of_day: TimeOfDay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShotRequest {
    pub scene_id: String,
    pub shot_number: usize,
    pub shot_type: ShotType,
    pub camera_angle: CameraAngle,
    pub camera_movement: CameraMovement,
    pub subject: String,
    pub action: String,
    pub dialogue: Option<String>,
    pub duration: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateShotRequest {
    pub visual_reference: Option<String>,
    pub video_reference: Option<String>,
    pub audio_reference: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryboardLayout {
    pub storyboard_id: String,
    pub columns: usize,
    pub rows: usize,
    pub shots: Vec<LayoutShot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutShot {
    pub shot_id: String,
    pub row: usize,
    pub col: usize,
    pub span_rows: usize,
    pub span_cols: usize,
}

pub struct StoryboardFormatter;

impl StoryboardFormatter {
    pub fn export_to_pdf(&self, storyboard: &Storyboard) -> String {
        format!("=== {} ===\n{}\n\n", storyboard.name, storyboard.description)
    }

    pub fn export_to_json(&self, storyboard: &Storyboard) -> String {
        serde_json::to_string_pretty(storyboard).unwrap_or_default()
    }

    pub fn export_to_csv(&self, storyboard: &Storyboard) -> String {
        let mut csv = String::from("Scene,Shot,Type,Angle,Movement,Subject,Action,Dialogue,Duration\n");
        
        for scene in &storyboard.scenes {
            for shot in &scene.shots {
                csv.push_str(&format!(
                    "{:?},{:?},{:?},{:?},{},{},{},{}\n",
                    shot.shot_number,
                    shot.shot_type,
                    shot.camera_angle,
                    shot.camera_movement,
                    shot.subject,
                    shot.action,
                    shot.dialogue.as_deref().unwrap_or(""),
                    shot.duration
                ));
            }
        }
        csv
    }

    pub fn create_nx_n_layout(&self, storyboard: &Storyboard, n: usize) -> StoryboardLayout {
        let mut shots = Vec::new();
        let mut current_row = 0;
        let mut current_col = 0;

        for scene in &storyboard.scenes {
            for shot in &scene.shots {
                if current_col >= n {
                    current_col = 0;
                    current_row += 1;
                }
                shots.push(LayoutShot {
                    shot_id: shot.id.clone(),
                    row: current_row,
                    col: current_col,
                    span_rows: 1,
                    span_cols: 1,
                });
                current_col += 1;
            }
        }

        StoryboardLayout {
            storyboard_id: storyboard.id.clone(),
            columns: n,
            rows: (shots.len() + n - 1) / n,
            shots,
        }
    }

    pub fn calculate_total_duration(&self, storyboard: &Storyboard) -> f32 {
        storyboard
            .scenes
            .iter()
            .flat_map(|s| &s.shots)
            .map(|s| s.duration)
            .sum::<f32>()
    }

    pub fn get_shot_count(&self, storyboard: &Storyboard) -> usize {
        storyboard
            .scenes
            .iter()
            .map(|s| s.shots.len())
            .sum::<usize>()
    }

    pub fn get_scenes_with_references<'a>(&self, storyboard: &'a Storyboard) -> Vec<&'a StoryboardShot> {
        storyboard
            .scenes
            .iter()
            .flat_map(|s| s.shots.iter())
            .filter(|s| {
                s.visual_reference.is_some()
                    || s.video_reference.is_some()
                    || s.audio_reference.is_some()
            })
            .collect()
    }
}

impl Storyboard {
    pub fn create(request: CreateStoryboardRequest) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        Self {
            id,
            project_id: request.project_id,
            name: request.name,
            description: request.description,
            scenes: Vec::new(),
            visual_style: request.visual_style,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn add_scene(&mut self, scene: StoryboardScene) {
        self.scenes.push(scene);
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn remove_scene(&mut self, scene_id: &str) -> bool {
        let index = self.scenes.iter().position(|s| s.id == scene_id);
        if let Some(idx) = index {
            self.scenes.remove(idx);
            self.updated_at = chrono::Utc::now().to_rfc3339();
            true
        } else {
            false
        }
    }

    pub fn update_scene(&mut self, scene_id: &str, scene: StoryboardScene) -> bool {
        let index = self.scenes.iter().position(|s| s.id == scene_id);
        if let Some(idx) = index {
            self.scenes[idx] = scene;
            self.updated_at = chrono::Utc::now().to_rfc3339();
            true
        } else {
            false
        }
    }

    pub fn get_scene(&self, scene_id: &str) -> Option<&StoryboardScene> {
        self.scenes.iter().find(|s| s.id == scene_id)
    }

    pub fn get_shot(&self, shot_id: &str) -> Option<&StoryboardShot> {
        self.scenes
            .iter()
            .flat_map(|s| &s.shots)
            .find(|s| s.id == shot_id)
    }

    pub fn update_shot(&mut self, shot_id: &str, updates: UpdateShotRequest) -> bool {
        for scene in &mut self.scenes {
            for shot in &mut scene.shots {
                if shot.id == shot_id {
                    if let Some(vis_ref) = updates.visual_reference {
                        shot.visual_reference = Some(vis_ref.clone());
                    }
                    if let Some(vid_ref) = updates.video_reference {
                        shot.video_reference = Some(vid_ref.clone());
                    }
                    if let Some(aud_ref) = updates.audio_reference {
                        shot.audio_reference = Some(aud_ref.clone());
                    }
                    if let Some(note) = updates.notes {
                        shot.notes = Some(note.clone());
                    }
                    self.updated_at = chrono::Utc::now().to_rfc3339();
                    return true;
                }
            }
        }
        false
    }

    pub fn get_camera_stats(&self) -> CameraStats {
        let mut shot_types: HashMap<String, usize> = HashMap::new();
        let mut angles: HashMap<String, usize> = HashMap::new();
        let mut movements: HashMap<String, usize> = HashMap::new();

        for scene in &self.scenes {
            for shot in &scene.shots {
                *shot_types.entry(format!("{:?}", shot.shot_type)).or_insert(0) += 1;
                *angles.entry(format!("{:?}", shot.camera_angle)).or_insert(0) += 1;
                *movements.entry(format!("{:?}", shot.camera_movement)).or_insert(0) += 1;
            }
        }

        CameraStats {
            shot_types,
            angles,
            movements,
            total_shots: self.scenes.iter().map(|s| s.shots.len()).sum::<usize>(),
        }
    }

    pub fn switch_style(&mut self, new_style: VisualStyle) {
        self.visual_style = new_style;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraStats {
    pub shot_types: HashMap<String, usize>,
    pub angles: HashMap<String, usize>,
    pub movements: HashMap<String, usize>,
    pub total_shots: usize,
}

impl Default for Storyboard {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: String::new(),
            name: String::new(),
            description: String::new(),
            scenes: Vec::new(),
            visual_style: VisualStyle::Realistic,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl Default for StoryboardShot {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            scene_id: String::new(),
            shot_number: 0,
            shot_type: ShotType::MediumShot,
            camera_angle: CameraAngle::EyeLevel,
            camera_movement: CameraMovement::Static,
            subject: String::new(),
            action: String::new(),
            dialogue: None,
            duration: 3.0,
            description: String::new(),
            visual_reference: None,
            video_reference: None,
            audio_reference: None,
            notes: None,
        }
    }
}

impl Default for StoryboardScene {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            storyboard_id: String::new(),
            scene_number: 0,
            location: String::new(),
            time_of_day: TimeOfDay::Morning,
            shots: Vec::new(),
        }
    }
}
