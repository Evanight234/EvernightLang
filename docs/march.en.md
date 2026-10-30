---
judul: "March Web Notebook (.emlbn)"
grup: "Mulai"
urutan: 5
---

# March Web Notebook (.emlbn)

## What is March?

**March** is the official interactive web notebook environment for EvernightLanguage,
inspired by the modern **Google Colab** (Dark Mode) user experience.

March is tailored for data exploration, machine learning experiments, and rapid prototyping
powered by the **`Matra`** ecosystem, operating directly in your web browser with zero external cloud dependencies.

![March Web Notebook Interface](https://raw.githubusercontent.com/Evanight234/EvernightLang/main/assets/march_preview.png)

## Key Highlights

1. **100% Local File Access**:
   - Unlike cloud notebooks that require uploading datasets to ephemeral remote servers, March reads and writes real files directly from your computer storage.
   - Calling `mt.baca_csv("sales.csv")` immediately inspects your local working folder.

2. **Dedicated `.emlbn` Format**:
   - Notebooks persist as `.emlbn` (*Evernight Machine Learning Notebook*) JSON files containing code cells, Markdown documentation, and cell execution status.

3. **Dual-Layer Intelligent Editor**:
   - **Real-Time Syntax Highlighting**: Colorizes code automatically with the Nusantara Dark palette.
   - **IntelliSense Auto-Complete**: Typing keyword prefixes (e.g., `j`) surfaces intelligent suggestions (`jika`, `jk`, `jkl`, etc.) navigable via arrow keys and `Tab`/`Enter`.

4. **Rich Interactive Outputs**:
   - `Tabel` (*DataFrames*) render into responsive HTML tables with clean column headers and zebra striping.
   - `Grafik` plots display as crisp vector SVG charts directly inline.

5. **Standalone Zero-Dependency Server**:
   - Integrated directly into the native `evernight.exe` binary with pure Rust, requiring no Python, Node.js, or external daemon.

## Running March

Run the following command in your terminal:

```powershell
# Open March Web Notebook in your default browser
evernight march

# Open or create a specific notebook file
evernight march eksperimen.emlbn

# Bind to a custom port
evernight march --port 8888
```

## Exporting Notebooks to `.eve` Scripts

To bundle all code cells into a standalone Evernight script:

```powershell
evernight march ekspor eksperimen.emlbn
```

This generates `eksperimen.eve`, which can run directly via `evernight run eksperimen.eve`.
