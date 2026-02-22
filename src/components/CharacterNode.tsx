import React from 'react';
import { Handle, Position, NodeProps } from 'reactflow';
import { User } from 'lucide-react';
import type { Character } from '../types';

export function CharacterNode({ data }: NodeProps) {
  const character = data.character as Character | undefined;

  return (
    <div className="bg-white dark:bg-slate-800 rounded-lg shadow-lg border-2 border-slate-200 dark:border-slate-700 min-w-[120px]">
      <Handle
        type="target"
        position={Position.Top}
        className="w-3 h-3 !bg-slate-400"
      />

      <div className="p-3">
        {data.avatar ? (
          <img
            src={data.avatar}
            alt={data.label}
            className="w-12 h-12 rounded-full object-cover mx-auto mb-2 border-2 border-slate-200 dark:border-slate-700"
          />
        ) : (
          <div className="w-12 h-12 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center mx-auto mb-2">
            <User className="w-6 h-6 text-white" />
          </div>
        )}
        <div className="text-center">
          <div className="font-semibold text-sm text-slate-900 dark:text-slate-100 truncate">
            {data.label}
          </div>
          {character && (
            <div className="text-xs text-slate-500 dark:text-slate-400 truncate mt-1">
              {character.gender || ''} {character.age ? `· ${character.age}岁` : ''}
            </div>
          )}
        </div>
      </div>

      <Handle
        type="source"
        position={Position.Bottom}
        className="w-3 h-3 !bg-slate-400"
      />
    </div>
  );
}
