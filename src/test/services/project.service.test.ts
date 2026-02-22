import { describe, it, expect, beforeEach, vi } from 'vitest';
import { projectService } from '../../services/api-logged';
import { invoke } from '@tauri-apps/api/core';

vi.mock('@tauri-apps/api/core');

const mockInvoke = invoke as any;

describe('ProjectService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('createProject', () => {
    it('should create a project successfully', async () => {
      const mockProject = {
        id: 'test-id',
        name: 'Test Project',
        description: 'Test Description',
        genre: 'Fantasy',
        template: null,
        status: 'draft',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      };

      mockInvoke.mockResolvedValue(mockProject);

      const request = {
        name: 'Test Project',
        description: 'Test Description',
        genre: 'Fantasy',
      };

      const result = await projectService.createProject(request);

      expect(result).toEqual(mockProject);
      expect(mockInvoke).toHaveBeenCalledWith('create_project', { request });
    });

    it('should handle errors properly', async () => {
      const error = new Error('Failed to create project');
      mockInvoke.mockRejectedValue(error);

      const request = {
        name: 'Test Project',
      };

      await expect(projectService.createProject(request)).rejects.toThrow(error);
    });
  });

  describe('getProjects', () => {
    it('should fetch all projects', async () => {
      const mockProjects = [
        {
          id: '1',
          name: 'Project 1',
          description: 'Description 1',
          genre: 'Fantasy',
          template: null,
          status: 'draft',
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
        {
          id: '2',
          name: 'Project 2',
          description: 'Description 2',
          genre: 'Sci-Fi',
          template: null,
          status: 'published',
          created_at: '2024-01-02T00:00:00Z',
          updated_at: '2024-01-02T00:00:00Z',
        },
      ];

      mockInvoke.mockResolvedValue(mockProjects);

      const result = await projectService.getProjects();

      expect(result).toEqual(mockProjects);
      expect(result).toHaveLength(2);
      expect(mockInvoke).toHaveBeenCalledWith('get_projects');
    });

    it('should return empty array when no projects exist', async () => {
      mockInvoke.mockResolvedValue([]);

      const result = await projectService.getProjects();

      expect(result).toEqual([]);
      expect(result).toHaveLength(0);
    });
  });

  describe('deleteProject', () => {
    it('should delete a project successfully', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await projectService.deleteProject('test-id');

      expect(mockInvoke).toHaveBeenCalledWith('delete_project', { projectId: 'test-id' });
    });

    it('should handle errors properly', async () => {
      const error = new Error('Failed to delete project');
      mockInvoke.mockRejectedValue(error);

      await expect(projectService.deleteProject('test-id')).rejects.toThrow(error);
    });
  });

  describe('updateProject', () => {
    it('should update project name', async () => {
      const mockProject = {
        id: 'test-id',
        name: 'Updated Project Name',
        description: 'Original Description',
        genre: 'Fantasy',
        template: null,
        status: 'draft',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-02T00:00:00Z',
      };

      mockInvoke.mockResolvedValue(mockProject);

      const result = await projectService.updateProject('test-id', 'Updated Project Name');

      expect(result).toEqual(mockProject);
      expect(mockInvoke).toHaveBeenCalledWith('update_project', {
        projectId: 'test-id',
        name: 'Updated Project Name',
        description: undefined,
        genre: undefined,
      });
    });

    it('should update project with all fields', async () => {
      const mockProject = {
        id: 'test-id',
        name: 'Updated Name',
        description: 'Updated Description',
        genre: 'Sci-Fi',
        template: null,
        status: 'draft',
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-02T00:00:00Z',
      };

      mockInvoke.mockResolvedValue(mockProject);

      const result = await projectService.updateProject(
        'test-id',
        'Updated Name',
        'Updated Description',
        'Sci-Fi'
      );

      expect(result).toEqual(mockProject);
      expect(mockInvoke).toHaveBeenCalledWith('update_project', {
        projectId: 'test-id',
        name: 'Updated Name',
        description: 'Updated Description',
        genre: 'Sci-Fi',
      });
    });
  });
});
