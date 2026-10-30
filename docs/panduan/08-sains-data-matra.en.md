---
judul: "Data Science & ML Ecosystem (Matra)"
grup: "Panduan"
urutan: 8
---

# Data Science & ML Ecosystem (Matra)

## Overview

**Matra** is the official unified data science library in EvernightLanguage for numerical computing,
tabular data analysis, data visualization, classical machine learning, and deep learning.

The core philosophy of Matra is **"One Library for the Entire Data Science Lifecycle"**:
eliminating dependency fragmentation (no need to independently install NumPy, Pandas, Matplotlib,
Scikit-learn, and PyTorch). All components are natively built into the EvernightLanguage compiler and runtime.

## Two-Tier Architecture

1. **Lower Tier (Rust Core Engine)**:
   - Tensor data is stored in a flat contiguous memory buffer (`Vec<f64>`).
   - Matrix multiplication, spatial KD-Tree search (KNN), and automatic differentiation (*autograd*) execute at native C++/Rust speeds.
   - Zero garbage collector pauses managed through Automatic Reference Counting (ARC).

2. **Upper Tier (Evernight `.eve` Syntax)**:
   - Expressive Indonesian syntax.
   - Rich error formatting with clear problem locations (e.g., `BAHAYA [MATRA_SHAPE]: Dimensi matriks tidak cocok untuk perkalian!`).

## Sub-Module Architecture

| Sub-Module | Primary Role | Python Equivalent |
|---|---|---|
| `matra.larik` | N-dimensional tensors, linear algebra, statistical reductions | NumPy, SciPy |
| `matra.tabel` | `Tabel` structure (DataFrame), local CSV I/O, filtering & groupby | Pandas, Polars |
| `matra.grafik` | Standalone SVG vector chart generator | Matplotlib, Seaborn |
| `matra.latih` & `matra.alur` | Data splitting, K-Fold cross validation, leakage-free Pipelines | Scikit-learn (`model_selection`, `Pipeline`) |
| `matra.seimbang` | Minority class resampling (`SMOTE`) | Imbalanced-learn (`imblearn`) |
| `matra.model` | Regression, KNN/Tree/Forest classifiers, KMeans clustering | Scikit-learn (`linear_model`, `ensemble`, etc.) |
| `matra.metrik` | Accuracy, F1, MSE evaluation metrics and classification reports | Scikit-learn (`metrics`) |
| `matra.syaraf` | Autograd engine, Neural Network layers, activations & optimizers | PyTorch, TensorFlow |
| `matra.pelatih` | Automated training orchestrator, Early Stopping, Checkpointing | PyTorch Lightning, Keras |

## Complete Integrated Workflow Example

```eve
impor "matra" sebagai mt

# 1. Membaca data tabular CSV
variabel df = mt.baca_csv("dataset.csv")

# 2. Pembersihan & persiapan data
variabel bersih = df.buang_kosong()
variabel X = bersih.ke_larik(["fitur1", "fitur2"])
variabel y = bersih.ke_larik(["target"])

# 3. Pemisahan data latih & uji
variabel [X_latih, X_uji, y_latih, y_uji] = mt.latih.pisah_data(X, y, 0.2)

# 4. Melatih model
variabel model = mt.model.KNNKlasifikasi(5)
model.latih(X_latih, y_latih)

# 5. Evaluasi performa
variabel y_pred = model.prediksi(X_uji)
cetak("Akurasi Uji: ", mt.metrik.akurasi(y_uji, y_pred))
```
