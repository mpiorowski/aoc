# Advent of Code CLI Manager (Rust)

## Overview
A personal **TUI (Text User Interface)** tool written in Rust to streamline the Advent of Code workflow. The user runs a single command to launch a dashboard-style interface from the root of the AoC repository, allowing them to manage the entire lifecycle of daily challenges without leaving the terminal UI.

## Core Features

### 1. TUI Dashboard & Navigation
- **Single Entry Point:** The app is launched once and remains active.
- **Visual Navigation:** Navigate between Years and Days using keyboard shortcuts or arrow keys.
- **Status Overview:** Visual indicators for days that are completed, in progress, or locked.

### 2. Day Scaffolding (Configuration)
- **New Day Setup:**
  - Trigger a "Create Day" action within the UI.
  - Automatically create the directory structure and solution template.
  - Template requirement: A function that accepts input and returns a solution.

### 3. Input Management (In-App)
- **Input Editor/Paste:** 
  - A TUI popup or pane to paste `test_input` directly.
  - Auto-save content to `test_input.txt`.
- **Test Expectations:**
  - Input field to define the expected `test_solution`.
- **Puzzle Input:**
  - Manage `input_1.txt` and `input_2.txt` similarly.

### 4. Execution & Validation
- **Interactive Runner:**
  - "Run Test" and "Run Solution" buttons/hotkeys.
  - **Live Output:** View stdout/stderr and results in a dedicated results pane within the TUI.
  - **Validation:** Visual feedback (Green/Red) for test cases.

## Technical Goals
- Written in **Rust**.
- Focus on ease of use ("sweet cli").
- "One root folder" architecture.

## Technical Stack
- **UI Engine:** `ratatui` (Standard Rust TUI library).
- **Backend:** `crossterm` (Terminal manipulation).
- **Text Editing:** `tui-textarea` (For inputting test data/config directly in terminal).
- **Async Runtime:** `tokio` (For non-blocking solution execution).
- **Error Handling:** `color-eyre` (Panic handling/restore terminal).
- **Data:** `serde` + `serde_json` (Config persistence).

## Future Ideas
- **Watch Mode:** specialized "dev" mode that watches the current day's solution file and auto-runs tests on save, flashing results in the TUI without manual triggering.
- **Snippet Toolbox:** A library of common AoC patterns (Dijkstra, Grid, BFS) that can be injected into the current solution via the TUI.
- **Performance Profiling:** Visual execution timer that compares Part 1 vs Part 2 runtimes (e.g., "Part 1: 4ms | Part 2: 15s").
- **Visualizer Hook:** A dedicated TUI pane that listens for specific debug output to render live 2D grids or state visualizations for simulation puzzles.

## Collaboration Strategy
- **User Role:** Lead Developer & Architect. Focuses on writing core logic, state management, and learning the `ratatui` ecosystem by doing.
- **Agent Role:** Consultant & UI/UX Designer.
  - **Responsibilities:**
    - Provide "small helps" and specific widget examples when asked.
    - Propose UI/UX layouts and flows.
    - Handle boilerplate or repetitive UI code if delegated.
    - **Do NOT** write the core logic unless explicitly asked.
  - **Goal:** Support the user in "feeling" the TUI code and mastering the library.
