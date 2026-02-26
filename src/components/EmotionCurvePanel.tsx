import React, { useState, useEffect } from "react";
import { emotionCurveService } from "../services/api";
import { TrendingUp, Activity, Zap, Clock, AlertCircle } from "lucide-react";

const ARC_TYPES = [
  { value: "standard", label: "标准弧线", description: "经典的起承转合结构" },
  { value: "slow_burn", label: "慢热型", description: "缓慢铺垫，后期高潮" },
  { value: "fast_paced", label: "快节奏", description: "连续小高潮，节奏紧凑" },
  { value: "wave", label: "波浪式", description: "多个波峰波谷" },
];

interface EmotionCurvePanelProps {
  projectId: string;
  chapters: number;
}

export const EmotionCurvePanel: React.FC<EmotionCurvePanelProps> = ({
  projectId,
  chapters,
}) => {
  const [arcType, setArcType] = useState("standard");
  const [curveData, setCurveData] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (projectId) {
      loadCurve();
    }
  }, [projectId, arcType]);

  const loadCurve = async () => {
    setLoading(true);
    try {
      const data = await emotionCurveService.calculateCurve(projectId, arcType, chapters);
      setCurveData(data);
    } catch (error) {
      console.error("加载情感曲线失败:", error);
    } finally {
      setLoading(false);
    }
  };

  const getEmotionColor = (value: number) => {
    if (value >= 85) return "bg-red-500";
    if (value >= 70) return "bg-orange-500";
    if (value >= 50) return "bg-yellow-500";
    if (value >= 30) return "bg-blue-400";
    return "bg-slate-400";
  };

  const getPhaseColor = (phase: string) => {
    const phaseLower = phase.toLowerCase();
    if (phaseLower.includes("高潮")) return "bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400";
    if (phaseLower.includes("铺垫") || phaseLower.includes("开篇")) return "bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400";
    if (phaseLower.includes("上升") || phaseLower.includes("第一波")) return "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400";
    if (phaseLower.includes("发展") || phaseLower.includes("短暂")) return "bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400";
    if (phaseLower.includes("低谷") || phaseLower.includes("回落")) return "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-400";
    if (phaseLower.includes("反转") || phaseLower.includes("转折")) return "bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400";
    return "bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400";
  };

  if (!curveData) {
    return (
      <div className="flex flex-col h-full bg-background dark:bg-gray-900">
        <div className="flex items-center justify-between px-4 py-3 border-b bg-muted/30">
          <div className="flex items-center gap-2">
            <TrendingUp className="w-5 h-5 text-primary" />
            <h2 className="font-semibold">情感曲线</h2>
          </div>
        </div>
        <div className="flex-1 flex items-center justify-center">
          <button
            onClick={loadCurve}
            disabled={loading}
            className="px-6 py-3 bg-primary text-white rounded-lg hover:bg-primary/90 disabled:opacity-50"
          >
            {loading ? "加载中..." : "生成曲线"}
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-background dark:bg-gray-900">
      <div className="flex items-center justify-between px-4 py-3 border-b bg-muted/30">
        <div className="flex items-center gap-2">
          <TrendingUp className="w-5 h-5 text-primary" />
          <h2 className="font-semibold">情感曲线</h2>
        </div>
        <div className="flex items-center gap-2">
          <select
            value={arcType}
            onChange={(e) => setArcType(e.target.value)}
            className="px-3 py-1.5 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-primary/20 dark:bg-gray-700"
          >
            {ARC_TYPES.map((type) => (
              <option key={type.value} value={type.value}>
                {type.label}
              </option>
            ))}
          </select>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-6">
        <div className="grid grid-cols-4 gap-3">
          <div className="p-4 bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-blue-950/30 dark:to-indigo-950/30 rounded-lg border border-blue-200 dark:border-blue-800">
            <div className="flex items-center gap-2 mb-2">
              <Activity className="w-4 h-4 text-blue-600" />
              <span className="text-sm font-medium">平均情感</span>
            </div>
            <div className="text-3xl font-bold text-blue-600">
              {curveData.overall_stats.avg_emotion.toFixed(1)}
            </div>
          </div>

          <div className="p-4 bg-gradient-to-br from-purple-50 to-pink-50 dark:from-purple-950/30 dark:to-pink-950/30 rounded-lg border border-purple-200 dark:border-purple-800">
            <div className="flex items-center gap-2 mb-2">
              <Zap className="w-4 h-4 text-purple-600" />
              <span className="text-sm font-medium">高潮章节</span>
            </div>
            <div className="flex flex-wrap gap-1">
              {curveData.overall_stats.climax_chapters.map((ch: number, i: number) => (
                <span key={i} className="text-xs px-2 py-0.5 bg-purple-600 text-white rounded-full">
                  第{ch}章
                </span>
              ))}
            </div>
          </div>

          <div className="p-4 bg-gradient-to-br from-green-50 to-emerald-50 dark:from-green-950/30 dark:to-emerald-950/30 rounded-lg border border-green-200 dark:border-green-800">
            <div className="flex items-center gap-2 mb-2">
              <Clock className="w-4 h-4 text-green-600" />
              <span className="text-sm font-medium">情感波动</span>
            </div>
            <div className="text-2xl font-bold text-green-600">
              {curveData.overall_stats.emotion_variance.toFixed(2)}
            </div>
          </div>

          <div className="p-4 bg-gradient-to-br from-orange-50 to-amber-50 dark:from-orange-950/30 dark:to-amber-950/30 rounded-lg border border-orange-200 dark:border-orange-800">
            <div className="flex items-center gap-2 mb-2">
              <AlertCircle className="w-4 h-4 text-orange-600" />
              <span className="text-sm font-medium">节奏平衡</span>
            </div>
            <div className="text-2xl font-bold text-orange-600">
              {curveData.overall_stats.pacing_balance.toFixed(2)}
            </div>
          </div>
        </div>

        <div>
          <h3 className="font-semibold mb-3 flex items-center gap-2">
            <TrendingUp className="w-5 h-5" />
            {ARC_TYPES.find((t) => t.value === arcType)?.label}
          </h3>
          <p className="text-sm text-muted-foreground mb-4">
            {ARC_TYPES.find((t) => t.value === arcType)?.description}
          </p>

          <div className="space-y-3">
            {curveData.curve_data.map((item: any, index: number) => (
              <div key={index} className="relative p-4 rounded-lg border-2 hover:border-primary/50 transition-all bg-card dark:bg-gray-800">
                <div className="flex items-start justify-between mb-3">
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-1">
                      <span className="text-xs font-mono px-2 py-0.5 bg-primary/10 text-primary rounded-full">
                        第{item.chapter_number}章
                      </span>
                      <span className={`text-xs px-2 py-0.5 rounded-full ${getPhaseColor(item.phase_name)}`}>
                        {item.phase_name}
                      </span>
                    </div>
                    <h4 className="font-medium text-sm">{item.chapter_title}</h4>
                  </div>
                  <div className="text-right">
                    <div className={`text-2xl font-bold ${getEmotionColor(item.emotion_target)} text-white px-3 py-1 rounded-lg`}>
                      {item.emotion_target.toFixed(0)}
                    </div>
                    <div className="text-xs text-muted-foreground mt-1">
                      {item.pacing}
                    </div>
                  </div>
                </div>

                <div className="flex items-center gap-3 text-xs text-muted-foreground">
                  <div className="flex items-center gap-1">
                    <span>情感区间:</span>
                    <span className="font-medium">
                      {item.emotion_range[0]} - {item.emotion_range[1]}
                    </span>
                  </div>
                  <div className="flex items-center gap-1">
                    <span>刺激密度:</span>
                    <span className="font-medium">{(item.thrill_density * 100).toFixed(0)}%</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <span>对话占比:</span>
                    <span className="font-medium">{(item.dialogue_ratio * 100).toFixed(0)}%</span>
                  </div>
                </div>

                {item.recommendations && item.recommendations.length > 0 && (
                  <div className="mt-3 p-2 bg-amber-50 dark:bg-amber-950/30 border border-amber-200 dark:border-amber-800 rounded-md">
                    <div className="flex items-start gap-2">
                      <AlertCircle className="w-4 h-4 text-amber-600 flex-shrink-0" />
                      <div className="flex-1">
                        <p className="text-xs font-medium text-amber-900 dark:text-amber-200 mb-1">建议</p>
                        {item.recommendations.map((rec: string, i: number) => (
                          <p key={i} className="text-xs text-amber-800 dark:text-amber-300">
                            • {rec}
                          </p>
                        ))}
                      </div>
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default EmotionCurvePanel;
