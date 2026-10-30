//! Kernel eksekusi stateful untuk March Notebook.
//!
//! Mengelola session `Vm::run_incremental()` sehingga variabel, modul, dan model
//! yang didefinisikan di sel sebelumnya tetap tersimpan di memori untuk sel berikutnya.

use super::notebook::MarchOutput;
use evernight_core::ast::Statement;
use evernight_core::{Lexer, Parser};
use evernight_vm::{Compiler, Value, Vm};
use std::path::PathBuf;

pub struct MarchKernel {
    pub vm: Vm,
    pub working_dir: PathBuf,
    pub execution_counter: usize,
}

impl MarchKernel {
    pub fn new(working_dir: PathBuf) -> Self {
        let mut vm = Vm::with_dir(working_dir.clone());
        vm.set_capture_output(true);
        MarchKernel {
            vm,
            working_dir,
            execution_counter: 0,
        }
    }

    pub fn reset(&mut self) {
        let mut vm = Vm::with_dir(self.working_dir.clone());
        vm.set_capture_output(true);
        self.vm = vm;
        self.execution_counter = 0;
    }

    pub fn jalankan_sel(&mut self, kode: &str) -> (usize, MarchOutput) {
        self.execution_counter += 1;
        let urutan = self.execution_counter;

        // 1. Lexing
        let mut lexer = Lexer::new(kode);
        let tokens = match lexer.scan_tokens() {
            Ok(t) => t,
            Err(e) => {
                return (
                    urutan,
                    MarchOutput {
                        tipe: "error".to_string(),
                        konten: format!(
                            "BAHAYA [{}] Baris {} (Kolom {}) - {}",
                            e.code, e.line, e.column, e.message
                        ),
                        baris_tabel: None,
                        kolom_tabel: None,
                        svg: None,
                    },
                );
            }
        };

        // 2. Parsing AST
        let mut parser = Parser::new(tokens);
        let mut ast = match parser.parse() {
            Ok(a) => a,
            Err(e) => {
                return (
                    urutan,
                    MarchOutput {
                        tipe: "error".to_string(),
                        konten: format!(
                            "BAHAYA [{}] Baris {} (Kolom {}) - {}",
                            e.code, e.line, e.column, e.message
                        ),
                        baris_tabel: None,
                        kolom_tabel: None,
                        svg: None,
                    },
                );
            }
        };

        // Jika statement terakhir adalah ExprStatement, ubah menjadi ReturnStatement
        // agar nilai ekspresi terakhir dikembalikan oleh VM (perilaku interaktif Colab/Jupyter)
        if let Some(Statement::ExprStatement { expr, line }) = ast.last().cloned() {
            let last_idx = ast.len() - 1;
            ast[last_idx] = Statement::ReturnStatement {
                value: Some(expr),
                line,
            };
        }

        // 3. Bytecode Compilation
        let mut compiler = Compiler::new();
        if let Err(err_str) = compiler.compile(&ast) {
            return (
                urutan,
                MarchOutput {
                    tipe: "error".to_string(),
                    konten: format!("BAHAYA [KOMPILASI] - {}", err_str),
                    baris_tabel: None,
                    kolom_tabel: None,
                    svg: None,
                },
            );
        }

        // 4. Eksekusi Inkremental pada VM
        self.vm.set_capture_output(true);
        let hasil = self.vm.run_incremental(compiler.chunk);
        let captured = self.vm.take_captured_output();
        let console_text = captured.join("\n");

        match hasil {
            Err(e) => {
                let mut err_msg = format!(
                    "BAHAYA [{}] Baris {} (Kolom {}) - {}",
                    e.code, e.line, e.column, e.message
                );
                if !console_text.is_empty() {
                    err_msg = format!("{}\n\n{}", console_text, err_msg);
                }
                (
                    urutan,
                    MarchOutput {
                        tipe: "error".to_string(),
                        konten: err_msg,
                        baris_tabel: None,
                        kolom_tabel: None,
                        svg: None,
                    },
                )
            }
            Ok(val) => match val {
                Value::Tabel(tbl_rc) => {
                    let tbl = tbl_rc.borrow();
                    let cols = tbl.nama_kolom.clone();
                    let num_rows = if !tbl.kolom.is_empty() {
                        tbl.kolom[0].len()
                    } else {
                        0
                    };
                    let max_display = num_rows.min(60);
                    let mut rows = Vec::new();
                    for r in 0..max_display {
                        let mut row = Vec::new();
                        for c in 0..cols.len() {
                            row.push(tbl.kolom[c][r].to_string());
                        }
                        rows.push(row);
                    }
                    (
                        urutan,
                        MarchOutput {
                            tipe: "tabel_html".to_string(),
                            konten: console_text,
                            baris_tabel: Some(rows),
                            kolom_tabel: Some(cols),
                            svg: None,
                        },
                    )
                }
                Value::Grafik(plot_rc) => {
                    let svg = plot_rc.borrow().ke_svg();
                    (
                        urutan,
                        MarchOutput {
                            tipe: "grafik_svg".to_string(),
                            konten: console_text,
                            baris_tabel: None,
                            kolom_tabel: None,
                            svg: Some(svg),
                        },
                    )
                }
                Value::Kosong => (
                    urutan,
                    MarchOutput {
                        tipe: "teks".to_string(),
                        konten: console_text,
                        baris_tabel: None,
                        kolom_tabel: None,
                        svg: None,
                    },
                ),
                other => {
                    let mut out = console_text;
                    let val_str = other.to_string();
                    if !val_str.is_empty() {
                        if !out.is_empty() {
                            out.push('\n');
                        }
                        out.push_str(&val_str);
                    }
                    (
                        urutan,
                        MarchOutput {
                            tipe: "teks".to_string(),
                            konten: out,
                            baris_tabel: None,
                            kolom_tabel: None,
                            svg: None,
                        },
                    )
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_stateful_execution() {
        let mut kernel = MarchKernel::new(PathBuf::from("."));

        // Sel 1: Deklarasi variabel
        let (seq1, out1) = kernel.jalankan_sel("variabel a = 42\nvariabel b = 8");
        assert_eq!(seq1, 1);
        assert_eq!(out1.tipe, "teks");

        // Sel 2: Menggunakan variabel dari sel 1
        let (seq2, out2) = kernel.jalankan_sel("variabel c = a + b\ncetak(\"Hasil: \", c)");
        assert_eq!(seq2, 2);
        assert_eq!(out2.tipe, "teks");
        assert!(out2.konten.contains("Hasil: 50"));
    }

    #[test]
    fn test_kernel_output_dataframe_dan_svg() {
        let mut kernel = MarchKernel::new(PathBuf::from("."));

        // Sel 1: Impor matra dan buat tabel
        let (_seq, out_tbl) = kernel.jalankan_sel("impor \"matra\" sebagai mt\nmt.tabel({\"umur\": [20, 25], \"skor\": [88.5, 92.0]})");
        assert_eq!(out_tbl.tipe, "tabel_html");
        assert!(out_tbl.kolom_tabel.is_some());
        let cols = out_tbl.kolom_tabel.unwrap();
        assert!(cols.contains(&"umur".to_string()));
        assert!(cols.contains(&"skor".to_string()));
        assert_eq!(out_tbl.baris_tabel.unwrap().len(), 2);

        // Sel 2: Buat grafik plot SVG
        let (_seq2, out_svg) = kernel.jalankan_sel("mt.grafik.garis([1, 2, 3], [10, 20, 30])");
        assert_eq!(out_svg.tipe, "grafik_svg");
        assert!(out_svg.svg.is_some());
        assert!(out_svg.svg.unwrap().contains("<svg"));
    }

    #[test]
    fn test_kernel_error_bahaya() {
        let mut kernel = MarchKernel::new(PathBuf::from("."));
        let (_seq, out_err) = kernel.jalankan_sel("variabel x = 10 / 0");
        assert_eq!(out_err.tipe, "error");
        assert!(out_err.konten.contains("BAHAYA"));
        assert!(out_err.konten.contains("DIVISION") || out_err.konten.contains("nol") || out_err.konten.contains("Pembagian"));
    }

    #[test]
    fn test_kernel_reset() {
        let mut kernel = MarchKernel::new(PathBuf::from("."));
        kernel.jalankan_sel("variabel test_var = 999");
        assert_eq!(kernel.execution_counter, 1);

        kernel.reset();
        assert_eq!(kernel.execution_counter, 0);

        // Variabel lama seharusnya sudah hilang
        let (_seq, out) = kernel.jalankan_sel("cetak(test_var)");
        assert_eq!(out.tipe, "error");
        assert!(out.konten.contains("BAHAYA"));
    }
}
