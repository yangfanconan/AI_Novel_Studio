import { describe, it, expect, beforeEach, vi } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ProjectList } from "@/components/ProjectList";
import type { Project } from "@/types";

describe("ProjectList Component", () => {
  const mockProjects: Project[] = [
    {
      id: "1",
      name: "Test Project 1",
      description: "First test project",
      genre: "fantasy",
      template: null,
      status: "active",
      created_at: "2024-01-01T00:00:00Z",
      updated_at: "2024-01-01T00:00:00Z",
    },
    {
      id: "2",
      name: "Test Project 2",
      description: "Second test project",
      genre: "scifi",
      template: null,
      status: "active",
      created_at: "2024-01-02T00:00:00Z",
      updated_at: "2024-01-02T00:00:00Z",
    },
  ];

  const mockOnSelectProject = vi.fn();
  const mockOnCreateProject = vi.fn();
  const mockOnDeleteProject = vi.fn();
  const mockOnRenameProject = vi.fn();
  const mockOnOpenSettings = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should render project list correctly", async () => {
    render(
      <ProjectList
        projects={mockProjects}
        currentProject={null}
        onSelectProject={mockOnSelectProject}
        onCreateProject={mockOnCreateProject}
        onDeleteProject={mockOnDeleteProject}
        onRenameProject={mockOnRenameProject}
      />
    );

    await waitFor(() => {
      expect(screen.getByText("Test Project 1")).toBeInTheDocument();
      expect(screen.getByText("Test Project 2")).toBeInTheDocument();
    });

    expect(screen.getByText("First test project")).toBeInTheDocument();
    expect(screen.getByText("Second test project")).toBeInTheDocument();
  });

  it("should display empty state when no projects", () => {
    render(
      <ProjectList
        projects={[]}
        currentProject={null}
        onSelectProject={mockOnSelectProject}
        onCreateProject={mockOnCreateProject}
        onDeleteProject={mockOnDeleteProject}
        onRenameProject={mockOnRenameProject}
      />
    );

    expect(screen.getByText("暂无项目")).toBeInTheDocument();
    expect(screen.getByText('点击"新建项目"开始创作')).toBeInTheDocument();
  });

  it("should highlight current project", async () => {
    render(
      <ProjectList
        projects={mockProjects}
        currentProject={mockProjects[0]}
        onSelectProject={mockOnSelectProject}
        onCreateProject={mockOnCreateProject}
        onDeleteProject={mockOnDeleteProject}
        onRenameProject={mockOnRenameProject}
      />
    );

    await waitFor(() => {
      const project1 = screen.getByText("Test Project 1").closest("button");
      const project2 = screen.getByText("Test Project 2").closest("button");

      expect(project1).toHaveClass("bg-primary", "text-primary-foreground");
      expect(project2).not.toHaveClass("bg-primary", "text-primary-foreground");
    });
  });

  it("should call onSelectProject when clicking a project", async () => {
    const user = userEvent.setup();

    render(
      <ProjectList
        projects={mockProjects}
        currentProject={null}
        onSelectProject={mockOnSelectProject}
        onCreateProject={mockOnCreateProject}
        onDeleteProject={mockOnDeleteProject}
        onRenameProject={mockOnRenameProject}
      />
    );

    await waitFor(() => {
      screen.getByText("Test Project 1");
    });

    await user.click(screen.getByText("Test Project 1"));
    expect(mockOnSelectProject).toHaveBeenCalledWith(mockProjects[0]);
  });

  it("should call onCreateProject when clicking create button", async () => {
    const user = userEvent.setup();

    render(
      <ProjectList
        projects={mockProjects}
        currentProject={null}
        onSelectProject={mockOnSelectProject}
        onCreateProject={mockOnCreateProject}
        onDeleteProject={mockOnDeleteProject}
        onRenameProject={mockOnRenameProject}
      />
    );

    const createButton = screen.getByText("新建项目");
    await user.click(createButton);
    expect(mockOnCreateProject).toHaveBeenCalled();
  });

  it("should display project genre badges", async () => {
    render(
      <ProjectList
        projects={mockProjects}
        currentProject={null}
        onSelectProject={mockOnSelectProject}
        onCreateProject={mockOnCreateProject}
        onDeleteProject={mockOnDeleteProject}
        onRenameProject={mockOnRenameProject}
      />
    );

    await waitFor(() => {
      expect(screen.getByText("奇幻")).toBeInTheDocument();
      expect(screen.getByText("科幻")).toBeInTheDocument();
    });
  });
});
