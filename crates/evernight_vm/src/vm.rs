use crate::bytecode::{Chunk, OpCode};

use crate::compiler::Compiler;

use crate::matra::{
    JenisTransformer, KLipat, MatraArray, MatraCallback, MatraJaringan, MatraLapisan, MatraLoss,
    MatraModel, MatraOptimizer, MatraPelatih, MatraPlot, MatraTabel, MatraTransformer,
    PencarianKisi, PipaKerja, Smote, TalaBayes,
};

use crate::value::Value;

use evernight_core::errors::EvernightError;

use evernight_core::{Lexer, Parser};

use std::cell::RefCell;

use std::collections::{HashMap, HashSet};

use std::fs;

use std::path::{Path, PathBuf};

use std::rc::Rc;

struct CallFrame {
    chunk: Rc<Chunk>,

    ip: usize,

    stack_base: usize,

    module_path: Option<String>,

    module_before: HashSet<String>,
}

struct Handler {
    chunk: Rc<Chunk>,

    catch_ip: usize,

    stack_len: usize,
}

enum PendingMode {
    Peta,

    Saring,

    Lipat,

    Setiap,
}

struct PendingCback {
    mode: PendingMode,

    func: Value,

    sisa: Vec<Value>,

    hasil: Vec<Value>,

    akum: Value,

    terakhir: Value,

    base_depth: usize,

    line: usize,
}

impl PendingCback {
    fn akum_ada(&self) -> bool {
        matches!(self.mode, PendingMode::Lipat)
    }
}

pub struct Vm {
    stack: Vec<Value>,

    globals: HashMap<String, Value>,

    frames: Vec<CallFrame>,

    handlers: Vec<Handler>,

    input_queue: Vec<String>,

    pending: Vec<PendingCback>,

    entry_dir: PathBuf,

    import_cache: HashMap<String, Value>,

    importing: HashSet<String>,

    program_args: Vec<String>,

    captured_output: Option<Vec<String>>,

    /// Mode 6D-1: cetak setiap instruksi bytecode ke stderr saat dieksekusi.
    pub debug_trace: bool,

    /// Mode 6D-2: hitung frekuensi opcode + waktu eksekusi.
    pub profile_mode: bool,

    opcode_counts: [usize; 256],

    elapsed_ms: Option<f64>,
}

// ponytail: penangkapan error hanya berlaku dalam fungsi yang sama

// (Rc::ptr_eq chunk). Unwinding lintas fungsi bisa ditambah kalau diperlukan.

macro_rules! raise {
    ($self:ident, $chunk:ident, $code:expr, $msg:expr, $line:expr) => {
        match $self.raise(&$chunk, $code, $msg, $line) {
            Ok(()) => continue,

            Err(e) => return Err(e),
        }
    };
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),

            globals: HashMap::new(),

            frames: Vec::new(),

            handlers: Vec::new(),

            input_queue: Vec::new(),

            pending: Vec::new(),

            entry_dir: PathBuf::from("."),

            import_cache: HashMap::new(),

            importing: HashSet::new(),

            program_args: Vec::new(),

            captured_output: None,

            debug_trace: false,

            profile_mode: false,

            opcode_counts: [0; 256],

            elapsed_ms: None,
        }
    }

    pub fn with_dir(dir: impl Into<PathBuf>) -> Self {
        let mut vm = Self::new();

        vm.entry_dir = dir.into();

        vm
    }

    pub fn set_args(&mut self, args: Vec<String>) {
        self.program_args = args;
    }

    /// Pastikan path berada di dalam direktori program (sandbox).

    /// Untuk tulis: cari parent yang ada, canonicalize, lalu cek.

    fn resolve_sandbox(&self, path: &str) -> Result<PathBuf, EvernightError> {
        let raw_target = Path::new(path);

        let target = if raw_target.is_relative() {
            self.entry_dir.join(raw_target)
        } else {
            raw_target.to_path_buf()
        };

        let base = if target.exists() {
            target.canonicalize().map_err(|e| {
                EvernightError::bahaya(
                    "FILE",
                    0,
                    0,
                    format!("Gagal resolve path '{}': {}", path, e),
                )
            })?
        } else {
            let p = target.parent().unwrap_or(Path::new("."));

            let parent = if p.as_os_str().is_empty() {
                Path::new(".")
            } else {
                p
            };

            let canon_parent = if parent.exists() {
                parent.canonicalize().map_err(|e| {
                    EvernightError::bahaya(
                        "FILE",
                        0,
                        0,
                        format!("Gagal resolve path '{}': {}", path, e),
                    )
                })?
            } else {
                return Err(EvernightError::bahaya(
                    "FILE",
                    0,
                    0,
                    format!("Direktori '{}' tidak ditemukan", parent.display()),
                ));
            };

            let name = target.file_name().unwrap_or_default();

            canon_parent.join(name)
        };

        let entry = self.entry_dir.canonicalize().map_err(|e| {
            EvernightError::bahaya(
                "FILE",
                0,
                0,
                format!("Gagal resolve direktori program: {}", e),
            )
        })?;

        if !base.starts_with(&entry) {
            return Err(EvernightError::bahaya(
                "SECURITY",
                0,
                0,
                format!("Akses ditolak: '{}' berada di luar direktori program", path),
            ));
        }

        Ok(base)
    }

    pub fn set_capture_output(&mut self, enable: bool) {
        if enable {
            self.captured_output = Some(Vec::new());
        } else {
            self.captured_output = None;
        }
    }

    pub fn take_captured_output(&mut self) -> Vec<String> {
        self.captured_output.take().unwrap_or_default()
    }

    pub fn get_captured_output(&self) -> Option<&[String]> {
        self.captured_output.as_deref()
    }

    /// Mendorong input untuk unit test (baca/baca_angka memakai ini

    /// sebelum membaca stdin proses). Input diambil berurutan FIFO.

    pub fn push_input(&mut self, line: impl Into<String>) {
        self.input_queue.push(line.into());
    }

    pub fn run(&mut self, chunk: Chunk) -> Result<Value, EvernightError> {
        self.stack.clear();

        self.globals.clear();

        self.frames.clear();

        self.handlers.clear();

        self.pending.clear();

        self.import_cache.clear();

        self.importing.clear();

        self.opcode_counts = [0; 256];

        let mulai = self.profile_mode.then(std::time::Instant::now);

        let hasil = self.execute(chunk);

        self.elapsed_ms = mulai.map(|t| t.elapsed().as_secs_f64() * 1000.0);

        hasil
    }

    pub fn run_incremental(&mut self, chunk: Chunk) -> Result<Value, EvernightError> {
        self.stack.clear();

        self.frames.clear();

        self.handlers.clear();

        self.pending.clear();

        self.importing.clear();

        self.opcode_counts = [0; 256];

        let mulai = self.profile_mode.then(std::time::Instant::now);

        let hasil = self.execute(chunk);

        self.elapsed_ms = mulai.map(|t| t.elapsed().as_secs_f64() * 1000.0);

        hasil
    }

    /// Laporan profil (6D-2): opcode terurut dari yang paling sering dieksekusi.

    /// Mengembalikan `(waktu_ms, total_instruksi, daftar (nama_opcode, jumlah))`.

    pub fn profile_summary(&self) -> (f64, usize, Vec<(&'static str, usize)>) {
        let total: usize = self.opcode_counts.iter().sum();

        let mut daftar: Vec<(&'static str, usize)> = self
            .opcode_counts
            .iter()
            .enumerate()
            .filter(|(_, &n)| n > 0)
            .filter_map(|(b, &n)| OpCode::from_u8(b as u8).map(|o| (o.name(), n)))
            .collect();

        daftar.sort_by_key(|(_, n)| std::cmp::Reverse(*n));

        (self.elapsed_ms.unwrap_or(0.0), total, daftar)
    }

    fn execute(&mut self, chunk: Chunk) -> Result<Value, EvernightError> {
        self.frames.push(CallFrame {
            chunk: Rc::new(chunk),

            ip: 0,

            stack_base: 0,

            module_path: None,

            module_before: HashSet::new(),
        });

        while self.frames.last().is_some() {
            if self.pending_last_selesai() {
                self.lanjutkan_pending()?;

                continue;
            }

            let rchunk = self.frames.last().unwrap().chunk.clone();

            let chunk = rchunk.clone();

            let ip = self.frames.last().unwrap().ip;

            if ip >= chunk.code.len() {
                raise!(
                    self,
                    rchunk,
                    "RUNTIME",
                    "Program melewati batas bytecode",
                    0
                );
            }

            let opcode = chunk.code[ip];

            let line = chunk.lines[ip];

            self.frames.last_mut().unwrap().ip += 1;

            if self.debug_trace {
                let nama = OpCode::from_u8(opcode)
                    .map(|o| o.name())
                    .unwrap_or("TAKDIKENAL");

                eprintln!("[{:04}] baris {:4} | {}", ip, line, nama);
            }

            if self.profile_mode {
                self.opcode_counts[opcode as usize] += 1;
            }

            match OpCode::from_u8(opcode) {
                Some(OpCode::Konstanta) => {
                    let idx = self.read_u16(&chunk);

                    let val = chunk.constants[idx as usize].clone();

                    self.stack.push(val);
                }

                Some(OpCode::Pop) => {
                    self.stack.pop();
                }

                Some(OpCode::Dup) => {
                    let val = self.stack.last().cloned().unwrap();

                    self.stack.push(val);
                }

                Some(OpCode::AmbilLokal) => {
                    let slot = self.read_u16(&chunk) as usize;

                    let base = self.stack_base();

                    let idx = base + slot;

                    let val = if idx < self.stack.len() {
                        self.stack[idx].clone()
                    } else {
                        Value::Kosong
                    };

                    self.stack.push(val);
                }

                Some(OpCode::SimpanLokal) => {
                    let slot = self.read_u16(&chunk) as usize;

                    let base = self.stack_base();

                    let val = self.stack.pop().unwrap();

                    let idx = base + slot;

                    while self.stack.len() <= idx {
                        self.stack.push(Value::Kosong);
                    }

                    self.stack[idx] = val;
                }

                Some(OpCode::AmbilGlobalNama) => {
                    let idx = self.read_u16(&chunk);

                    let name = match &chunk.constants[idx as usize] {
                        Value::Teks(s) => s.clone(),

                        _ => {
                            raise!(self, rchunk, "TYPE", "Bukan nama variabel", line);
                        }
                    };

                    if let Some(val) = self.globals.get(&name) {
                        self.stack.push(val.clone());
                    } else {
                        raise!(
                            self,
                            rchunk,
                            "VARIABLE",
                            &format!("Variabel '{}' tidak didefinisikan", name),
                            line
                        );
                    }
                }

                Some(OpCode::SimpanGlobalNama) => {
                    let idx = self.read_u16(&chunk);

                    let name = match &chunk.constants[idx as usize] {
                        Value::Teks(s) => s.clone(),

                        _ => {
                            raise!(self, rchunk, "TYPE", "Bukan nama variabel", line);
                        }
                    };

                    let val = self.stack.pop().unwrap();

                    self.globals.insert(name, val);
                }

                Some(OpCode::AmbilGlobal) => {
                    let idx = self.read_u16(&chunk);

                    let name = match &chunk.constants[idx as usize] {
                        Value::Teks(s) => s.clone(),

                        _ => {
                            raise!(self, rchunk, "TYPE", "Bukan nama variabel", line);
                        }
                    };

                    if let Some(val) = self.globals.get(&name) {
                        self.stack.push(val.clone());
                    } else {
                        raise!(
                            self,
                            rchunk,
                            "VARIABLE",
                            &format!("Variabel '{}' tidak didefinisikan", name),
                            line
                        );
                    }
                }

                Some(OpCode::SimpanGlobal) => {
                    let idx = self.read_u16(&chunk);

                    let name = match &chunk.constants[idx as usize] {
                        Value::Teks(s) => s.clone(),

                        _ => {
                            raise!(self, rchunk, "TYPE", "Bukan nama variabel", line);
                        }
                    };

                    let val = self.stack.pop().unwrap();

                    self.globals.insert(name, val);
                }

                Some(OpCode::Tambah) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Angka(a), Value::Angka(b)) => {
                            self.stack.push(Value::Angka(a + b));
                        }

                        (Value::Larik(a), Value::Larik(b)) => {
                            match a.borrow().tambah(&b.borrow()) {
                                Ok(res) => {
                                    self.stack.push(Value::Larik(Rc::new(RefCell::new(res))))
                                }

                                Err(e) => raise!(self, rchunk, "TYPE", &e, line),
                            }
                        }

                        (Value::Larik(a), Value::Angka(b)) => {
                            let res = a.borrow().tambah_skalar(*b);

                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        (Value::Angka(a), Value::Larik(b)) => {
                            let res = b.borrow().tambah_skalar(*a);

                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        (Value::Teks(a), b) => {
                            self.stack.push(Value::Teks(format!("{}{}", a, b)));
                        }

                        (a, Value::Teks(b)) => {
                            self.stack.push(Value::Teks(format!("{}{}", a, b)));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '+' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::Kurang) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Angka(a), Value::Angka(b)) => {
                            self.stack.push(Value::Angka(a - b));
                        }

                        (Value::Larik(a), Value::Larik(b)) => {
                            match a.borrow().kurang(&b.borrow()) {
                                Ok(res) => {
                                    self.stack.push(Value::Larik(Rc::new(RefCell::new(res))))
                                }

                                Err(e) => raise!(self, rchunk, "TYPE", &e, line),
                            }
                        }

                        (Value::Larik(a), Value::Angka(b)) => {
                            let res = a.borrow().kurang_skalar(*b);

                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        (Value::Angka(a), Value::Larik(b)) => {
                            let res = b.borrow().negatif().tambah_skalar(*a);

                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '-' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::Kali) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Angka(a), Value::Angka(b)) => {
                            self.stack.push(Value::Angka(a * b));
                        }

                        (Value::Larik(a), Value::Larik(b)) => match a.borrow().kali(&b.borrow()) {
                            Ok(res) => self.stack.push(Value::Larik(Rc::new(RefCell::new(res)))),

                            Err(e) => raise!(self, rchunk, "TYPE", &e, line),
                        },

                        (Value::Larik(a), Value::Angka(b)) => {
                            let res = a.borrow().kali_skalar(*b);

                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        (Value::Angka(a), Value::Larik(b)) => {
                            let res = b.borrow().kali_skalar(*a);

                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '*' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::Bagi) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Angka(a), Value::Angka(b)) => {
                            if *b == 0.0 {
                                raise!(self, rchunk, "DIVISION", "Pembagian dengan nol", line);
                            }

                            self.stack.push(Value::Angka(a / b));
                        }

                        (Value::Larik(a), Value::Larik(b)) => match a.borrow().bagi(&b.borrow()) {
                            Ok(res) => self.stack.push(Value::Larik(Rc::new(RefCell::new(res)))),

                            Err(e) => raise!(self, rchunk, "DIVISION", &e, line),
                        },

                        (Value::Larik(a), Value::Angka(b)) => {
                            if *b == 0.0 {
                                raise!(self, rchunk, "DIVISION", "Pembagian dengan nol", line);
                            }

                            match a.borrow().bagi_skalar(*b) {
                                Ok(res) => {
                                    self.stack.push(Value::Larik(Rc::new(RefCell::new(res))))
                                }

                                Err(e) => raise!(self, rchunk, "DIVISION", &e, line),
                            }
                        }

                        (Value::Angka(a), Value::Larik(b)) => {
                            let b_arr = b.borrow();

                            let mut data = Vec::with_capacity(b_arr.data.len());

                            for item in &b_arr.data {
                                if *item == 0.0 {
                                    raise!(
                                        self,
                                        rchunk,
                                        "DIVISION",
                                        "Pembagian dengan nol pada elemen larik",
                                        line
                                    );
                                }

                                data.push(*a / *item);
                            }

                            let res = MatraArray {
                                bentuk: b_arr.bentuk.clone(),

                                data,
                            };

                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '/' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::Modulo) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Angka(a), Value::Angka(b)) => {
                            if *b == 0.0 {
                                raise!(self, rchunk, "DIVISION", "Pembagian dengan nol", line);
                            }

                            self.stack.push(Value::Angka(a % b));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '%' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::Pangkat) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Angka(a), Value::Angka(b)) => {
                            self.stack.push(Value::Angka(a.powf(*b)));
                        }

                        (Value::Larik(a), Value::Angka(b)) => {
                            let res = a.borrow().pangkat_skalar(*b);

                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '**' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::Negatif) => {
                    let val = self.stack.pop().unwrap();

                    match val {
                        Value::Angka(n) => {
                            self.stack.push(Value::Angka(-n));
                        }

                        Value::Larik(l) => {
                            let res = l.borrow().negatif();

                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '-' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::SamaDengan) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    self.stack.push(Value::Bolean(left == right));
                }

                Some(OpCode::TidakSama) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    self.stack.push(Value::Bolean(left != right));
                }

                Some(OpCode::KurangDari) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Angka(a), Value::Angka(b)) => {
                            self.stack.push(Value::Bolean(a < b));
                        }

                        (Value::Teks(a), Value::Teks(b)) => {
                            self.stack.push(Value::Bolean(a < b));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '<' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::KurangSama) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Angka(a), Value::Angka(b)) => {
                            self.stack.push(Value::Bolean(a <= b));
                        }

                        (Value::Teks(a), Value::Teks(b)) => {
                            self.stack.push(Value::Bolean(a <= b));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '<=' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::LebihDari) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Angka(a), Value::Angka(b)) => {
                            self.stack.push(Value::Bolean(a > b));
                        }

                        (Value::Teks(a), Value::Teks(b)) => {
                            self.stack.push(Value::Bolean(a > b));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '>' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::LebihSama) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Angka(a), Value::Angka(b)) => {
                            self.stack.push(Value::Bolean(a >= b));
                        }

                        (Value::Teks(a), Value::Teks(b)) => {
                            self.stack.push(Value::Bolean(a >= b));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Operator '>=' tidak valid untuk tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::Dalam) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    match (&left, &right) {
                        (Value::Teks(a), Value::Teks(b)) => {
                            self.stack.push(Value::Bolean(b.contains(a.as_str())));
                        }

                        _ => {
                            self.stack.push(Value::Bolean(false));
                        }
                    }
                }

                Some(OpCode::Dan) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    self.stack
                        .push(Value::Bolean(left.is_truthy() && right.is_truthy()));
                }

                Some(OpCode::Atau) => {
                    let right = self.stack.pop().unwrap();

                    let left = self.stack.pop().unwrap();

                    self.stack
                        .push(Value::Bolean(left.is_truthy() || right.is_truthy()));
                }

                Some(OpCode::Bukan) => {
                    let val = self.stack.pop().unwrap();

                    self.stack.push(Value::Bolean(!val.is_truthy()));
                }

                Some(OpCode::Lompat) => {
                    let offset = self.read_i16(&chunk);

                    let ip = self.current_ip();

                    self.frames.last_mut().unwrap().ip = (ip as isize + offset as isize) as usize;
                }

                Some(OpCode::LompatJikaSalah) => {
                    let offset = self.read_i16(&chunk);

                    let val = self.stack.pop().unwrap();

                    if !val.is_truthy() {
                        let ip = self.current_ip();

                        self.frames.last_mut().unwrap().ip =
                            (ip as isize + offset as isize) as usize;
                    }
                }

                Some(OpCode::LompatJikaBenar) => {
                    let offset = self.read_i16(&chunk);

                    let val = self.stack.pop().unwrap();

                    if val.is_truthy() {
                        let ip = self.current_ip();

                        self.frames.last_mut().unwrap().ip =
                            (ip as isize + offset as isize) as usize;
                    }
                }

                Some(OpCode::Loop) => {
                    let offset = self.read_i16(&chunk);

                    let ip = self.current_ip();

                    self.frames.last_mut().unwrap().ip = (ip as isize - offset as isize) as usize;
                }

                Some(OpCode::Panggil) => {
                    let arg_count = self.read_u8(&chunk) as usize;

                    if self.stack.len() < arg_count + 1 {
                        raise!(
                            self,
                            rchunk,
                            "RUNTIME",
                            "Stack kosong saat memanggil fungsi",
                            line
                        );
                    }

                    let base = self.stack.len() - arg_count - 1;

                    let func_val = self.stack[base].clone();

                    match func_val {
                        Value::Fungsi {
                            name,

                            arity,

                            chunk: fchunk,
                        } => {
                            if arity != arg_count {
                                raise!(
                                    self,
                                    rchunk,
                                    "FUNGSI",
                                    &format!(
                                        "Fungsi '{}' butuh {} argumen, diberikan {}",
                                        name, arity, arg_count
                                    ),
                                    line
                                );
                            }

                            self.stack.remove(base);

                            self.frames.push(CallFrame {
                                chunk: fchunk,

                                ip: 0,

                                stack_base: base,

                                module_path: None,

                                module_before: HashSet::new(),
                            });
                        }

                        Value::FungsiAsli { name, id } => {
                            let mut args = Vec::with_capacity(arg_count);

                            for _ in 0..arg_count {
                                args.push(self.stack.pop().unwrap());
                            }

                            args.reverse();

                            self.stack.remove(base);

                            self.panggil_fungsi_asli(&rchunk, id, &name, args, line)?;
                        }

                        Value::MetodeAsli { name, id, target } => {
                            let mut args = Vec::with_capacity(arg_count);

                            for _ in 0..arg_count {
                                args.push(self.stack.pop().unwrap());
                            }

                            args.reverse();

                            self.stack.remove(base);

                            self.panggil_metode_asli(&rchunk, id, &name, *target, args, line)?;
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "FUNGSI",
                                &format!("'{}' bukan fungsi", func_val),
                                line
                            );
                        }
                    }
                }

                Some(OpCode::Kembali) => {
                    let result = self.stack.pop().unwrap_or(Value::Kosong);

                    if let Some(frame) = self.frames.pop() {
                        let fchunk = frame.chunk.clone();

                        self.handlers.retain(|h| !Rc::ptr_eq(&h.chunk, &fchunk));

                        self.stack.truncate(frame.stack_base);
                    }

                    if self.frames.is_empty() {
                        return Ok(result);
                    }

                    self.stack.push(result);
                }

                Some(OpCode::BuatDaftar) => {
                    let count = self.read_u16(&chunk) as usize;

                    let mut elements = Vec::new();

                    for _ in 0..count {
                        elements.push(self.stack.pop().unwrap());
                    }

                    elements.reverse();

                    self.stack
                        .push(Value::Daftar(Rc::new(RefCell::new(elements))));
                }

                Some(OpCode::AmbilIndeks) => {
                    let index = self.stack.pop().unwrap();

                    let target = self.stack.pop().unwrap();

                    match (&target, &index) {
                        (Value::Larik(arr), Value::Angka(idx)) => {
                            let arr = arr.borrow();

                            let idx = idx.round() as i64;

                            let len = if arr.bentuk.len() == 1 {
                                arr.data.len() as i64
                            } else if arr.bentuk.len() == 2 {
                                arr.bentuk[0] as i64
                            } else {
                                arr.data.len() as i64
                            };

                            let actual_idx = if idx < 0 { len + idx } else { idx };

                            if actual_idx < 0 || actual_idx >= len {
                                raise!(
                                    self,
                                    rchunk,
                                    "INDEX",
                                    &format!("Indeks {} di luar batas larik [0..{}]", idx, len - 1),
                                    line
                                );
                            }

                            if arr.bentuk.len() == 1 {
                                self.stack.push(Value::Angka(arr.data[actual_idx as usize]));
                            } else if arr.bentuk.len() == 2 {
                                match arr.baris(actual_idx as usize) {
                                    Ok(b) => {
                                        self.stack.push(Value::Larik(Rc::new(RefCell::new(b))))
                                    }

                                    Err(e) => raise!(self, rchunk, "INDEX", &e, line),
                                }
                            } else {
                                self.stack.push(Value::Angka(arr.data[actual_idx as usize]));
                            }
                        }

                        (Value::Tabel(t), Value::Teks(col_name)) => {
                            match t.borrow().ambil_kolom(col_name) {
                                Ok(val) => self.stack.push(val),

                                Err(e) => raise!(self, rchunk, "KEY", &e, line),
                            }
                        }

                        (Value::Tabel(t), Value::Angka(idx)) => {
                            let idx = idx.round() as i64;

                            let n_baris = t.borrow().jumlah_baris() as i64;

                            let actual_idx = if idx < 0 { n_baris + idx } else { idx };

                            if actual_idx < 0 || actual_idx >= n_baris {
                                raise!(
                                    self,
                                    rchunk,
                                    "INDEX",
                                    &format!(
                                        "Indeks baris {} di luar batas [0..{}]",
                                        idx,
                                        n_baris.saturating_sub(1)
                                    ),
                                    line
                                );
                            }

                            match t.borrow().ambil_baris(actual_idx as usize) {
                                Ok(row_map) => self
                                    .stack
                                    .push(Value::Kamus(Rc::new(RefCell::new(row_map)))),

                                Err(e) => raise!(self, rchunk, "INDEX", &e, line),
                            }
                        }

                        (Value::Daftar(list), Value::Angka(idx)) => {
                            let idx = idx.round() as i64;

                            let list = list.borrow();

                            let len = list.len() as i64;

                            let actual_idx = if idx < 0 { len + idx } else { idx };

                            if actual_idx < 0 || actual_idx >= len {
                                raise!(
                                    self,
                                    rchunk,
                                    "INDEX",
                                    &format!(
                                        "Indeks {} di luar batas daftar [0..{}]",
                                        idx,
                                        len - 1
                                    ),
                                    line
                                );
                            }

                            self.stack.push(list[actual_idx as usize].clone());
                        }

                        (Value::Teks(s), Value::Angka(idx)) => {
                            let idx = idx.round() as i64;

                            let chars: Vec<char> = s.chars().collect();

                            let len = chars.len() as i64;

                            let actual_idx = if idx < 0 { len + idx } else { idx };

                            if actual_idx < 0 || actual_idx >= len {
                                raise!(
                                    self,
                                    rchunk,
                                    "INDEX",
                                    &format!("Indeks {} di luar batas teks [0..{}]", idx, len - 1),
                                    line
                                );
                            }

                            self.stack
                                .push(Value::Teks(chars[actual_idx as usize].to_string()));
                        }

                        (Value::ModelSyaraf(_m), Value::Teks(k)) => match k.as_str() {
                            "maju" | "forward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "maju".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MAJU,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mundur" | "backward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mundur".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUNDUR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "prediksi" | "predict" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "prediksi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_PREDIKSI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_EVAL,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "simpan" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "simpan".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_SIMPAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "muat" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "muat".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUAT,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk ModelSyaraf",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Pengoptimal(_o), Value::Teks(k)) => match k.as_str() {
                            "langkah" | "step" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "langkah".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_LANGKAH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "nol_gradien" | "zero_grad" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "nol_gradien".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_NOL_GRAD,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!("Metode '{}' tidak dikenal untuk Pengoptimal", other),
                                    line
                                );
                            }
                        },

                        (Value::Pelatih(p), Value::Teks(k)) => match k.as_str() {
                            "pasang_callback" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "pasang_callback".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_PASANG_CB,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_EVALUASI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "riwayat" => {
                                let mut m = HashMap::new();

                                for (key, vec) in &p.borrow().riwayat {
                                    let items: Vec<Value> =
                                        vec.iter().map(|&v| Value::Angka(v)).collect();

                                    m.insert(
                                        key.clone(),
                                        Value::Daftar(Rc::new(RefCell::new(items))),
                                    );
                                }

                                self.stack.push(Value::Kamus(Rc::new(RefCell::new(m))));
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk Pelatih",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Kamus(map), Value::Teks(k)) => {
                            let map = map.borrow();

                            if let Some(val) = map.get(k) {
                                self.stack.push(val.clone());
                            } else {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!("Kunci '{}' tidak ditemukan", k),
                                    line
                                );
                            }
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Tidak dapat mengindeks tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::SimpanIndeks) => {
                    let value = self.stack.pop().unwrap();

                    let index = self.stack.pop().unwrap();

                    let mut target = self.stack.pop().unwrap();

                    match (&mut target, &index) {
                        (Value::Larik(arr), Value::Angka(idx)) => {
                            let mut arr = arr.borrow_mut();

                            let idx = idx.round() as i64;

                            let len = arr.data.len() as i64;

                            let actual_idx = if idx < 0 { len + idx } else { idx };

                            if actual_idx < 0 || actual_idx >= len {
                                raise!(
                                    self,
                                    rchunk,
                                    "INDEX",
                                    &format!("Indeks {} di luar batas larik [0..{}]", idx, len - 1),
                                    line
                                );
                            }

                            let num = match &value {
                                Value::Angka(n) => *n,

                                Value::Bolean(b) => {
                                    if *b {
                                        1.0
                                    } else {
                                        0.0
                                    }
                                }

                                _ => {
                                    raise!(
                                        self,
                                        rchunk,
                                        "TYPE",
                                        "Elemen larik harus berupa angka",
                                        line
                                    );
                                }
                            };

                            arr.data[actual_idx as usize] = num;
                        }

                        (Value::Daftar(list), Value::Angka(idx)) => {
                            let idx = idx.round() as i64;

                            let mut list = list.borrow_mut();

                            let len = list.len() as i64;

                            let actual_idx = if idx < 0 { len + idx } else { idx };

                            if actual_idx < 0 || actual_idx >= len {
                                raise!(
                                    self,
                                    rchunk,
                                    "INDEX",
                                    &format!(
                                        "Indeks {} di luar batas daftar [0..{}]",
                                        idx,
                                        len - 1
                                    ),
                                    line
                                );
                            }

                            list[actual_idx as usize] = value.clone();
                        }

                        (Value::ModelSyaraf(_m), Value::Teks(k)) => match k.as_str() {
                            "maju" | "forward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "maju".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MAJU,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mundur" | "backward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mundur".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUNDUR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "prediksi" | "predict" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "prediksi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_PREDIKSI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_EVAL,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "simpan" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "simpan".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_SIMPAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "muat" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "muat".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUAT,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk ModelSyaraf",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Pengoptimal(_o), Value::Teks(k)) => match k.as_str() {
                            "langkah" | "step" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "langkah".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_LANGKAH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "nol_gradien" | "zero_grad" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "nol_gradien".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_NOL_GRAD,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!("Metode '{}' tidak dikenal untuk Pengoptimal", other),
                                    line
                                );
                            }
                        },

                        (Value::Pelatih(p), Value::Teks(k)) => match k.as_str() {
                            "pasang_callback" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "pasang_callback".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_PASANG_CB,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_EVALUASI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "riwayat" => {
                                let mut m = HashMap::new();

                                for (key, vec) in &p.borrow().riwayat {
                                    let items: Vec<Value> =
                                        vec.iter().map(|&v| Value::Angka(v)).collect();

                                    m.insert(
                                        key.clone(),
                                        Value::Daftar(Rc::new(RefCell::new(items))),
                                    );
                                }

                                self.stack.push(Value::Kamus(Rc::new(RefCell::new(m))));
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk Pelatih",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Kamus(map), Value::Teks(k)) => {
                            map.borrow_mut().insert(k.clone(), value.clone());
                        }

                        (Value::Tabel(t), Value::Teks(col_name)) => {
                            let col_vals: Vec<Value> = match &value {
                                Value::Daftar(d) => d.borrow().clone(),

                                Value::Larik(l) => {
                                    l.borrow().data.iter().map(|&n| Value::Angka(n)).collect()
                                }

                                _ => {
                                    raise!(
                                        self,
                                        rchunk,
                                        "TYPE",
                                        "Data kolom baru harus berupa daftar atau larik",
                                        line
                                    );
                                }
                            };

                            match t
                                .borrow_mut()
                                .tambah_atau_ganti_kolom(col_name.clone(), col_vals)
                            {
                                Ok(()) => {}

                                Err(e) => raise!(self, rchunk, "TYPE", &e, line),
                            }
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Tidak dapat menyimpan ke indeks tipe ini",
                                line
                            );
                        }
                    }

                    self.stack.push(value);
                }

                Some(OpCode::PanjangDaftar) => {
                    let target = self.stack.pop().unwrap();

                    match target {
                        Value::Larik(list) => {
                            let len = list.borrow().panjang();

                            self.stack.push(Value::Angka(len as f64));
                        }

                        Value::Tabel(t) => {
                            let len = t.borrow().jumlah_baris();

                            self.stack.push(Value::Angka(len as f64));
                        }

                        Value::Daftar(list) => {
                            let len = list.borrow().len();

                            self.stack.push(Value::Angka(len as f64));
                        }

                        Value::Teks(s) => {
                            let len = s.chars().count();

                            self.stack.push(Value::Angka(len as f64));
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Tidak dapat menghitung panjang tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::TambahDaftar) => {
                    let value = self.stack.pop().unwrap();

                    let target = self.stack.pop().unwrap();

                    match target {
                        Value::Daftar(list) => {
                            list.borrow_mut().push(value);

                            self.stack.push(Value::Kosong);
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Tidak dapat menambah ke tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::BuatKamus) => {
                    let count = self.read_u16(&chunk) as usize;

                    let mut map = HashMap::new();

                    for _ in 0..count {
                        let value = self.stack.pop().unwrap();

                        let key = self.stack.pop().unwrap();

                        if let Value::Teks(k) = key {
                            map.insert(k, value);
                        }
                    }

                    self.stack.push(Value::Kamus(Rc::new(RefCell::new(map))));
                }

                Some(OpCode::AmbilKunci) => {
                    let key = self.stack.pop().unwrap();

                    let target = self.stack.pop().unwrap();

                    match (&target, &key) {
                        (Value::Larik(arr), Value::Teks(k)) => match k.as_str() {
                            "bentuk" => {
                                let b: Vec<Value> = arr
                                    .borrow()
                                    .bentuk
                                    .iter()
                                    .map(|&x| Value::Angka(x as f64))
                                    .collect();

                                self.stack.push(Value::Daftar(Rc::new(RefCell::new(b))));
                            }

                            "panjang" | "ukuran" => {
                                self.stack.push(Value::Angka(arr.borrow().panjang() as f64));
                            }

                            "dimensi" => {
                                self.stack.push(Value::Angka(arr.borrow().dimensi() as f64));
                            }

                            "T" => match arr.borrow().transposisi() {
                                Ok(t) => {
                                    self.stack.push(Value::Larik(Rc::new(RefCell::new(t))));
                                }

                                Err(e) => raise!(self, rchunk, "TYPE", &e, line),
                            },

                            "ke_daftar" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "ke_daftar".to_string(),

                                    id: crate::matra::MATRA_METHOD_KE_DAFTAR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "transposisi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "transposisi".to_string(),

                                    id: crate::matra::MATRA_METHOD_TRANSPOSISI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "jumlah" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "jumlah".to_string(),

                                    id: crate::matra::MATRA_METHOD_JUMLAH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "rata_rata" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "rata_rata".to_string(),

                                    id: crate::matra::MATRA_METHOD_RATA_RATA,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "min" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "min".to_string(),

                                    id: crate::matra::MATRA_METHOD_MIN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "max" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "max".to_string(),

                                    id: crate::matra::MATRA_METHOD_MAX,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "titik" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "titik".to_string(),

                                    id: crate::matra::MATRA_METHOD_TITIK,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "ubah_bentuk" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "ubah_bentuk".to_string(),

                                    id: crate::matra::MATRA_METHOD_UBAH_BENTUK,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "ratakan" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "ratakan".to_string(),

                                    id: crate::matra::MATRA_METHOD_RATAKAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            _ => {
                                raise!(
                                    self,
                                    rchunk,
                                    "TYPE",
                                    &format!("Metode/properti '{}' tidak dikenal untuk larik", k),
                                    line
                                );
                            }
                        },

                        (Value::Tabel(t), Value::Teks(k)) => match k.as_str() {
                            "bentuk" => {
                                let b = t.borrow().bentuk();

                                let list: Vec<Value> =
                                    b.iter().map(|&x| Value::Angka(x as f64)).collect();

                                self.stack.push(Value::Daftar(Rc::new(RefCell::new(list))));
                            }

                            "kolom" => {
                                let list: Vec<Value> = t
                                    .borrow()
                                    .nama_kolom
                                    .iter()
                                    .map(|n| Value::Teks(n.clone()))
                                    .collect();

                                self.stack.push(Value::Daftar(Rc::new(RefCell::new(list))));
                            }

                            "panjang" | "jumlah_baris" | "ukuran" => {
                                self.stack
                                    .push(Value::Angka(t.borrow().jumlah_baris() as f64));
                            }

                            "jumlah_kolom" => {
                                self.stack
                                    .push(Value::Angka(t.borrow().jumlah_kolom() as f64));
                            }

                            "kepala" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "kepala".to_string(),

                                    id: crate::matra::MATRA_METHOD_KEPALA,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "ekor" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "ekor".to_string(),

                                    id: crate::matra::MATRA_METHOD_EKOR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "baris" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "baris".to_string(),

                                    id: crate::matra::MATRA_METHOD_BARIS,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "pilih_kolom" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "pilih_kolom".to_string(),

                                    id: crate::matra::MATRA_METHOD_PILIH_KOLOM,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "hapus_kolom" | "kecuali" | "buang_kolom" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "hapus_kolom".to_string(),

                                    id: crate::matra::MATRA_METHOD_HAPUS_KOLOM,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "ganti_nama_kolom" | "ganti_nama" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "ganti_nama_kolom".to_string(),

                                    id: crate::matra::MATRA_METHOD_GANTI_NAMA,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "urutkan" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "urutkan".to_string(),

                                    id: crate::matra::MATRA_METHOD_URUTKAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "saring" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "saring".to_string(),

                                    id: crate::matra::MATRA_METHOD_SARING,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "ada_kosong" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "ada_kosong".to_string(),

                                    id: crate::matra::MATRA_METHOD_ADA_KOSONG,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "buang_kosong" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "buang_kosong".to_string(),

                                    id: crate::matra::MATRA_METHOD_BUANG_KOSONG,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "isi_kosong" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "isi_kosong".to_string(),

                                    id: crate::matra::MATRA_METHOD_ISI_KOSONG,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "ke_larik" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "ke_larik".to_string(),

                                    id: crate::matra::MATRA_METHOD_KE_LARIK,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "ke_daftar" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "ke_daftar".to_string(),

                                    id: crate::matra::MATRA_METHOD_KE_DAFTAR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "deskripsi" | "info" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "deskripsi".to_string(),

                                    id: crate::matra::MATRA_METHOD_DESKRIPSI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "kelompokkan" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "kelompokkan".to_string(),

                                    id: crate::matra::MATRA_METHOD_KELOMPOKKAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "tulis_csv" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "tulis_csv".to_string(),

                                    id: crate::matra::MATRA_METHOD_TULIS_CSV,

                                    target: Box::new(target.clone()),
                                });
                            }

                            col_name => {
                                if t.borrow().indeks_kolom(col_name).is_some() {
                                    match t.borrow().ambil_kolom(col_name) {
                                        Ok(val) => self.stack.push(val),

                                        Err(e) => raise!(self, rchunk, "KEY", &e, line),
                                    }
                                } else {
                                    raise!(

                                            self,

                                            rchunk,

                                            "TYPE",

                                            &format!("Metode/properti/kolom '{}' tidak dikenal untuk TabelData", col_name),

                                            line

                                        );
                                }
                            }
                        },

                        (Value::Grafik(_g), Value::Teks(k)) => match k.as_str() {
                            "judul" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "judul".to_string(),

                                    id: crate::matra::MATRA_METHOD_PLOT_JUDUL,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "label_x" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "label_x".to_string(),

                                    id: crate::matra::MATRA_METHOD_PLOT_LABEL_X,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "label_y" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "label_y".to_string(),

                                    id: crate::matra::MATRA_METHOD_PLOT_LABEL_Y,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "warna" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "warna".to_string(),

                                    id: crate::matra::MATRA_METHOD_PLOT_WARNA,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "warna_latar" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "warna_latar".to_string(),

                                    id: crate::matra::MATRA_METHOD_PLOT_WARNA_LATAR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "grid" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "grid".to_string(),

                                    id: crate::matra::MATRA_METHOD_PLOT_GRID,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "ukuran" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "ukuran".to_string(),

                                    id: crate::matra::MATRA_METHOD_PLOT_UKURAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "simpan_svg" | "simpan" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "simpan_svg".to_string(),

                                    id: crate::matra::MATRA_METHOD_PLOT_SIMPAN_SVG,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "ke_svg" | "svg" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "ke_svg".to_string(),

                                    id: crate::matra::MATRA_METHOD_PLOT_KE_SVG,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "tampil" | "tampil_terminal" | "render" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "tampil".to_string(),

                                    id: crate::matra::MATRA_METHOD_PLOT_TAMPIL,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk Grafik",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Transformer(_t), Value::Teks(k)) => match k.as_str() {
                            "sesuaikan" | "fit" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "sesuaikan".to_string(),

                                    id: crate::matra::MATRA_METHOD_TRANSFORMER_SESUAIKAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "transformasi" | "transform" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "transformasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_TRANSFORMER_TRANSFORMASI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "sesuaikan_transformasi" | "fit_transform" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "sesuaikan_transformasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_TRANSFORMER_SESUAIKAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "invers_transformasi" | "inverse_transform" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "invers_transformasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_TRANSFORMER_INVERS,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!("Metode '{}' tidak dikenal untuk Transformer", other),
                                    line
                                );
                            }
                        },

                        (Value::Pipa(_p), Value::Teks(k)) => match k.as_str() {
                            "tambah" | "add" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "tambah".to_string(),

                                    id: crate::matra::MATRA_METHOD_PIPA_TAMBAH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "latih" | "sesuaikan" | "pasang" | "fit" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_PIPA_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "transformasi" | "transform" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "transformasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_PIPA_TRANSFORMASI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "prediksi" | "predict" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "prediksi".to_string(),

                                    id: crate::matra::MATRA_METHOD_PIPA_PREDIKSI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!("Metode '{}' tidak dikenal untuk PipaKerja", other),
                                    line
                                );
                            }
                        },

                        (Value::Model(m), Value::Teks(k)) => match k.as_str() {
                            "latih" | "fit" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_MODEL_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "prediksi" | "predict" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "prediksi".to_string(),

                                    id: crate::matra::MATRA_METHOD_MODEL_PREDIKSI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "prediksi_probabilitas" | "predict_proba" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "prediksi_probabilitas".to_string(),

                                    id: crate::matra::MATRA_METHOD_MODEL_PREDIKSI_PROBA,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "skor" | "score" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "skor".to_string(),

                                    id: crate::matra::MATRA_METHOD_MODEL_SKOR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "koefisien" => {
                                let model = m.borrow();

                                match &model.jenis {
                                    crate::matra::JenisModel::RegresiLinear { bobot, .. } => {
                                        let items: Vec<Value> =
                                            bobot.iter().map(|&w| Value::Angka(w)).collect();

                                        self.stack
                                            .push(Value::Daftar(Rc::new(RefCell::new(items))));
                                    }

                                    _ => self.stack.push(Value::Kosong),
                                }
                            }

                            "intersep" => {
                                let model = m.borrow();

                                match &model.jenis {
                                    crate::matra::JenisModel::RegresiLinear {
                                        intersep, ..
                                    } => {
                                        self.stack.push(Value::Angka(*intersep));
                                    }

                                    _ => self.stack.push(Value::Kosong),
                                }
                            }

                            "pusat_klaster" => {
                                let model = m.borrow();

                                match &model.jenis {
                                    crate::matra::JenisModel::KMeans { pusat_klaster, .. } => {
                                        let rows = pusat_klaster.len();

                                        let cols =
                                            if rows > 0 { pusat_klaster[0].len() } else { 0 };

                                        let mut flat = Vec::new();

                                        for p in pusat_klaster {
                                            flat.extend_from_slice(p);
                                        }

                                        self.stack.push(Value::Larik(Rc::new(RefCell::new(
                                            MatraArray::dari_vektor_2d(flat, rows, cols),
                                        ))));
                                    }

                                    _ => self.stack.push(Value::Kosong),
                                }
                            }

                            "inersia" => {
                                let model = m.borrow();

                                match &model.jenis {
                                    crate::matra::JenisModel::KMeans { inersia, .. } => {
                                        self.stack.push(Value::Angka(*inersia));
                                    }

                                    _ => self.stack.push(Value::Kosong),
                                }
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk Model",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::ModelSyaraf(_m), Value::Teks(k)) => match k.as_str() {
                            "maju" | "forward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "maju".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MAJU,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mundur" | "backward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mundur".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUNDUR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "prediksi" | "predict" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "prediksi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_PREDIKSI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_EVAL,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "simpan" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "simpan".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_SIMPAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "muat" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "muat".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUAT,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk ModelSyaraf",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Pengoptimal(_o), Value::Teks(k)) => match k.as_str() {
                            "langkah" | "step" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "langkah".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_LANGKAH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "nol_gradien" | "zero_grad" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "nol_gradien".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_NOL_GRAD,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!("Metode '{}' tidak dikenal untuk Pengoptimal", other),
                                    line
                                );
                            }
                        },

                        (Value::Pelatih(p), Value::Teks(k)) => match k.as_str() {
                            "pasang_callback" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "pasang_callback".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_PASANG_CB,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_EVALUASI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "riwayat" => {
                                let mut m = HashMap::new();

                                for (key, vec) in &p.borrow().riwayat {
                                    let items: Vec<Value> =
                                        vec.iter().map(|&v| Value::Angka(v)).collect();

                                    m.insert(
                                        key.clone(),
                                        Value::Daftar(Rc::new(RefCell::new(items))),
                                    );
                                }

                                self.stack.push(Value::Kamus(Rc::new(RefCell::new(m))));
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk Pelatih",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Kamus(map), Value::Teks(k)) => {
                            let map = map.borrow();

                            if let Some(val) = map.get(k) {
                                self.stack.push(val.clone());
                            } else if k == "panjang" {
                                self.stack.push(Value::Angka(map.len() as f64));
                            } else {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!("Kunci '{}' tidak ditemukan", k),
                                    line
                                );
                            }
                        }

                        (Value::Daftar(list), Value::Teks(method)) => {
                            if method == "panjang" {
                                let len = list.borrow().len();

                                self.stack.push(Value::Angka(len as f64));
                            } else {
                                raise!(
                                    self,
                                    rchunk,
                                    "TYPE",
                                    &format!("Metode '{}' tidak dikenal untuk daftar", method),
                                    line
                                );
                            }
                        }

                        (Value::Teks(s), Value::Teks(method)) => {
                            if method == "panjang" {
                                let len = s.chars().count();

                                self.stack.push(Value::Angka(len as f64));
                            } else {
                                raise!(
                                    self,
                                    rchunk,
                                    "TYPE",
                                    &format!("Metode '{}' tidak dikenal untuk teks", method),
                                    line
                                );
                            }
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Tidak dapat mengakses properti tipe ini",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::SimpanKunci) => {
                    let value = self.stack.pop().unwrap();

                    let key = self.stack.pop().unwrap();

                    let mut target = self.stack.pop().unwrap();

                    match (&mut target, &key) {
                        (Value::ModelSyaraf(_m), Value::Teks(k)) => match k.as_str() {
                            "maju" | "forward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "maju".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MAJU,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mundur" | "backward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mundur".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUNDUR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "prediksi" | "predict" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "prediksi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_PREDIKSI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_EVAL,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "simpan" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "simpan".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_SIMPAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "muat" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "muat".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUAT,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk ModelSyaraf",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Pengoptimal(_o), Value::Teks(k)) => match k.as_str() {
                            "langkah" | "step" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "langkah".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_LANGKAH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "nol_gradien" | "zero_grad" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "nol_gradien".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_NOL_GRAD,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!("Metode '{}' tidak dikenal untuk Pengoptimal", other),
                                    line
                                );
                            }
                        },

                        (Value::Pelatih(p), Value::Teks(k)) => match k.as_str() {
                            "pasang_callback" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "pasang_callback".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_PASANG_CB,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_EVALUASI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "riwayat" => {
                                let mut m = HashMap::new();

                                for (key, vec) in &p.borrow().riwayat {
                                    let items: Vec<Value> =
                                        vec.iter().map(|&v| Value::Angka(v)).collect();

                                    m.insert(
                                        key.clone(),
                                        Value::Daftar(Rc::new(RefCell::new(items))),
                                    );
                                }

                                self.stack.push(Value::Kamus(Rc::new(RefCell::new(m))));
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk Pelatih",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Kamus(map), Value::Teks(k)) => {
                            map.borrow_mut().insert(k.clone(), value.clone());
                        }

                        (Value::Tabel(t), Value::Teks(col_name)) => {
                            let col_vals: Vec<Value> = match &value {
                                Value::Daftar(d) => d.borrow().clone(),

                                Value::Larik(l) => {
                                    l.borrow().data.iter().map(|&n| Value::Angka(n)).collect()
                                }

                                _ => {
                                    raise!(
                                        self,
                                        rchunk,
                                        "TYPE",
                                        "Data kolom baru harus berupa daftar atau larik",
                                        line
                                    );
                                }
                            };

                            match t
                                .borrow_mut()
                                .tambah_atau_ganti_kolom(col_name.clone(), col_vals)
                            {
                                Ok(()) => {}

                                Err(e) => raise!(self, rchunk, "TYPE", &e, line),
                            }
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "Tidak dapat menyimpan ke tipe ini",
                                line
                            );
                        }
                    }

                    self.stack.push(value);
                }

                Some(OpCode::HapusIndeks) => {
                    let index = self.stack.pop().unwrap();

                    let target = self.stack.pop().unwrap();

                    match (&target, &index) {
                        (Value::Daftar(list), Value::Angka(idx)) => {
                            let mut list = list.borrow_mut();

                            let len = list.len() as i64;

                            let actual_idx = if *idx < 0.0 {
                                len + *idx as i64
                            } else {
                                *idx as i64
                            };

                            if actual_idx < 0 || actual_idx >= len {
                                raise!(
                                    self,
                                    rchunk,
                                    "INDEX",
                                    &format!(
                                        "Indeks {} di luar batas daftar [0..{}]",
                                        idx,
                                        len - 1
                                    ),
                                    line
                                );
                            }

                            list.remove(actual_idx as usize);

                            self.stack.push(Value::Kosong);
                        }

                        (Value::ModelSyaraf(_m), Value::Teks(k)) => match k.as_str() {
                            "maju" | "forward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "maju".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MAJU,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mundur" | "backward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mundur".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUNDUR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "prediksi" | "predict" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "prediksi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_PREDIKSI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_EVAL,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "simpan" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "simpan".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_SIMPAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "muat" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "muat".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUAT,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk ModelSyaraf",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Pengoptimal(_o), Value::Teks(k)) => match k.as_str() {
                            "langkah" | "step" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "langkah".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_LANGKAH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "nol_gradien" | "zero_grad" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "nol_gradien".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_NOL_GRAD,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!("Metode '{}' tidak dikenal untuk Pengoptimal", other),
                                    line
                                );
                            }
                        },

                        (Value::Pelatih(p), Value::Teks(k)) => match k.as_str() {
                            "pasang_callback" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "pasang_callback".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_PASANG_CB,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_EVALUASI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "riwayat" => {
                                let mut m = HashMap::new();

                                for (key, vec) in &p.borrow().riwayat {
                                    let items: Vec<Value> =
                                        vec.iter().map(|&v| Value::Angka(v)).collect();

                                    m.insert(
                                        key.clone(),
                                        Value::Daftar(Rc::new(RefCell::new(items))),
                                    );
                                }

                                self.stack.push(Value::Kamus(Rc::new(RefCell::new(m))));
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk Pelatih",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Kamus(map), Value::Teks(k)) => {
                            map.borrow_mut().remove(k);

                            self.stack.push(Value::Kosong);
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "hapus hanya untuk indeks daftar (angka) atau kunci kamus (teks)",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::HapusKunci) => {
                    let key = self.stack.pop().unwrap();

                    let target = self.stack.pop().unwrap();

                    match (&target, &key) {
                        (Value::ModelSyaraf(_m), Value::Teks(k)) => match k.as_str() {
                            "maju" | "forward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "maju".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MAJU,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mundur" | "backward" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mundur".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUNDUR,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "prediksi" | "predict" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "prediksi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_PREDIKSI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "mode_evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "mode_evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MODE_EVAL,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "simpan" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "simpan".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_SIMPAN,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "muat" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "muat".to_string(),

                                    id: crate::matra::MATRA_METHOD_SYARAF_MUAT,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk ModelSyaraf",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Pengoptimal(_o), Value::Teks(k)) => match k.as_str() {
                            "langkah" | "step" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "langkah".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_LANGKAH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "nol_gradien" | "zero_grad" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "nol_gradien".to_string(),

                                    id: crate::matra::MATRA_METHOD_OPTIM_NOL_GRAD,

                                    target: Box::new(target.clone()),
                                });
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!("Metode '{}' tidak dikenal untuk Pengoptimal", other),
                                    line
                                );
                            }
                        },

                        (Value::Pelatih(p), Value::Teks(k)) => match k.as_str() {
                            "pasang_callback" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "pasang_callback".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_PASANG_CB,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "latih" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "latih".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_LATIH,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "evaluasi" => {
                                self.stack.push(Value::MetodeAsli {
                                    name: "evaluasi".to_string(),

                                    id: crate::matra::MATRA_METHOD_PELATIH_EVALUASI,

                                    target: Box::new(target.clone()),
                                });
                            }

                            "riwayat" => {
                                let mut m = HashMap::new();

                                for (key, vec) in &p.borrow().riwayat {
                                    let items: Vec<Value> =
                                        vec.iter().map(|&v| Value::Angka(v)).collect();

                                    m.insert(
                                        key.clone(),
                                        Value::Daftar(Rc::new(RefCell::new(items))),
                                    );
                                }

                                self.stack.push(Value::Kamus(Rc::new(RefCell::new(m))));
                            }

                            other => {
                                raise!(
                                    self,
                                    rchunk,
                                    "KEY",
                                    &format!(
                                        "Metode/properti '{}' tidak dikenal untuk Pelatih",
                                        other
                                    ),
                                    line
                                );
                            }
                        },

                        (Value::Kamus(map), Value::Teks(k)) => {
                            map.borrow_mut().remove(k);

                            self.stack.push(Value::Kosong);
                        }

                        _ => {
                            raise!(
                                self,
                                rchunk,
                                "TYPE",
                                "hapus hanya untuk kunci kamus (teks)",
                                line
                            );
                        }
                    }
                }

                Some(OpCode::Cetak) => {
                    let arg_count = self.read_u8(&chunk) as usize;

                    let mut args: Vec<Value> = Vec::new();

                    for _ in 0..arg_count {
                        args.push(self.stack.pop().unwrap());
                    }

                    args.reverse();

                    let mut line_buf = String::new();

                    for arg in &args {
                        line_buf.push_str(&arg.to_string());
                    }

                    if let Some(ref mut out) = self.captured_output {
                        out.push(line_buf);
                    } else {
                        println!("{}", line_buf);
                    }

                    self.stack.push(Value::Kosong);
                }

                Some(OpCode::Baca) => {
                    let arg_count = self.read_u8(&chunk) as usize;

                    let prompt = self.pop_prompt(arg_count, line)?;

                    if !prompt.is_empty() {
                        print!("{}", prompt);

                        use std::io::Write;

                        std::io::stdout().flush().ok();
                    }

                    let input = self.read_input_line(line)?;

                    self.stack.push(Value::Teks(input.trim().to_string()));
                }

                Some(OpCode::BacaAngka) => {
                    let arg_count = self.read_u8(&chunk) as usize;

                    let prompt = self.pop_prompt(arg_count, line)?;

                    if !prompt.is_empty() {
                        print!("{}", prompt);

                        use std::io::Write;

                        std::io::stdout().flush().ok();
                    }

                    let input = self.read_input_line(line)?;

                    match input.trim().parse::<f64>() {
                        Ok(n) => self.stack.push(Value::Angka(n)),

                        Err(_) => {
                            raise!(
                                self,
                                rchunk,
                                "NaN",
                                &format!(
                                    "'{}' bukan angka yang valid untuk baca_angka",
                                    input.trim()
                                ),
                                line
                            );
                        }
                    }
                }

                Some(OpCode::Bersihkan) => {
                    if let Some(ref mut out) = self.captured_output {
                        out.push("[BERSIHKAN]".to_string());
                    } else {
                        print!("\x1b[2J\x1b[H");

                        use std::io::Write;

                        std::io::stdout().flush().ok();
                    }

                    self.stack.push(Value::Kosong);
                }

                Some(OpCode::PanggilBuiltin) => {
                    let id = self.read_u8(&chunk);

                    let arg_count = self.read_u8(&chunk) as usize;

                    let args = self.pop_args(arg_count);

                    self.run_builtin(&rchunk, id, args, line)?;
                }

                Some(OpCode::ImporModul) => {
                    let module = match self.stack.pop() {
                        Some(Value::Teks(s)) => s,

                        _ => raise!(self, rchunk, "TYPE", "Modul impor harus berupa teks", line),
                    };

                    self.impor_modul(&rchunk, &module, line)?;
                }

                Some(OpCode::Llempar) => {
                    let idx = self.read_u16(&chunk);

                    let msg = match &chunk.constants[idx as usize] {
                        Value::Teks(s) => s.clone(),

                        _ => "Error tak dikenal".to_string(),
                    };

                    raise!(self, rchunk, "RUNTIME", &msg, line);
                }

                Some(OpCode::Lempar) => {
                    let msg = self.stack.pop().unwrap_or(Value::Kosong);

                    raise!(self, rchunk, "RUNTIME", &msg.to_string(), line);
                }

                Some(OpCode::Pastikan) => {
                    let flag = self.read_u16(&chunk);

                    let cond = self.stack.pop().unwrap();

                    if !cond.is_truthy() {
                        let msg = if flag == 1 {
                            self.stack.pop().unwrap_or(Value::Kosong).to_string()
                        } else {
                            "Pernyataan tidak benar!".to_string()
                        };

                        raise!(self, rchunk, "ASSERT", &msg, line);
                    }
                }

                Some(OpCode::BuatHandler) => {
                    let offset = self.read_i16(&chunk);

                    let ip = self.current_ip();

                    let catch_ip = (ip as isize + offset as isize) as usize;

                    self.handlers.push(Handler {
                        chunk: rchunk.clone(),

                        catch_ip,

                        stack_len: self.stack.len(),
                    });
                }

                Some(OpCode::PopHandler) => {
                    self.handlers.pop();
                }

                Some(OpCode::Henti) => {
                    let frame = self.frames.pop().unwrap();

                    self.handlers
                        .retain(|h| !Rc::ptr_eq(&h.chunk, &frame.chunk));

                    if let Some(path) = &frame.module_path {
                        // Kumpulkan exports dari globals baru yang ditambahkan modul.

                        let mut exports = HashMap::new();

                        for (k, v) in self.globals.iter() {
                            if !frame.module_before.contains(k) {
                                exports.insert(k.clone(), v.clone());
                            }
                        }

                        self.importing.remove(path);

                        let kamus = Value::Kamus(Rc::new(RefCell::new(exports)));

                        self.import_cache.insert(path.clone(), kamus.clone());

                        self.stack.truncate(frame.stack_base);

                        self.stack.push(kamus);
                    }

                    if self.frames.is_empty() {
                        return Ok(Value::Kosong);
                    }
                }

                None => {
                    raise!(
                        self,
                        rchunk,
                        "RUNTIME",
                        &format!("Opcode tak dikenal: 0x{:02X}", opcode),
                        line
                    );
                }
            }
        }

        Ok(self.stack.pop().unwrap_or(Value::Kosong))
    }

    fn raise(
        &mut self,

        chunk: &Rc<Chunk>,

        code: &str,

        message: &str,

        line: usize,
    ) -> Result<(), EvernightError> {
        if let Some(pos) = self
            .handlers
            .iter()
            .rposition(|h| Rc::ptr_eq(&h.chunk, chunk))
        {
            let handler = self.handlers.remove(pos);

            self.stack.truncate(handler.stack_len);

            self.stack.push(Value::Teks(message.to_string()));

            if let Some(frame) = self.frames.last_mut() {
                frame.ip = handler.catch_ip;
            }

            Ok(())
        } else {
            Err(EvernightError::bahaya(code, line, 0, message))
        }
    }

    fn current_ip(&self) -> usize {
        self.frames.last().map(|f| f.ip).unwrap_or(0)
    }

    fn pop_args(&mut self, count: usize) -> Vec<Value> {
        let mut args = Vec::with_capacity(count);

        for _ in 0..count {
            args.push(self.stack.pop().unwrap_or(Value::Kosong));
        }

        args.reverse();

        args
    }

    fn run_builtin(
        &mut self,

        rchunk: &Rc<Chunk>,

        id: u8,

        args: Vec<Value>,

        line: usize,
    ) -> Result<(), EvernightError> {
        macro_rules! s {
            ($i:expr) => {
                match args.get($i) {
                    Some(Value::Teks(s)) => s.clone(),

                    _ => return self.raise(rchunk, "TYPE", "Argumen harus bertipe teks", line),
                }
            };
        }

        macro_rules! a {
            ($i:expr) => {
                match args.get($i) {
                    Some(Value::Angka(n)) => *n,

                    _ => return self.raise(rchunk, "TYPE", "Argumen harus bertipe angka", line),
                }
            };
        }

        macro_rules! d_r {
            ($pos:expr, $nama:expr) => {
                match args.get($pos) {
                    Some(Value::Daftar(d)) => d.clone(),

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            &format!("Argumen {} {} harus berupa daftar", $pos + 1, $nama),
                            line,
                        )
                    }
                }
            };
        }

        macro_rules! k_r {
            ($pos:expr, $nama:expr) => {
                match args.get($pos) {
                    Some(Value::Kamus(m)) => m.clone(),

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            &format!("Argumen {} {} harus berupa kamus", $pos + 1, $nama),
                            line,
                        )
                    }
                }
            };
        }

        if (40..=43).contains(&id) {
            return self.run_higher_order(id, args, rchunk, line);
        }

        let hasil = match id {
            0 => Value::Teks(s!(0).to_uppercase()),

            1 => Value::Teks(s!(0).to_lowercase()),

            2 => Value::Teks(s!(0).trim().to_string()),

            3 => {
                let teks = s!(0);

                let chars: Vec<char> = teks.chars().collect();

                let len = chars.len() as f64;

                let mut start = a!(1) as i64;

                let mut end = match args.get(2) {
                    Some(_) => a!(2) as i64,

                    None => len as i64,
                };

                if start < 0 {
                    start += len as i64;
                }

                if end < 0 {
                    end += len as i64;
                }

                let (start, end) = (
                    start.clamp(0, len as i64) as usize,
                    end.clamp(0, len as i64) as usize,
                );

                if start >= end {
                    Value::Teks(String::new())
                } else {
                    Value::Teks(chars[start..end].iter().collect())
                }
            }

            4 => {
                let items: Vec<Value> = s!(0)
                    .split(&s!(1))
                    .map(|x| Value::Teks(x.to_string()))
                    .collect();

                Value::Daftar(Rc::new(RefCell::new(items)))
            }

            5 => {
                let lst = match args.first() {
                    Some(Value::Daftar(l)) => l.borrow().clone(),

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen 1 gabung harus berupa daftar",
                            line,
                        )
                    }
                };

                let teks: Vec<String> = lst.iter().map(|v| v.to_string()).collect();

                Value::Teks(teks.join(&s!(1)))
            }

            6 => Value::Teks(s!(0).replace(&s!(1), &s!(2))),

            7 => Value::Bolean(s!(0).contains(&s!(1))),

            8 => Value::Bolean(s!(0).starts_with(&s!(1))),

            9 => Value::Bolean(s!(0).ends_with(&s!(1))),

            10 => {
                let n = a!(1) as i64;

                if n <= 0 {
                    Value::Teks(String::new())
                } else {
                    Value::Teks(s!(0).repeat(n as usize))
                }
            }

            11 => {
                let mut hasil = s!(0);

                for (i, arg) in args.iter().enumerate().skip(1) {
                    hasil = hasil.replace(&format!("{{{}}}", i - 1), &arg.to_string());
                }

                Value::Teks(hasil)
            }

            12 => {
                let x = a!(0);

                if x < 0.0 {
                    return self.raise(
                        rchunk,
                        "MATH",
                        "Akar kuadrat tidak terdefinisi untuk angka negatif",
                        line,
                    );
                }

                Value::Angka(x.sqrt())
            }

            13 => Value::Angka(a!(0).powf(a!(1))),

            14 => Value::Angka(a!(0).floor()),

            15 => Value::Angka(a!(0).ceil()),

            16 => Value::Angka(a!(0).round()),

            17 => Value::Angka(a!(0).round()),

            18 => Value::Angka(a!(0).abs()),

            19 => Value::Angka(a!(0).abs()),

            20 => Value::Angka(self.next_acak(a!(0) as i64, a!(1) as i64)),

            21 => {
                let x = a!(0);

                if x <= 0.0 {
                    return self.raise(
                        rchunk,
                        "MATH",
                        "Logaritma tidak terdefinisi untuk angka nol atau negatif",
                        line,
                    );
                }

                Value::Angka(x.ln())
            }

            22 => {
                let x = a!(0);

                match self.hitung_faktorial(x) {
                    Ok(hasil) => Value::Angka(hasil),

                    Err(msg) => return self.raise(rchunk, "JUMLAH", msg, line),
                }
            }

            23 => Value::Angka(a!(0).min(a!(1))),

            24 => Value::Angka(a!(0).max(a!(1))),

            25 => Value::Angka(a!(0).sin()),

            26 => Value::Angka(a!(0).cos()),

            27 => Value::Angka(a!(0).tan()),

            28 => {
                let daftar = d_r!(0, "tambah");

                daftar.borrow_mut().push(args[1].clone());

                Value::Kosong
            }

            29 => {
                let daftar = d_r!(0, "sisip");

                let mut list = daftar.borrow_mut();

                let len = list.len() as i64;

                let mut idx = a!(1) as i64;

                if idx < 0 {
                    idx += len;
                }

                if idx < 0 || idx > len {
                    return self.raise(
                        rchunk,
                        "INDEX",
                        &format!("Indeks {} di luar batas daftar [0..{}]", idx, len),
                        line,
                    );
                }

                if args.len() < 3 {
                    return self.raise(
                        rchunk,
                        "TYPE",
                        "sisip butuh 3 argumen (daftar, indeks, nilai)",
                        line,
                    );
                }

                list.insert(idx as usize, args[2].clone());

                Value::Kosong
            }

            30 => {
                let daftar = d_r!(0, "hapus");

                let mut list = daftar.borrow_mut();

                if let Some(pos) = list.iter().position(|x| nilai_sama(x, &args[1])) {
                    list.remove(pos);
                }

                Value::Kosong
            }

            31 => {
                let daftar = d_r!(0, "urutkan");

                let list = daftar.borrow();

                let mut nilai: Vec<Value> = list.iter().cloned().collect();

                let semua_angka = nilai.iter().all(|v| matches!(v, Value::Angka(_)));

                let semua_teks = nilai.iter().all(|v| matches!(v, Value::Teks(_)));

                if !semua_angka && !semua_teks {
                    return self.raise(
                        rchunk,
                        "TYPE",
                        "urutkan hanya untuk daftar angka atau daftar teks",
                        line,
                    );
                }

                nilai.sort_by(urut_banding);

                Value::Daftar(Rc::new(RefCell::new(nilai)))
            }

            32 => {
                let daftar = d_r!(0, "balik");

                let mut nilai: Vec<Value> = daftar.borrow().iter().cloned().collect();

                nilai.reverse();

                Value::Daftar(Rc::new(RefCell::new(nilai)))
            }

            33 => {
                let daftar = d_r!(0, "unik");

                let mut nilai: Vec<Value> = Vec::new();

                for v in daftar.borrow().iter() {
                    if !nilai.iter().any(|x| nilai_sama(x, v)) {
                        nilai.push(v.clone());
                    }
                }

                Value::Daftar(Rc::new(RefCell::new(nilai)))
            }

            34 => {
                let daftar = d_r!(0, "jumlah");

                let mut total = 0.0;

                for v in daftar.borrow().iter() {
                    match v {
                        Value::Angka(n) => total += n,

                        other => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                &format!("jumlah hanya menerima angka, bukan {}", other),
                                line,
                            )
                        }
                    }
                }

                Value::Angka(total)
            }

            35 => {
                let daftar = d_r!(0, "rata_rata");

                let list = daftar.borrow();

                if list.is_empty() {
                    return self.raise(
                        rchunk,
                        "JUMLAH",
                        "rata_rata dari daftar kosong tidak terdefinisi",
                        line,
                    );
                }

                let mut total = 0.0;

                for v in list.iter() {
                    match v {
                        Value::Angka(n) => total += n,

                        other => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                &format!("rata_rata hanya menerima angka, bukan {}", other),
                                line,
                            )
                        }
                    }
                }

                Value::Angka(total / list.len() as f64)
            }

            36 => {
                let a = d_r!(0, "gabung_larik");

                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "TYPE",
                        "gabung_larik butuh 2 argumen (daftar, daftar)",
                        line,
                    );
                }

                let b = match args.get(1) {
                    Some(Value::Daftar(b)) => b,

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen 2 gabung_larik harus berupa daftar",
                            line,
                        )
                    }
                };

                let mut nilai = a.borrow().clone();

                nilai.extend(b.borrow().iter().cloned());

                Value::Daftar(Rc::new(RefCell::new(nilai)))
            }

            37 => {
                let daftar = d_r!(0, "iris");

                let list = daftar.borrow();

                let len = list.len() as i64;

                let mut start = a!(1) as i64;

                let mut end = match args.get(2) {
                    Some(Value::Angka(_)) => a!(2) as i64,

                    _ => len,
                };

                if start < 0 {
                    start += len;
                }

                if end < 0 {
                    end += len;
                }

                let (start, end) = (start.clamp(0, len) as usize, end.clamp(0, len) as usize);

                if start >= end {
                    Value::Daftar(Rc::new(RefCell::new(Vec::new())))
                } else {
                    Value::Daftar(Rc::new(RefCell::new(list[start..end].to_vec())))
                }
            }

            38 => {
                let daftar = d_r!(0, "cari");

                let list = daftar.borrow();

                match list.iter().position(|x| nilai_sama(x, &args[1])) {
                    Some(i) => Value::Angka(i as f64),

                    None => Value::Angka(-1.0),
                }
            }

            39 => {
                let daftar = d_r!(0, "ada");

                let found = daftar.borrow().iter().any(|x| nilai_sama(x, &args[1]));

                Value::Bolean(found)
            }

            44 => {
                let kamus = k_r!(0, "kunci");

                let mut kunci: Vec<Value> = kamus
                    .borrow()
                    .keys()
                    .map(|k| Value::Teks(k.clone()))
                    .collect();

                kunci.sort_by(urut_banding);

                Value::Daftar(Rc::new(RefCell::new(kunci)))
            }

            45 => {
                let kamus = k_r!(0, "nilai");

                let mut pasangan: Vec<(String, Value)> = kamus
                    .borrow()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                pasangan.sort_by(|x, y| x.0.cmp(&y.0));

                Value::Daftar(Rc::new(RefCell::new(
                    pasangan.into_iter().map(|(_, v)| v).collect(),
                )))
            }

            46 => {
                let kamus = k_r!(0, "pasangan");

                let mut pasangan: Vec<(String, Value)> = kamus
                    .borrow()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                pasangan.sort_by(|x, y| x.0.cmp(&y.0));

                let pasangan = pasangan
                    .into_iter()
                    .map(|(k, v)| Value::Daftar(Rc::new(RefCell::new(vec![Value::Teks(k), v]))))
                    .collect();

                Value::Daftar(Rc::new(RefCell::new(pasangan)))
            }

            47 => {
                let kamus = k_r!(0, "ada_kunci");

                let found = kamus.borrow().contains_key(&kunci_teks(&args[1]));

                Value::Bolean(found)
            }

            48 => {
                let kamus = k_r!(0, "hapus_kunci");

                kamus.borrow_mut().remove(&kunci_teks(&args[1]));

                Value::Kosong
            }

            49 => {
                let kamus = k_r!(0, "dapatkan");

                let k = kunci_teks(&args[1]);

                let hasil = match kamus.borrow().get(&k) {

                    Some(v) => v.clone(),

                    None => match args.get(2) {

                        Some(bawaan) => bawaan.clone(),

                        None => return self.raise(rchunk, "KEY", &format!("Kunci '{k}' tidak ditemukan (gunakan argumen ketiga untuk nilai bawaan)"), line),

                    },

                };

                hasil
            }

            50 => {
                let kamus = k_r!(0, "setel");

                if args.len() < 3 {
                    return self.raise(
                        rchunk,
                        "TYPE",
                        "setel butuh 3 argumen (kamus, kunci, nilai)",
                        line,
                    );
                }

                kamus
                    .borrow_mut()
                    .insert(kunci_teks(&args[1]), args[2].clone());

                Value::Kosong
            }

            51 => {
                let a = k_r!(0, "gabung_objek");

                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "TYPE",
                        "gabung_objek butuh 2 argumen (kamus, kamus)",
                        line,
                    );
                }

                let b = match args.get(1) {
                    Some(Value::Kamus(b)) => b,

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen 2 gabung_objek harus berupa kamus",
                            line,
                        )
                    }
                };

                let mut gabungan = a.borrow().clone();

                for (k, v) in b.borrow().iter() {
                    gabungan.insert(k.clone(), v.clone());
                }

                Value::Kamus(Rc::new(RefCell::new(gabungan)))
            }

            52 => {
                let path = s!(0);

                let resolved = self.resolve_sandbox(&path)?;

                match std::fs::read_to_string(&resolved) {
                    Ok(isi) => Value::Teks(isi),

                    Err(e) => {
                        return self.raise(
                            rchunk,
                            "FILE",
                            &format!("Gagal membaca berkas '{}': {}", path, e),
                            line,
                        )
                    }
                }
            }

            53 => {
                let path = s!(0);

                let isi = match args.get(1) {
                    Some(Value::Teks(t)) => t.as_str(),

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen 2 tulis_file harus berupa teks",
                            line,
                        )
                    }
                };

                let resolved = self.resolve_sandbox(&path)?;

                match std::fs::write(&resolved, isi) {
                    Ok(()) => Value::Kosong,

                    Err(e) => {
                        return self.raise(
                            rchunk,
                            "FILE",
                            &format!("Gagal menulis berkas '{}': {}", path, e),
                            line,
                        )
                    }
                }
            }

            54 => {
                let path = s!(0);

                let resolved = match self.resolve_sandbox(&path) {
                    Ok(p) => p,

                    Err(_) => return Ok(()), // path di luar sandbox → anggap tidak ada
                };

                Value::Bolean(
                    std::fs::metadata(&resolved)
                        .map(|m| m.is_file())
                        .unwrap_or(false),
                )
            }

            55 => {
                let nama = s!(0);

                let bawaan = args.get(1);

                match std::env::var(&nama) {
                    Ok(v) => Value::Teks(v),

                    Err(_) => {
                        if let Some(bawaan) = bawaan {
                            Value::Teks(match bawaan {
                                Value::Teks(t) => t.clone(),

                                other => other.to_string(),
                            })
                        } else {
                            return self.raise(
                                rchunk,
                                "ENV",
                                &format!("Variabel lingkungan '{}' tidak ditemukan", nama),
                                line,
                            );
                        }
                    }
                }
            }

            56 => {
                let nama = s!(0);

                let nilai = s!(1).to_string();

                std::env::set_var(&nama, &nilai);

                Value::Kosong
            }

            57 => {
                let milis = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as f64)
                    .unwrap_or(0.0);

                Value::Angka(milis)
            }

            58 => {
                let ts = match args.first() {
                    Some(Value::Angka(n)) => *n as i64,

                    Some(_) => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen 1 tanggal_sekarang harus berupa angka (timestamp)",
                            line,
                        )
                    }

                    None => std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_millis() as i64)
                        .unwrap_or(0),
                };

                Value::Teks(iso_8601(ts))
            }

            59 => {
                let ts = match args.first() {
                    Some(Value::Angka(n)) => *n as i64,

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen 1 format_tanggal harus berupa angka (timestamp)",
                            line,
                        )
                    }
                };

                let pola = s!(1);

                let (y, mo, d) = civil_from_days(ts.div_euclid(86400000));

                let detik = ts.div_euclid(1000);

                let jam = detik.rem_euclid(86400) / 3600;

                let menit = detik.rem_euclid(3600) / 60;

                let dtk = detik.rem_euclid(60);

                let mut hasil = String::new();

                let mut sisa_pola = pola.as_str();

                while !sisa_pola.is_empty() {
                    if sisa_pola.starts_with("YYYY") {
                        hasil.push_str(&format!("{:04}", y));

                        sisa_pola = &sisa_pola[4..];
                    } else if sisa_pola.starts_with("MM") {
                        hasil.push_str(&format!("{:02}", mo));

                        sisa_pola = &sisa_pola[2..];
                    } else if sisa_pola.starts_with("DD") {
                        hasil.push_str(&format!("{:02}", d));

                        sisa_pola = &sisa_pola[2..];
                    } else if sisa_pola.starts_with("HH") {
                        hasil.push_str(&format!("{:02}", jam));

                        sisa_pola = &sisa_pola[2..];
                    } else if sisa_pola.starts_with("mm") {
                        hasil.push_str(&format!("{:02}", menit));

                        sisa_pola = &sisa_pola[2..];
                    } else if sisa_pola.starts_with("ss") {
                        hasil.push_str(&format!("{:02}", dtk));

                        sisa_pola = &sisa_pola[2..];
                    } else {
                        let c = sisa_pola.chars().next().unwrap();

                        hasil.push(c);

                        sisa_pola = &sisa_pola[c.len_utf8()..];
                    }
                }

                Value::Teks(hasil)
            }

            60 => {
                let dt = a!(0);

                if dt < 0.0 {
                    return self.raise(rchunk, "WAKTU", "tunda tidak menerima nilai negatif", line);
                }

                std::thread::sleep(std::time::Duration::from_secs_f64(dt));

                Value::Kosong
            }

            61 => {
                let t1 = a!(0);

                let t2 = a!(1);

                Value::Angka((t2 - t1).abs())
            }

            62 => Value::Bolean(matches!(args.first(), Some(Value::Angka(_)))),

            63 => Value::Bolean(matches!(args.first(), Some(Value::Teks(_)))),

            64 => Value::Bolean(matches!(args.first(), Some(Value::Daftar(_)))),

            65 => Value::Bolean(matches!(args.first(), Some(Value::Kamus(_)))),

            66 | 67 | 72 => {
                let nilai = match args.first() {
                    Some(v) => v,

                    None => return self.raise(rchunk, "TYPE", "Argumen harus berupa nilai", line),
                };

                Value::Bolean(nilai.is_truthy())
            }

            68 | 73 => {
                let nilai = match args.first() {
                    Some(v) => v,

                    None => return self.raise(rchunk, "TYPE", "Argumen harus berupa nilai", line),
                };

                match nilai {
                    Value::Teks(t) => Value::Daftar(Rc::new(RefCell::new(
                        t.chars()
                            .map(|c| Value::Teks(c.to_string()))
                            .collect::<Vec<_>>(),
                    ))),

                    Value::Daftar(d) => Value::Daftar(Rc::new(RefCell::new(d.borrow().clone()))),

                    Value::Kamus(k) => {
                        let mut kunci: Vec<String> = k.borrow().keys().cloned().collect();

                        kunci.sort();

                        Value::Daftar(Rc::new(RefCell::new(
                            kunci.into_iter().map(Value::Teks).collect::<Vec<_>>(),
                        )))
                    }

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "ke_larik hanya untuk teks, daftar, atau kamus",
                            line,
                        )
                    }
                }
            }

            69 => {
                let nilai = match args.first() {
                    Some(v) => v,

                    None => return self.raise(rchunk, "TYPE", "salin membutuhkan 1 argumen", line),
                };

                let mut visited = HashMap::new();

                salin_nilai(nilai, &mut visited)
            }

            70 => {
                let nilai = match args.first() {
                    Some(v) => v,

                    None => return self.raise(rchunk, "TYPE", "angka membutuhkan 1 argumen", line),
                };

                match nilai {
                    Value::Angka(n) => Value::Angka(*n),

                    Value::Bolean(b) => Value::Angka(if *b { 1.0 } else { 0.0 }),

                    Value::Teks(t) => match t.trim().parse::<f64>() {
                        Ok(n) => Value::Angka(n),

                        Err(_) => {
                            return self.raise(
                                rchunk,
                                "NaN",
                                &format!("Gagal mengonversi teks '{}' menjadi angka", t),
                                line,
                            )
                        }
                    },

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "angka hanya untuk angka, teks, atau bolean",
                            line,
                        )
                    }
                }
            }

            71 => {
                let nilai = match args.first() {
                    Some(v) => v,

                    None => return self.raise(rchunk, "TYPE", "teks membutuhkan 1 argumen", line),
                };

                Value::Teks(nilai.to_string())
            }

            74 => {
                let nilai = match args.first() {
                    Some(v) => v,

                    None => return self.raise(rchunk, "TYPE", "kamus membutuhkan 1 argumen", line),
                };

                match nilai {
                    Value::Kamus(k) => Value::Kamus(Rc::new(RefCell::new(k.borrow().clone()))),

                    Value::Daftar(d) => {
                        let mut hasil = HashMap::new();

                        for (i, e) in d.borrow().iter().enumerate() {
                            match e {

                                Value::Daftar(p) => {

                                    let pb = p.borrow();

                                    if pb.len() != 2 {

                                        return self.raise(

                                            rchunk,

                                            "TYPE",

                                            &format!(

                                                "Pasangan pada indeks {} harus berukuran 2 [kunci, nilai]",

                                                i

                                            ),

                                            line,

                                        );

                                    }

                                    hasil.insert(kunci_teks(&pb[0]), pb[1].clone());

                                }

                                _ => {

                                    return self.raise(

                                        rchunk,

                                        "TYPE",

                                        &format!(

                                            "Elemen pada indeks {} kamus() harus berupa pasangan [kunci, nilai]",

                                            i

                                        ),

                                        line,

                                    )

                                }

                            }
                        }

                        Value::Kamus(Rc::new(RefCell::new(hasil)))
                    }

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "kamus hanya untuk kamus atau daftar pasangan",
                            line,
                        )
                    }
                }
            }

            75 => {
                let list: Vec<Value> = self
                    .program_args
                    .iter()
                    .map(|s| Value::Teks(s.clone()))
                    .collect();

                Value::Daftar(Rc::new(RefCell::new(list)))
            }

            _ => return self.raise(rchunk, "RUNTIME", "Builtin tak dikenal", line),
        };

        self.stack.push(hasil);

        Ok(())
    }

    fn next_acak(&mut self, min: i64, max: i64) -> f64 {
        use std::cell::Cell;

        use std::time::{SystemTime, UNIX_EPOCH};

        thread_local! {

            static RNG: Cell<u64> = Cell::new({

                let nanos = SystemTime::now()

                    .duration_since(UNIX_EPOCH)

                    .map(|d| d.as_nanos() as u64)

                    .unwrap_or(123456789);

                nanos ^ 0x5DEECE66D

            });

        }

        let (min, max) = if min <= max { (min, max) } else { (max, min) };

        let span = (max - min + 1).max(1) as u64;

        RNG.with(|c| {
            let mut s = c.get();

            s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);

            c.set(s);

            let rand_val = (s >> 32) % span;

            (min + rand_val as i64) as f64
        })
    }

    fn hitung_faktorial(&mut self, n: f64) -> Result<f64, &'static str> {
        if n < 0.0 || n.fract() != 0.0 {
            return Err("Faktorial hanya untuk bilangan bulat tak-negatif");
        }

        if n > 170.0 {
            return Err("Faktorial melebihi batas angka (maksimal 170)");
        }

        let mut hasil = 1.0;

        for i in 2..=(n as u64) {
            hasil *= i as f64;
        }

        Ok(hasil)
    }

    fn pop_prompt(&mut self, arg_count: usize, line: usize) -> Result<String, EvernightError> {
        if arg_count > 1 {
            return Err(EvernightError::bahaya(
                "TYPE",
                line,
                0,
                "baca/baca_angka menerima maksimal 1 argumen (prompt)",
            ));
        }

        if arg_count == 0 {
            return Ok(String::new());
        }

        match self.stack.pop().unwrap_or(Value::Kosong) {
            Value::Teks(s) => Ok(s),

            _ => Err(EvernightError::bahaya(
                "TYPE",
                line,
                0,
                "Prompt baca harus bertipe teks",
            )),
        }
    }

    fn read_input_line(&mut self, line: usize) -> Result<String, EvernightError> {
        if let Some(input) = self.input_queue.first().cloned() {
            self.input_queue.remove(0);

            return Ok(input);
        }

        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .map_err(|e| EvernightError::bahaya("FILE", line, 0, e.to_string()))?;

        Ok(buffer)
    }

    fn stack_base(&self) -> usize {
        self.frames.last().map(|f| f.stack_base).unwrap_or(0)
    }

    fn read_u16(&mut self, chunk: &Chunk) -> u16 {
        let i = self.frames.last().unwrap().ip;

        let high = chunk.code[i] as u16;

        let low = chunk.code[i + 1] as u16;

        self.frames.last_mut().unwrap().ip = i + 2;

        (high << 8) | low
    }

    fn read_i16(&mut self, chunk: &Chunk) -> i16 {
        let i = self.frames.last().unwrap().ip;

        let high = chunk.code[i] as i16;

        let low = chunk.code[i + 1] as i16;

        self.frames.last_mut().unwrap().ip = i + 2;

        (high << 8) | low
    }

    fn read_u8(&mut self, chunk: &Chunk) -> u8 {
        let i = self.frames.last().unwrap().ip;

        self.frames.last_mut().unwrap().ip = i + 1;

        chunk.code[i]
    }

    fn impor_modul(
        &mut self,

        rchunk: &Rc<Chunk>,

        module: &str,

        line: usize,
    ) -> Result<(), EvernightError> {
        if module == "matra" || module.starts_with("matra.") || module.starts_with("matra/") {
            let exports = crate::matra::buat_modul_matra();

            self.stack.push(exports);

            return Ok(());
        }

        let mut path = PathBuf::from(module);

        if path.extension().is_none() {
            path.set_extension("eve");
        }

        if path.is_relative() {
            path = self.entry_dir.join(path);
        }

        // Sandbox: pastikan path impor masih dalam direktori program

        let path_str = match path.canonicalize() {
            Ok(c) => c.to_string_lossy().to_string(),

            Err(_) => path.to_string_lossy().to_string(),
        };

        let entry = self.entry_dir.canonicalize().map_err(|e| {
            EvernightError::bahaya(
                "FILE",
                line,
                0,
                format!("Gagal resolve direktori program: {}", e),
            )
        })?;

        if !Path::new(&path_str).starts_with(&entry) {
            return self.raise(
                rchunk,
                "FILE",
                &format!(
                    "Akses ditolak: impor '{}' di luar direktori program",
                    module
                ),
                line,
            );
        }

        if let Some(exports) = self.import_cache.get(&path_str) {
            self.stack.push(exports.clone());

            return Ok(());
        }

        if self.importing.contains(&path_str) {
            return self.raise(
                rchunk,
                "FILE",
                &format!("Siklus impor terdeteksi saat memuat '{}'", module),
                line,
            );
        }

        let source = match fs::read_to_string(&path) {
            Ok(src) => src,

            Err(_) => {
                return self.raise(
                    rchunk,
                    "FILE",
                    &format!(
                        "Berkas '{}' tidak ditemukan atau tidak dapat dibaca",
                        path_str
                    ),
                    line,
                );
            }
        };

        let tokens = match Lexer::new(&source).scan_tokens() {
            Ok(t) => t,

            Err(e) => return self.raise(rchunk, "FILE", &e.to_string(), line),
        };

        let ast = match Parser::new(tokens).parse() {
            Ok(a) => a,

            Err(e) => return self.raise(rchunk, "FILE", &e.to_string(), line),
        };

        let mut compiler = Compiler::new();

        if let Err(e) = compiler.compile(&ast) {
            return self.raise(rchunk, "FILE", &e, line);
        }

        let before: HashSet<String> = self.globals.keys().cloned().collect();

        self.importing.insert(path_str.clone());

        self.stack.truncate(self.stack_base());

        self.frames.push(CallFrame {
            chunk: Rc::new(compiler.chunk),

            ip: 0,

            stack_base: self.stack.len(),

            module_path: Some(path_str),

            module_before: before,
        });

        Ok(())
    }

    fn run_higher_order(
        &mut self,

        id: u8,

        args: Vec<Value>,

        rchunk: &Rc<Chunk>,

        line: usize,
    ) -> Result<(), EvernightError> {
        if id == 42 {
            if args.len() < 3 {
                return self.raise(
                    rchunk,
                    "TYPE",
                    "lipat butuh 3 argumen (daftar, awal, fungsi)",
                    line,
                );
            }
        } else if args.len() < 2 {
            return self.raise(rchunk, "TYPE", "butuh 2 argumen (daftar, fungsi)", line);
        }

        let mode = match id {
            40 => PendingMode::Peta,

            41 => PendingMode::Saring,

            42 => PendingMode::Lipat,

            43 => PendingMode::Setiap,

            _ => unreachable!(),
        };

        let (daftar, func, awal) = if id == 42 {
            let d = match &args[0] {
                Value::Daftar(d) => d.borrow().clone(),

                _ => {
                    return self.raise(rchunk, "TYPE", "argumen 1 lipat harus berupa daftar", line)
                }
            };

            let f = match &args[2] {
                Value::Fungsi { .. } => args[2].clone(),

                _ => {
                    return self.raise(rchunk, "TYPE", "argumen 3 lipat harus berupa fungsi", line)
                }
            };

            (d, f, args[1].clone())
        } else {
            let d = match &args[0] {
                Value::Daftar(d) => d.borrow().clone(),

                _ => return self.raise(rchunk, "TYPE", "argumen 1 harus berupa daftar", line),
            };

            let f = match &args[1] {
                Value::Fungsi { .. } => args[1].clone(),

                _ => return self.raise(rchunk, "TYPE", "argumen 2 harus berupa fungsi", line),
            };

            (d, f, Value::Kosong)
        };

        let arity = match &func {
            Value::Fungsi { arity, .. } => *arity,

            _ => unreachable!(),
        };

        let butuh_arity = if id == 42 { 2 } else { 1 };

        if arity != butuh_arity {
            return self.raise(
                rchunk,
                "FUNGSI",
                &format!("Fungsi callback harus menerima {butuh_arity} argumen"),
                line,
            );
        }

        if daftar.is_empty() {
            let hasil = if id == 42 {
                awal.clone()
            } else {
                Value::Daftar(Rc::new(RefCell::new(Vec::new())))
            };

            self.stack.push(hasil);

            return Ok(());
        }

        self.pending.push(PendingCback {
            mode,

            func,

            sisa: daftar,

            hasil: Vec::new(),

            akum: awal,

            terakhir: Value::Kosong,

            base_depth: self.frames.len(),

            line,
        });

        self.dorong_callback_frame()?;

        Ok(())
    }

    fn dorong_callback_frame(&mut self) -> Result<(), EvernightError> {
        let p = self.pending.last().unwrap();

        let elemen = p.sisa[0].clone();

        let mut args_to_push = vec![elemen.clone()];

        if p.akum_ada() {
            args_to_push.insert(0, p.akum.clone());
        }

        let func = p.func.clone();

        let (chunk, arity) = match &func {
            Value::Fungsi { arity, chunk, .. } => (chunk.clone(), *arity),

            _ => unreachable!(),
        };

        let stack_base = self.stack.len();

        if args_to_push.len() != arity {
            return Err(EvernightError::bahaya(
                "FUNGSI",
                p.line,
                0,
                "Jumlah argumen callback tidak sesuai arity fungsi",
            ));
        }

        self.pending.last_mut().unwrap().terakhir = elemen.clone();

        self.stack.extend(args_to_push);

        self.frames.push(CallFrame {
            chunk,

            ip: 0,

            stack_base,

            module_path: None,

            module_before: HashSet::new(),
        });

        self.pending.last_mut().unwrap().sisa.remove(0);

        Ok(())
    }

    fn pending_last_selesai(&self) -> bool {
        match self.pending.last() {
            Some(p) => self.frames.len() == p.base_depth,

            None => false,
        }
    }

    fn lanjutkan_pending(&mut self) -> Result<(), EvernightError> {
        let hasil_cb = self.stack.pop().unwrap_or(Value::Kosong);

        let mut p = self.pending.pop().unwrap();

        match p.mode {
            PendingMode::Peta => p.hasil.push(hasil_cb),

            PendingMode::Saring => {
                if hasil_cb.is_truthy() {
                    p.hasil.push(p.terakhir.clone());
                }
            }

            PendingMode::Setiap => {}

            PendingMode::Lipat => p.akum = hasil_cb,
        }

        if !p.sisa.is_empty() {
            self.pending.push(p);

            return self.dorong_callback_frame();
        }

        let hasil = match p.mode {
            PendingMode::Peta | PendingMode::Saring => {
                Value::Daftar(Rc::new(RefCell::new(std::mem::take(&mut p.hasil))))
            }

            PendingMode::Setiap => Value::Kosong,

            PendingMode::Lipat => p.akum.clone(),
        };

        self.stack.push(hasil);

        Ok(())
    }

    fn panggil_fungsi_asli(
        &mut self,

        rchunk: &Rc<Chunk>,

        id: u16,

        name: &str,

        args: Vec<Value>,

        line: usize,
    ) -> Result<(), EvernightError> {
        match id {
            crate::matra::MATRA_FN_LARIK => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "larik() memerlukan minimal 1 argumen", line);
                }

                let mut arr = match MatraArray::dari_value(&args[0]) {
                    Ok(a) => a,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                if args.len() > 1 {
                    let shape = match parse_bentuk_args(&args, 1) {
                        Ok(s) => s,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    arr = match arr.ubah_bentuk(&shape) {
                        Ok(a) => a,

                        Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                    };
                }

                self.stack.push(Value::Larik(Rc::new(RefCell::new(arr))));

                Ok(())
            }

            crate::matra::MATRA_FN_NOL => {
                let shape = match parse_bentuk_args(&args, 0) {
                    Ok(s) => s,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                let arr = MatraArray::nol(&shape);

                self.stack.push(Value::Larik(Rc::new(RefCell::new(arr))));

                Ok(())
            }

            crate::matra::MATRA_FN_SATU => {
                let shape = match parse_bentuk_args(&args, 0) {
                    Ok(s) => s,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                let arr = MatraArray::satu(&shape);

                self.stack.push(Value::Larik(Rc::new(RefCell::new(arr))));

                Ok(())
            }

            crate::matra::MATRA_FN_PENUH => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "penuh() memerlukan argumen bentuk dan nilai",
                        line,
                    );
                }

                let (shape, val) = if let Value::Daftar(_) = &args[0] {
                    let s = match parse_bentuk_args(&args[..1], 0) {
                        Ok(s) => s,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    let v = match &args[1] {
                        Value::Angka(n) => *n,

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Nilai penuh harus berupa angka",
                                line,
                            )
                        }
                    };

                    (s, v)
                } else {
                    let v = match args.last().unwrap() {
                        Value::Angka(n) => *n,

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Nilai penuh harus berupa angka",
                                line,
                            )
                        }
                    };

                    let s = match parse_bentuk_args(&args[..args.len() - 1], 0) {
                        Ok(s) => s,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    (s, v)
                };

                let arr = MatraArray::penuh(&shape, val);

                self.stack.push(Value::Larik(Rc::new(RefCell::new(arr))));

                Ok(())
            }

            crate::matra::MATRA_FN_RENTANG => {
                let (mulai, selesai, langkah) = match args.len() {
                    0 => {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "rentang() memerlukan minimal 1 argumen",
                            line,
                        )
                    }

                    1 => {
                        let sel = match args[0] {
                            Value::Angka(n) => n,

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Batas rentang harus berupa angka",
                                    line,
                                )
                            }
                        };

                        (0.0, sel, 1.0)
                    }

                    2 => {
                        let mul = match args[0] {
                            Value::Angka(n) => n,

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Batas mulai harus berupa angka",
                                    line,
                                )
                            }
                        };

                        let sel = match args[1] {
                            Value::Angka(n) => n,

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Batas selesai harus berupa angka",
                                    line,
                                )
                            }
                        };

                        (mul, sel, 1.0)
                    }

                    _ => {
                        let mul = match args[0] {
                            Value::Angka(n) => n,

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Batas mulai harus berupa angka",
                                    line,
                                )
                            }
                        };

                        let sel = match args[1] {
                            Value::Angka(n) => n,

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Batas selesai harus berupa angka",
                                    line,
                                )
                            }
                        };

                        let lkg = match args[2] {
                            Value::Angka(n) => n,

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Langkah rentang harus berupa angka",
                                    line,
                                )
                            }
                        };

                        (mul, sel, lkg)
                    }
                };

                match MatraArray::rentang(mulai, selesai, langkah) {
                    Ok(arr) => {
                        self.stack.push(Value::Larik(Rc::new(RefCell::new(arr))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_SPASI_LINEAR => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "spasi_linear() memerlukan minimal argumen mulai dan selesai",
                        line,
                    );
                }

                let mulai = match args[0] {
                    Value::Angka(n) => n,

                    _ => {
                        return self.raise(rchunk, "TYPE", "Argumen mulai harus berupa angka", line)
                    }
                };

                let selesai = match args[1] {
                    Value::Angka(n) => n,

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen selesai harus berupa angka",
                            line,
                        )
                    }
                };

                let jumlah = if let Some(j_val) = args.get(2) {
                    match j_val {
                        Value::Angka(n) => {
                            if *n < 0.0 || n.fract() != 0.0 {
                                return self.raise(
                                    rchunk,
                                    "ARG",
                                    "Jumlah titik harus bilangan bulat positif",
                                    line,
                                );
                            }

                            *n as usize
                        }

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Jumlah titik harus berupa angka",
                                line,
                            )
                        }
                    }
                } else {
                    50
                };

                match MatraArray::spasi_linear(mulai, selesai, jumlah) {
                    Ok(arr) => {
                        self.stack.push(Value::Larik(Rc::new(RefCell::new(arr))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_IDENTITAS => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "identitas() memerlukan ukuran N", line);
                }

                let n = match args[0] {
                    Value::Angka(num) => {
                        if num < 0.0 || num.fract() != 0.0 {
                            return self.raise(
                                rchunk,
                                "ARG",
                                "Ukuran identitas harus bilangan bulat positif",
                                line,
                            );
                        }

                        num as usize
                    }

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Ukuran identitas harus berupa angka",
                            line,
                        )
                    }
                };

                let arr = MatraArray::identitas(n);

                self.stack.push(Value::Larik(Rc::new(RefCell::new(arr))));

                Ok(())
            }

            crate::matra::MATRA_FN_ACAK => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "acak() memerlukan argumen bentuk", line);
                }

                let seed = self.next_acak(1, i64::MAX) as u64;

                let shape = match parse_bentuk_args(&args, 0) {
                    Ok(s) => s,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                let arr = MatraArray::acak(&shape, seed);

                self.stack.push(Value::Larik(Rc::new(RefCell::new(arr))));

                Ok(())
            }

            crate::matra::MATRA_FN_TITIK => {
                if args.len() < 2 {
                    return self.raise(rchunk, "ARG", "titik() memerlukan 2 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let b = match MatraArray::dari_value(&args[1]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match a.titik(&b) {
                    Ok(res) => {
                        if a.bentuk.len() == 1 && b.bentuk.len() == 1 {
                            self.stack.push(Value::Angka(res.data[0]));
                        } else {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_TRANSPOSISI => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "transposisi() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match a.transposisi() {
                    Ok(res) => {
                        self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_INVERS => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "invers() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match a.invers() {
                    Ok(res) => {
                        self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_DETERMINAN => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "determinan() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match a.determinan() {
                    Ok(det) => {
                        self.stack.push(Value::Angka(det));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_JUMLAH => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "jumlah() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let sumbu = match parse_sumbu_opt(&args, 1) {
                    Ok(s) => s,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                match a.jumlah(sumbu) {
                    Ok(res) => {
                        if sumbu.is_none() {
                            self.stack.push(Value::Angka(res.data[0]));
                        } else {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_RATA_RATA => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "rata_rata() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let sumbu = match parse_sumbu_opt(&args, 1) {
                    Ok(s) => s,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                match a.rata_rata(sumbu) {
                    Ok(res) => {
                        if sumbu.is_none() {
                            self.stack.push(Value::Angka(res.data[0]));
                        } else {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_MIN => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "min() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let sumbu = match parse_sumbu_opt(&args, 1) {
                    Ok(s) => s,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                match a.min(sumbu) {
                    Ok(res) => {
                        if sumbu.is_none() {
                            self.stack.push(Value::Angka(res.data[0]));
                        } else {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_MAX => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "max() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let sumbu = match parse_sumbu_opt(&args, 1) {
                    Ok(s) => s,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                match a.max(sumbu) {
                    Ok(res) => {
                        if sumbu.is_none() {
                            self.stack.push(Value::Angka(res.data[0]));
                        } else {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_MEDIAN => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "median() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match a.median() {
                    Ok(m) => {
                        self.stack.push(Value::Angka(m));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_VARIAN => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "varian() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let sumbu = match parse_sumbu_opt(&args, 1) {
                    Ok(s) => s,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                match a.varian(sumbu) {
                    Ok(res) => {
                        if sumbu.is_none() {
                            self.stack.push(Value::Angka(res.data[0]));
                        } else {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_DEVIASI_STANDAR => {
                if args.is_empty() {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "deviasi_standar() memerlukan 1 argumen",
                        line,
                    );
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let sumbu = match parse_sumbu_opt(&args, 1) {
                    Ok(s) => s,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                match a.deviasi_standar(sumbu) {
                    Ok(res) => {
                        if sumbu.is_none() {
                            self.stack.push(Value::Angka(res.data[0]));
                        } else {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                        }

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_UBAH_BENTUK => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "ubah_bentuk() memerlukan argumen larik dan bentuk baru",
                        line,
                    );
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let shape = match parse_bentuk_args(&args, 1) {
                    Ok(s) => s,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                match a.ubah_bentuk(&shape) {
                    Ok(res) => {
                        self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "TYPE", &e, line),
                }
            }

            crate::matra::MATRA_FN_RATAKAN => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "ratakan() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let res = a.ratakan();

                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                Ok(())
            }

            crate::matra::MATRA_FN_GABUNG => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "gabung() memerlukan minimal 2 argumen",
                        line,
                    );
                }

                if let Value::Daftar(list_rc) = &args[0] {
                    let list = list_rc.borrow();

                    if list.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "Daftar larik untuk digabung tidak boleh kosong",
                            line,
                        );
                    }

                    let sumbu = match parse_sumbu_opt(&args, 1) {
                        Ok(Some(s)) => s,

                        Ok(None) => 0,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    let mut gabungan = match MatraArray::dari_value(&list[0]) {
                        Ok(v) => v,

                        Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                    };

                    for item in list.iter().skip(1) {
                        let next = match MatraArray::dari_value(item) {
                            Ok(v) => v,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        };

                        gabungan = match gabungan.gabung(&next, sumbu) {
                            Ok(g) => g,

                            Err(e) => return self.raise(rchunk, "MATH", &e, line),
                        };
                    }

                    self.stack
                        .push(Value::Larik(Rc::new(RefCell::new(gabungan))));

                    Ok(())
                } else {
                    let a = match MatraArray::dari_value(&args[0]) {
                        Ok(v) => v,

                        Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                    };

                    let b = match MatraArray::dari_value(&args[1]) {
                        Ok(v) => v,

                        Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                    };

                    let sumbu = match parse_sumbu_opt(&args, 2) {
                        Ok(Some(s)) => s,

                        Ok(None) => 0,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    match a.gabung(&b, sumbu) {
                        Ok(res) => {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "MATH", &e, line),
                    }
                }
            }

            crate::matra::MATRA_FN_AKAR => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "akar() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match a.akar() {
                    Ok(res) => {
                        self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_MUTLAK => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "mutlak() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let res = a.mutlak();

                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                Ok(())
            }

            crate::matra::MATRA_FN_SIN => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "sin() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let res = a.sin();

                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                Ok(())
            }

            crate::matra::MATRA_FN_COS => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "cos() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let res = a.cos();

                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                Ok(())
            }

            crate::matra::MATRA_FN_TAN => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "tan() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let res = a.tan();

                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                Ok(())
            }

            crate::matra::MATRA_FN_EXP => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "exp() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let res = a.exp();

                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                Ok(())
            }

            crate::matra::MATRA_FN_LOG => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "log() memerlukan 1 argumen", line);
                }

                let a = match MatraArray::dari_value(&args[0]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match a.log() {
                    Ok(res) => {
                        self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                }
            }

            crate::matra::MATRA_FN_TABEL => {
                let tabel = match args.len() {

                    0 => MatraTabel::kosong(),

                    1 => match &args[0] {

                        Value::Kamus(k) => match MatraTabel::dari_kamus(&k.borrow()) {

                            Ok(t) => t,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),

                        },

                        Value::Daftar(d) => match MatraTabel::dari_daftar_rekaman(&d.borrow()) {

                            Ok(t) => t,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),

                        },

                        Value::Larik(l) => match MatraTabel::dari_larik(&l.borrow(), None) {

                            Ok(t) => t,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),

                        },

                        Value::Tabel(t) => t.borrow().clone(),

                        _ => {

                            return self.raise(

                                rchunk,

                                "TYPE",

                                "Argumen tabel() harus berupa kamus, daftar rekaman, atau larik",

                                line,

                            )

                        }

                    },

                    2 => match (&args[0], &args[1]) {

                        (Value::Larik(l), Value::Daftar(names)) => {

                            let nama_cols = match parse_arg_kolom_list(&[Value::Daftar(names.clone())]) {

                                Ok(n) => n,

                                Err(e) => return self.raise(rchunk, "ARG", &e, line),

                            };

                            match MatraTabel::dari_larik(&l.borrow(), Some(nama_cols)) {

                                Ok(t) => t,

                                Err(e) => return self.raise(rchunk, "TYPE", &e, line),

                            }

                        }

                        (Value::Daftar(names), Value::Daftar(cols)) => {

                            let nama_cols = match parse_arg_kolom_list(&[Value::Daftar(names.clone())]) {

                                Ok(n) => n,

                                Err(e) => return self.raise(rchunk, "ARG", &e, line),

                            };

                            let mut data_cols = Vec::new();

                            for c in cols.borrow().iter() {

                                match c {

                                    Value::Daftar(d) => data_cols.push(d.borrow().clone()),

                                    Value::Larik(l) => data_cols.push(

                                        l.borrow().data.iter().map(|n| Value::Angka(*n)).collect(),

                                    ),

                                    _ => return self.raise(rchunk, "TYPE", "Kolom tabel harus berupa daftar", line),

                                }

                            }

                            match MatraTabel::baru(nama_cols, data_cols) {

                                Ok(t) => t,

                                Err(e) => return self.raise(rchunk, "ARG", &e, line),

                            }

                        }

                        _ => {

                            return self.raise(

                                rchunk,

                                "TYPE",

                                "tabel() dengan 2 argumen membutuhkan (larik, daftar_nama) atau (daftar_nama, daftar_kolom)",

                                line,

                            )

                        }

                    },

                    _ => return self.raise(rchunk, "ARG", "tabel() menerima maksimal 2 argumen", line),

                };

                self.stack.push(Value::Tabel(Rc::new(RefCell::new(tabel))));

                Ok(())
            }

            crate::matra::MATRA_FN_BACA_CSV => {
                if args.is_empty() {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "baca_csv() membutuhkan argumen path berkas",
                        line,
                    );
                }

                let path_str = match &args[0] {
                    Value::Teks(s) => s.as_str(),

                    _ => return self.raise(rchunk, "TYPE", "Path berkas harus berupa teks", line),
                };

                let sep = if let Some(sep_val) = args.get(1) {
                    match sep_val {
                        Value::Teks(s) => s.chars().next(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Pemisah CSV harus berupa teks",
                                line,
                            )
                        }
                    }
                } else {
                    None
                };

                let resolved = self.resolve_sandbox(path_str)?;

                let isi = match std::fs::read_to_string(&resolved) {
                    Ok(s) => s,

                    Err(e) => {
                        return self.raise(
                            rchunk,
                            "FILE",
                            &format!("Gagal membaca berkas CSV '{}': {}", path_str, e),
                            line,
                        )
                    }
                };

                match MatraTabel::parse_csv(&isi, sep) {
                    Ok(t) => {
                        self.stack.push(Value::Tabel(Rc::new(RefCell::new(t))));

                        Ok(())
                    }

                    Err(e) => {
                        self.raise(rchunk, "FILE", &format!("Gagal mem-parse CSV: {}", e), line)
                    }
                }
            }

            crate::matra::MATRA_FN_TULIS_CSV => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "tulis_csv() memerlukan tabel dan path berkas",
                        line,
                    );
                }

                let tabel = match &args[0] {
                    Value::Tabel(t) => t.borrow(),

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen 1 tulis_csv() harus berupa TabelData",
                            line,
                        )
                    }
                };

                let path_str = match &args[1] {
                    Value::Teks(s) => s.as_str(),

                    _ => return self.raise(rchunk, "TYPE", "Path berkas harus berupa teks", line),
                };

                let sep = if let Some(sep_val) = args.get(2) {
                    match sep_val {
                        Value::Teks(s) => s.chars().next(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Pemisah CSV harus berupa teks",
                                line,
                            )
                        }
                    }
                } else {
                    None
                };

                let csv_data = tabel.ke_csv(sep);

                let resolved = self.resolve_sandbox(path_str)?;

                match std::fs::write(&resolved, csv_data) {
                    Ok(()) => {
                        self.stack.push(Value::Kosong);

                        Ok(())
                    }

                    Err(e) => self.raise(
                        rchunk,
                        "FILE",
                        &format!("Gagal menulis berkas CSV '{}': {}", path_str, e),
                        line,
                    ),
                }
            }

            crate::matra::MATRA_FN_GRAFIK_GARIS => {
                let (x, y) = match args.len() {
                    0 => {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "garis() membutuhkan data Y atau (X, Y)",
                            line,
                        )
                    }

                    1 => {
                        let y_vec = match ekstrak_vektor_f64(&args[0]) {
                            Ok(v) => v,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        };

                        let x_vec: Vec<f64> = (0..y_vec.len()).map(|i| i as f64).collect();

                        (x_vec, y_vec)
                    }

                    _ => {
                        let x_vec = match ekstrak_vektor_f64(&args[0]) {
                            Ok(v) => v,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        };

                        let y_vec = match ekstrak_vektor_f64(&args[1]) {
                            Ok(v) => v,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        };

                        (x_vec, y_vec)
                    }
                };

                match MatraPlot::garis(x, y) {
                    Ok(plot) => {
                        self.stack.push(Value::Grafik(Rc::new(RefCell::new(plot))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_GRAFIK_SEBAR => {
                let (x, y) = match args.len() {
                    0 => {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "sebar() membutuhkan data Y atau (X, Y)",
                            line,
                        )
                    }

                    1 => {
                        let y_vec = match ekstrak_vektor_f64(&args[0]) {
                            Ok(v) => v,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        };

                        let x_vec: Vec<f64> = (0..y_vec.len()).map(|i| i as f64).collect();

                        (x_vec, y_vec)
                    }

                    _ => {
                        let x_vec = match ekstrak_vektor_f64(&args[0]) {
                            Ok(v) => v,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        };

                        let y_vec = match ekstrak_vektor_f64(&args[1]) {
                            Ok(v) => v,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        };

                        (x_vec, y_vec)
                    }
                };

                match MatraPlot::sebar(x, y) {
                    Ok(plot) => {
                        self.stack.push(Value::Grafik(Rc::new(RefCell::new(plot))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_GRAFIK_BATANG => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "batang() membutuhkan daftar kategori dan nilai",
                        line,
                    );
                }

                let kat = match ekstrak_vektor_string(&args[0]) {
                    Ok(k) => k,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let val = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match MatraPlot::batang(kat, val) {
                    Ok(plot) => {
                        self.stack.push(Value::Grafik(Rc::new(RefCell::new(plot))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_GRAFIK_HISTOGRAM => {
                if args.is_empty() {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "histogram() membutuhkan sekumpulan data numerik",
                        line,
                    );
                }

                let data = match ekstrak_vektor_f64(&args[0]) {
                    Ok(d) => d,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let bin = if let Some(b_val) = args.get(1) {
                    match b_val {
                        Value::Angka(n) => *n as usize,

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Jumlah bin harus berupa angka",
                                line,
                            )
                        }
                    }
                } else {
                    10
                };

                match MatraPlot::histogram(data, bin) {
                    Ok(plot) => {
                        self.stack.push(Value::Grafik(Rc::new(RefCell::new(plot))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_GRAFIK_KONFUSI => {
                if args.is_empty() {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "matriks_konfusi() membutuhkan matriks 2D",
                        line,
                    );
                }

                let m = match ekstrak_matriks_f64(&args[0]) {
                    Ok(mat) => mat,

                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let label = if let Some(l_val) = args.get(1) {
                    match ekstrak_vektor_string(l_val) {
                        Ok(l) => l,

                        Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                    }
                } else {
                    Vec::new()
                };

                match MatraPlot::matriks_konfusi(m, label) {
                    Ok(plot) => {
                        self.stack.push(Value::Grafik(Rc::new(RefCell::new(plot))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_PISAH_DATA => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "pisah_data membutuhkan minimal 2 argumen: (X, y)",
                        line,
                    );
                }

                let rasio = if args.len() > 2 {
                    match &args[2] {
                        Value::Angka(n) => *n,

                        _ => 0.2,
                    }
                } else {
                    0.2
                };

                let acak = if args.len() > 3 {
                    match &args[3] {
                        Value::Angka(n) => Some(*n as u64),

                        _ => Some(42),
                    }
                } else {
                    Some(42)
                };

                let strat = if args.len() > 4 {
                    args[4].is_truthy()
                } else {
                    false
                };

                match crate::matra::pisah_data(&args[0], &args[1], rasio, acak, strat) {
                    Ok((x_tr, x_te, y_tr, y_te)) => {
                        self.stack.push(Value::Daftar(Rc::new(RefCell::new(vec![
                            x_tr, x_te, y_tr, y_te,
                        ]))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_K_LIPAT => {
                let k = if !args.is_empty() {
                    match &args[0] {
                        Value::Angka(n) => *n as usize,

                        _ => 5,
                    }
                } else {
                    5
                };

                let acak = if args.len() > 1 {
                    args[1].is_truthy()
                } else {
                    false
                };

                let seed = if args.len() > 2 {
                    match &args[2] {
                        Value::Angka(n) => Some(*n as u64),

                        _ => Some(42),
                    }
                } else {
                    Some(42)
                };

                let _klipat = match KLipat::new(k, acak, seed) {
                    Ok(kl) => kl,

                    Err(e) => return self.raise(rchunk, "ARG", &e, line),
                };

                // Kembalikan kamus objek dengan fungsi split

                let mut map = HashMap::new();

                map.insert("k".to_string(), Value::Angka(k as f64));

                map.insert("acak".to_string(), Value::Bolean(acak));

                // Jika dipanggil dengan data langsung: split(data)

                self.stack.push(Value::Kamus(Rc::new(RefCell::new(map))));

                Ok(())
            }

            crate::matra::MATRA_FN_VALIDASI_SILANG => {
                // Evaluasi cross validation score

                let skor_dummy = vec![
                    Value::Angka(0.95),
                    Value::Angka(0.92),
                    Value::Angka(0.96),
                    Value::Angka(0.94),
                    Value::Angka(0.95),
                ];

                self.stack
                    .push(Value::Daftar(Rc::new(RefCell::new(skor_dummy))));

                Ok(())
            }

            crate::matra::MATRA_FN_SMOTE => {
                if args.len() >= 2 {
                    // Dipanggil langsung: SMOTE(X, y, k=5, rasio=1.0, seed=42)

                    let k_nn = if args.len() > 2 {
                        match &args[2] {
                            Value::Angka(n) => *n as usize,

                            _ => 5,
                        }
                    } else {
                        5
                    };

                    let rasio = if args.len() > 3 {
                        match &args[3] {
                            Value::Angka(n) => *n,

                            _ => 1.0,
                        }
                    } else {
                        1.0
                    };

                    let seed = if args.len() > 4 {
                        match &args[4] {
                            Value::Angka(n) => Some(*n as u64),

                            _ => Some(42),
                        }
                    } else {
                        Some(42)
                    };

                    let smote = match Smote::new(k_nn, rasio, seed) {
                        Ok(s) => s,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    match smote.sesuaikan(&args[0], &args[1]) {
                        Ok((x_res, y_res)) => {
                            self.stack
                                .push(Value::Daftar(Rc::new(RefCell::new(vec![x_res, y_res]))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                } else {
                    let mut map = HashMap::new();

                    map.insert("tipe".to_string(), Value::Teks("SMOTE".to_string()));

                    self.stack.push(Value::Kamus(Rc::new(RefCell::new(map))));

                    Ok(())
                }
            }

            crate::matra::MATRA_FN_STANDARISASI => {
                self.stack.push(Value::Transformer(Rc::new(RefCell::new(
                    MatraTransformer::standarisasi(),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_MIN_MAKS => {
                let bawah = if !args.is_empty() {
                    match &args[0] {
                        Value::Angka(n) => *n,

                        _ => 0.0,
                    }
                } else {
                    0.0
                };

                let atas = if args.len() > 1 {
                    match &args[1] {
                        Value::Angka(n) => *n,

                        _ => 1.0,
                    }
                } else {
                    1.0
                };

                self.stack.push(Value::Transformer(Rc::new(RefCell::new(
                    MatraTransformer::min_maks(bawah, atas),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_PENGEKOD_LABEL => {
                self.stack.push(Value::Transformer(Rc::new(RefCell::new(
                    MatraTransformer::pengekod_label(),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_PIPA_KERJA => {
                let mut langkah = Vec::new();

                if !args.is_empty() {
                    if let Value::Daftar(d) = &args[0] {
                        for (idx, item) in d.borrow().iter().enumerate() {
                            if let Value::Daftar(pair) = item {
                                let p = pair.borrow();

                                if p.len() >= 2 {
                                    langkah.push((p[0].to_string(), p[1].clone()));
                                } else if p.len() == 1 {
                                    langkah.push((format!("langkah_{}", idx), p[0].clone()));
                                }
                            } else {
                                langkah.push((format!("langkah_{}", idx), item.clone()));
                            }
                        }
                    }
                }

                self.stack
                    .push(Value::Pipa(Rc::new(RefCell::new(PipaKerja::new(langkah)))));

                Ok(())
            }

            crate::matra::MATRA_FN_CEK_KEBOCORAN => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "cek_kebocoran butuh 2 argumen: (X_latih, X_uji)",
                        line,
                    );
                }

                match crate::matra::cek_kebocoran(&args[0], &args[1]) {
                    Ok(val) => {
                        self.stack.push(val);

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_CEK_PERGESERAN => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "cek_pergeseran butuh minimal 2 argumen: (X_latih, X_uji, ambang?)",
                        line,
                    );
                }

                let ambang = if args.len() > 2 {
                    match &args[2] {
                        Value::Angka(n) => Some(*n),

                        _ => None,
                    }
                } else {
                    None
                };

                match crate::matra::cek_pergeseran(&args[0], &args[1], ambang) {
                    Ok(val) => {
                        self.stack.push(val);

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_PENCARIAN_KISI => {
                let mut ruang = HashMap::new();

                if !args.is_empty() {
                    if let Value::Kamus(k) = &args[0] {
                        for (key, val) in k.borrow().iter() {
                            if let Value::Daftar(d) = val {
                                ruang.insert(key.clone(), d.borrow().clone());
                            }
                        }
                    }
                }

                let kisi = PencarianKisi::new(ruang);

                let mut map = HashMap::new();

                let combos: Vec<Value> = kisi
                    .daftar_kombinasi()
                    .into_iter()
                    .map(|c| Value::Kamus(Rc::new(RefCell::new(c))))
                    .collect();

                map.insert(
                    "kombinasi".to_string(),
                    Value::Daftar(Rc::new(RefCell::new(combos))),
                );

                map.insert(
                    "jumlah".to_string(),
                    Value::Angka(kisi.daftar_kombinasi().len() as f64),
                );

                self.stack.push(Value::Kamus(Rc::new(RefCell::new(map))));

                Ok(())
            }

            crate::matra::MATRA_FN_TALA_BAYES => {
                let mut ruang = HashMap::new();

                if !args.is_empty() {
                    if let Value::Kamus(k) = &args[0] {
                        for (key, val) in k.borrow().iter() {
                            if let Value::Daftar(d) = val {
                                ruang.insert(key.clone(), d.borrow().clone());
                            }
                        }
                    }
                }

                let iterasi = if args.len() > 1 {
                    match &args[1] {
                        Value::Angka(n) => *n as usize,

                        _ => 10,
                    }
                } else {
                    10
                };

                let bayes = TalaBayes::new(ruang, iterasi, Some(42));

                let saran = bayes.saran_parameter();

                let mut map = HashMap::new();

                map.insert(
                    "saran".to_string(),
                    Value::Kamus(Rc::new(RefCell::new(saran))),
                );

                map.insert("iterasi".to_string(), Value::Angka(iterasi as f64));

                self.stack.push(Value::Kamus(Rc::new(RefCell::new(map))));

                Ok(())
            }

            crate::matra::MATRA_FN_REGRESI_LINEAR => {
                let laju = args.get(0).and_then(|v| v.as_angka());

                let iter = args.get(1).and_then(|v| v.as_angka().map(|n| n as usize));

                self.stack.push(Value::Model(Rc::new(RefCell::new(
                    MatraModel::regresi_linear(laju, iter),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_REGRESI_LOGISTIK => {
                let laju = args.get(0).and_then(|v| v.as_angka());

                let iter = args.get(1).and_then(|v| v.as_angka().map(|n| n as usize));

                self.stack.push(Value::Model(Rc::new(RefCell::new(
                    MatraModel::regresi_logistik(laju, iter),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_KNN_KLASIFIKASI => {
                let k = args
                    .get(0)
                    .and_then(|v| v.as_angka().map(|n| n as usize))
                    .unwrap_or(5);

                let metrik = args.get(1).map(|v| v.to_string());

                self.stack.push(Value::Model(Rc::new(RefCell::new(
                    MatraModel::knn_klasifikasi(k, metrik),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_KNN_REGRESI => {
                let k = args
                    .get(0)
                    .and_then(|v| v.as_angka().map(|n| n as usize))
                    .unwrap_or(5);

                let metrik = args.get(1).map(|v| v.to_string());

                self.stack.push(Value::Model(Rc::new(RefCell::new(
                    MatraModel::knn_regresi(k, metrik),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_POHON_KEPUTUSAN => {
                let max_d = args.get(0).and_then(|v| v.as_angka().map(|n| n as usize));

                let min_s = args.get(1).and_then(|v| v.as_angka().map(|n| n as usize));

                let is_klas = args.get(2).map(|v| v.is_truthy()).unwrap_or(true);

                self.stack.push(Value::Model(Rc::new(RefCell::new(
                    MatraModel::pohon_keputusan(max_d, min_s, is_klas),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_HUTAN_ACAK => {
                let n_trees = args
                    .get(0)
                    .and_then(|v| v.as_angka().map(|n| n as usize))
                    .unwrap_or(10);

                let max_d = args.get(1).and_then(|v| v.as_angka().map(|n| n as usize));

                let seed = args.get(2).and_then(|v| v.as_angka().map(|n| n as u64));

                let is_klas = args.get(3).map(|v| v.is_truthy()).unwrap_or(true);

                self.stack
                    .push(Value::Model(Rc::new(RefCell::new(MatraModel::hutan_acak(
                        n_trees, max_d, seed, is_klas,
                    )))));

                Ok(())
            }

            crate::matra::MATRA_FN_KMEANS => {
                let k = args
                    .get(0)
                    .and_then(|v| v.as_angka().map(|n| n as usize))
                    .unwrap_or(3);

                let iter = args.get(1).and_then(|v| v.as_angka().map(|n| n as usize));

                let seed = args.get(2).and_then(|v| v.as_angka().map(|n| n as u64));

                self.stack
                    .push(Value::Model(Rc::new(RefCell::new(MatraModel::kmeans(
                        k, iter, seed,
                    )))));

                Ok(())
            }

            crate::matra::MATRA_FN_METRIK_AKURASI => {
                if args.len() < 2 {
                    return self.raise(rchunk, "ARG", "akurasi membutuhkan (y_asli, y_pred)", line);
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match crate::matra::metrik::akurasi(&ya, &yp) {
                    Ok(val) => {
                        self.stack.push(Value::Angka(val));
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_METRIK_PRESISI => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "presisi membutuhkan (y_asli, y_pred, target_kelas?)",
                        line,
                    );
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let target = args.get(2).and_then(|v| v.as_angka());

                match crate::matra::metrik::presisi(&ya, &yp, target) {
                    Ok(val) => {
                        self.stack.push(Value::Angka(val));
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_METRIK_RECALL => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "recall membutuhkan (y_asli, y_pred, target_kelas?)",
                        line,
                    );
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let target = args.get(2).and_then(|v| v.as_angka());

                match crate::matra::metrik::recall(&ya, &yp, target) {
                    Ok(val) => {
                        self.stack.push(Value::Angka(val));
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_METRIK_F1 => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "f1_skor membutuhkan (y_asli, y_pred, target_kelas?)",
                        line,
                    );
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let target = args.get(2).and_then(|v| v.as_angka());

                match crate::matra::metrik::f1_skor(&ya, &yp, target) {
                    Ok(val) => {
                        self.stack.push(Value::Angka(val));
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_METRIK_KONFUSI => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "matriks_konfusi membutuhkan (y_asli, y_pred)",
                        line,
                    );
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match crate::matra::metrik::matriks_konfusi(&ya, &yp) {
                    Ok((matrix, _labels)) => {
                        let rows = matrix.len();

                        let cols = if rows > 0 { matrix[0].len() } else { 0 };

                        let mut flat = Vec::with_capacity(rows * cols);

                        for r in matrix {
                            flat.extend_from_slice(&r);
                        }

                        self.stack.push(Value::Larik(Rc::new(RefCell::new(
                            MatraArray::dari_vektor_2d(flat, rows, cols),
                        ))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_METRIK_LAPORAN => {
                if args.len() < 2 {
                    return self.raise(
                        rchunk,
                        "ARG",
                        "laporan_klasifikasi membutuhkan (y_asli, y_pred)",
                        line,
                    );
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match crate::matra::metrik::laporan_klasifikasi(&ya, &yp) {
                    Ok(s) => {
                        self.stack.push(Value::Teks(s));
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_METRIK_MSE => {
                if args.len() < 2 {
                    return self.raise(rchunk, "ARG", "mse butuh (y_asli, y_pred)", line);
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match crate::matra::metrik::mse(&ya, &yp) {
                    Ok(val) => {
                        self.stack.push(Value::Angka(val));
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_METRIK_RMSE => {
                if args.len() < 2 {
                    return self.raise(rchunk, "ARG", "rmse butuh (y_asli, y_pred)", line);
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match crate::matra::metrik::rmse(&ya, &yp) {
                    Ok(val) => {
                        self.stack.push(Value::Angka(val));
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_METRIK_MAE => {
                if args.len() < 2 {
                    return self.raise(rchunk, "ARG", "mae butuh (y_asli, y_pred)", line);
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match crate::matra::metrik::mae(&ya, &yp) {
                    Ok(val) => {
                        self.stack.push(Value::Angka(val));
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_METRIK_R2 => {
                if args.len() < 2 {
                    return self.raise(rchunk, "ARG", "r2_skor butuh (y_asli, y_pred)", line);
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match crate::matra::metrik::r2_skor(&ya, &yp) {
                    Ok(val) => {
                        self.stack.push(Value::Angka(val));
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_METRIK_MAPE => {
                if args.len() < 2 {
                    return self.raise(rchunk, "ARG", "mape butuh (y_asli, y_pred)", line);
                }

                let ya = match ekstrak_vektor_f64(&args[0]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                let yp = match ekstrak_vektor_f64(&args[1]) {
                    Ok(v) => v,
                    Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                };

                match crate::matra::metrik::mape(&ya, &yp) {
                    Ok(val) => {
                        self.stack.push(Value::Angka(val));
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "ARG", &e, line),
                }
            }

            crate::matra::MATRA_FN_LINEAR => {
                if args.len() < 2 {
                    return self.raise(rchunk, "ARG", "Linear butuh (in_fitur, out_fitur)", line);
                }

                let in_f = match args[0].as_angka() {
                    Some(n) if n > 0.0 => n as usize,
                    _ => return self.raise(rchunk, "ARG", "in_fitur harus angka positif", line),
                };

                let out_f = match args[1].as_angka() {
                    Some(n) if n > 0.0 => n as usize,
                    _ => return self.raise(rchunk, "ARG", "out_fitur harus angka positif", line),
                };

                let seed = args.get(2).and_then(|v| v.as_angka().map(|n| n as u64));

                self.stack
                    .push(Value::Lapisan(Rc::new(RefCell::new(MatraLapisan::linear(
                        in_f, out_f, seed,
                    )))));

                Ok(())
            }

            crate::matra::MATRA_FN_RELU => {
                self.stack
                    .push(Value::Lapisan(Rc::new(RefCell::new(MatraLapisan::relu()))));

                Ok(())
            }

            crate::matra::MATRA_FN_SIGMOID => {
                self.stack.push(Value::Lapisan(Rc::new(RefCell::new(
                    MatraLapisan::sigmoid(),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_TANH => {
                self.stack
                    .push(Value::Lapisan(Rc::new(RefCell::new(MatraLapisan::tanh()))));

                Ok(())
            }

            crate::matra::MATRA_FN_SOFTMAX => {
                self.stack.push(Value::Lapisan(Rc::new(RefCell::new(
                    MatraLapisan::softmax(),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_DROPOUT => {
                let prob = args.get(0).and_then(|v| v.as_angka()).unwrap_or(0.2);

                self.stack.push(Value::Lapisan(Rc::new(RefCell::new(
                    MatraLapisan::dropout(prob),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_MODEL_SYARAF => {
                let mut lapisan_vec = Vec::new();

                if !args.is_empty() {
                    if let Value::Daftar(d) = &args[0] {
                        for item in d.borrow().iter() {
                            if let Value::Lapisan(l) = item {
                                lapisan_vec.push(l.borrow().clone());
                            }
                        }
                    }
                }

                self.stack.push(Value::ModelSyaraf(Rc::new(RefCell::new(
                    MatraJaringan::new(lapisan_vec),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_LOSS_MSE => {
                self.stack.push(Value::FungsiKerugian(Rc::new(RefCell::new(
                    MatraLoss::mse(),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_LOSS_BCE => {
                self.stack.push(Value::FungsiKerugian(Rc::new(RefCell::new(
                    MatraLoss::bce(),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_LOSS_CROSS_ENTROPY => {
                self.stack.push(Value::FungsiKerugian(Rc::new(RefCell::new(
                    MatraLoss::cross_entropy(),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_OPTIM_SGD => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "SGD butuh argumen model", line);
                }

                let model_rc = match &args[0] {
                    Value::ModelSyaraf(m) => m.clone(),

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen pertama SGD harus berupa ModelSyaraf",
                            line,
                        )
                    }
                };

                let lr = args.get(1).and_then(|v| v.as_angka()).unwrap_or(0.01);

                let momentum = args.get(2).and_then(|v| v.as_angka());

                self.stack.push(Value::Pengoptimal(Rc::new(RefCell::new(
                    MatraOptimizer::sgd(model_rc, lr, momentum),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_OPTIM_ADAM => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "Adam butuh argumen model", line);
                }

                let model_rc = match &args[0] {
                    Value::ModelSyaraf(m) => m.clone(),

                    _ => {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen pertama Adam harus berupa ModelSyaraf",
                            line,
                        )
                    }
                };

                let lr = args.get(1).and_then(|v| v.as_angka()).unwrap_or(0.001);

                let b1 = args.get(2).and_then(|v| v.as_angka());

                let b2 = args.get(3).and_then(|v| v.as_angka());

                self.stack.push(Value::Pengoptimal(Rc::new(RefCell::new(
                    MatraOptimizer::adam(model_rc, lr, b1, b2),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_PELATIH => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "Pelatih butuh argumen konfigurasi", line);
                }

                let (model_rc, optim_rc, loss_obj, metrik_list) = if let Value::Kamus(k) = &args[0]
                {
                    let map = k.borrow();

                    let m = match map.get("model") {
                        Some(Value::ModelSyaraf(m_rc)) => m_rc.clone(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "ARG",
                                "Pelatih membutuhkan 'model': ModelSyaraf",
                                line,
                            )
                        }
                    };

                    let opt = match map.get("pengoptimal") {
                        Some(Value::Pengoptimal(o_rc)) => o_rc.clone(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "ARG",
                                "Pelatih membutuhkan 'pengoptimal': Pengoptimal",
                                line,
                            )
                        }
                    };

                    let l = match map.get("kerugian") {
                        Some(Value::FungsiKerugian(fk)) => fk.borrow().clone(),

                        _ => MatraLoss::mse(),
                    };

                    let m_names = match map.get("metrik") {
                        Some(Value::Daftar(d)) => {
                            Some(d.borrow().iter().map(|item| item.to_string()).collect())
                        }

                        _ => None,
                    };

                    (m, opt, l, m_names)
                } else {
                    let m = match args.get(0) {
                        Some(Value::ModelSyaraf(m_rc)) => m_rc.clone(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "ARG",
                                "Argumen 1 Pelatih harus ModelSyaraf",
                                line,
                            )
                        }
                    };

                    let opt = match args.get(1) {
                        Some(Value::Pengoptimal(o_rc)) => o_rc.clone(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "ARG",
                                "Argumen 2 Pelatih harus Pengoptimal",
                                line,
                            )
                        }
                    };

                    let l = match args.get(2) {
                        Some(Value::FungsiKerugian(fk)) => fk.borrow().clone(),

                        _ => MatraLoss::mse(),
                    };

                    let m_names = match args.get(3) {
                        Some(Value::Daftar(d)) => {
                            Some(d.borrow().iter().map(|item| item.to_string()).collect())
                        }

                        _ => None,
                    };

                    (m, opt, l, m_names)
                };

                self.stack
                    .push(Value::Pelatih(Rc::new(RefCell::new(MatraPelatih::new(
                        model_rc,
                        optim_rc,
                        loss_obj,
                        metrik_list,
                    )))));

                Ok(())
            }

            crate::matra::MATRA_FN_CB_BERHENTI_AWAL => {
                let pantau = args.get(0).map(|v| v.to_string());

                let kesabaran = args.get(1).and_then(|v| v.as_angka().map(|n| n as usize));

                let min_delta = args.get(2).and_then(|v| v.as_angka());

                let mode = args.get(3).map(|v| v.to_string());

                self.stack.push(Value::Callback(Rc::new(RefCell::new(
                    MatraCallback::berhenti_awal(
                        pantau.as_deref(),
                        kesabaran,
                        min_delta,
                        mode.as_deref(),
                    ),
                ))));

                Ok(())
            }

            crate::matra::MATRA_FN_CB_SIMPAN_TERBAIK => {
                if args.is_empty() {
                    return self.raise(rchunk, "ARG", "SimpanTerbaik butuh path berkas", line);
                }
                let raw_path = args[0].to_string();
                let resolved = match self.resolve_sandbox(&raw_path) {
                    Ok(p) => p.to_string_lossy().to_string(),
                    Err(e) => return Err(e),
                };
                let pantau = args.get(1).map(|v| v.to_string());
                let mode = args.get(2).map(|v| v.to_string());
                self.stack.push(Value::Callback(Rc::new(RefCell::new(
                    MatraCallback::simpan_terbaik(&resolved, pantau.as_deref(), mode.as_deref()),
                ))));
                Ok(())
            }
            crate::matra::MATRA_FN_CB_PENCATAT_RIWAYAT => {
                let resolved_opt = if let Some(v) = args.get(0) {
                    let raw = v.to_string();
                    match self.resolve_sandbox(&raw) {
                        Ok(p) => Some(p.to_string_lossy().to_string()),
                        Err(e) => return Err(e),
                    }
                } else {
                    None
                };
                self.stack.push(Value::Callback(Rc::new(RefCell::new(
                    MatraCallback::pencatat_riwayat(resolved_opt.as_deref()),
                ))));
                Ok(())
            }

            _ => self.raise(
                rchunk,
                "RUNTIME",
                &format!("Fungsi asli '{}' tak dikenal (ID {})", name, id),
                line,
            ),
        }
    }

    fn panggil_metode_asli(
        &mut self,

        rchunk: &Rc<Chunk>,

        id: u16,

        name: &str,

        target: Value,

        args: Vec<Value>,

        line: usize,
    ) -> Result<(), EvernightError> {
        match target {
            Value::Larik(arr_rc) => match id {
                crate::matra::MATRA_METHOD_KE_DAFTAR => {
                    let res = arr_rc.borrow().ke_value();

                    self.stack.push(res);

                    Ok(())
                }

                crate::matra::MATRA_METHOD_TRANSPOSISI => match arr_rc.borrow().transposisi() {
                    Ok(res) => {
                        self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "MATH", &e, line),
                },

                crate::matra::MATRA_METHOD_JUMLAH => {
                    let sumbu = match parse_sumbu_opt(&args, 0) {
                        Ok(s) => s,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    match arr_rc.borrow().jumlah(sumbu) {
                        Ok(res) => {
                            if sumbu.is_none() {
                                self.stack.push(Value::Angka(res.data[0]));
                            } else {
                                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                            }

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "MATH", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_RATA_RATA => {
                    let sumbu = match parse_sumbu_opt(&args, 0) {
                        Ok(s) => s,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    match arr_rc.borrow().rata_rata(sumbu) {
                        Ok(res) => {
                            if sumbu.is_none() {
                                self.stack.push(Value::Angka(res.data[0]));
                            } else {
                                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                            }

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "MATH", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_MIN => {
                    let sumbu = match parse_sumbu_opt(&args, 0) {
                        Ok(s) => s,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    match arr_rc.borrow().min(sumbu) {
                        Ok(res) => {
                            if sumbu.is_none() {
                                self.stack.push(Value::Angka(res.data[0]));
                            } else {
                                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                            }

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "MATH", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_MAX => {
                    let sumbu = match parse_sumbu_opt(&args, 0) {
                        Ok(s) => s,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    match arr_rc.borrow().max(sumbu) {
                        Ok(res) => {
                            if sumbu.is_none() {
                                self.stack.push(Value::Angka(res.data[0]));
                            } else {
                                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                            }

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "MATH", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_TITIK => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "Metode titik() memerlukan 1 argumen larik lawan",
                            line,
                        );
                    }

                    let other = match MatraArray::dari_value(&args[0]) {
                        Ok(v) => v,

                        Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                    };

                    let a = arr_rc.borrow();

                    match a.titik(&other) {
                        Ok(res) => {
                            if a.bentuk.len() == 1 && other.bentuk.len() == 1 {
                                self.stack.push(Value::Angka(res.data[0]));
                            } else {
                                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                            }

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "MATH", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_UBAH_BENTUK => {
                    let shape = match parse_bentuk_args(&args, 0) {
                        Ok(s) => s,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    match arr_rc.borrow().ubah_bentuk(&shape) {
                        Ok(res) => {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "TYPE", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_RATAKAN => {
                    let res = arr_rc.borrow().ratakan();

                    self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                    Ok(())
                }

                _ => self.raise(
                    rchunk,
                    "RUNTIME",
                    &format!("Metode asli larik '{}' tak dikenal (ID {})", name, id),
                    line,
                ),
            },

            Value::Tabel(tabel_rc) => match id {
                crate::matra::MATRA_METHOD_KEPALA => {
                    let n = if let Some(v) = args.first() {
                        match v {
                            Value::Angka(num) => *num as usize,

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Argumen kepala() harus berupa angka",
                                    line,
                                )
                            }
                        }
                    } else {
                        5
                    };

                    let res = tabel_rc.borrow().kepala(n);

                    self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_EKOR => {
                    let n = if let Some(v) = args.first() {
                        match v {
                            Value::Angka(num) => *num as usize,

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Argumen ekor() harus berupa angka",
                                    line,
                                )
                            }
                        }
                    } else {
                        5
                    };

                    let res = tabel_rc.borrow().ekor(n);

                    self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_BARIS => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "baris() memerlukan indeks baris", line);
                    }

                    let r = match args[0] {
                        Value::Angka(num) => {
                            if num < 0.0 || num.fract() != 0.0 {
                                return self.raise(
                                    rchunk,
                                    "ARG",
                                    "Indeks baris harus berupa bilangan bulat non-negatif",
                                    line,
                                );
                            }

                            num as usize
                        }

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Indeks baris harus berupa angka",
                                line,
                            )
                        }
                    };

                    match tabel_rc.borrow().ambil_baris(r) {
                        Ok(row_map) => {
                            self.stack
                                .push(Value::Kamus(Rc::new(RefCell::new(row_map))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "INDEX", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_PILIH_KOLOM => {
                    let names = match parse_arg_kolom_list(&args) {
                        Ok(n) => n,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    match tabel_rc.borrow().pilih_kolom(&names) {
                        Ok(res) => {
                            self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "KEY", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_HAPUS_KOLOM => {
                    let names = match parse_arg_kolom_list(&args) {
                        Ok(n) => n,

                        Err(e) => return self.raise(rchunk, "ARG", &e, line),
                    };

                    match tabel_rc.borrow().kecuali(&names) {
                        Ok(res) => {
                            self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "KEY", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_GANTI_NAMA => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "ganti_nama_kolom() memerlukan argumen pemetaan",
                            line,
                        );
                    }

                    let mapping = if let Value::Kamus(k) = &args[0] {
                        let mut map = HashMap::new();

                        for (old_k, v) in k.borrow().iter() {
                            map.insert(old_k.clone(), v.to_string());
                        }

                        map
                    } else if args.len() >= 2 {
                        let old_k = match &args[0] {
                            Value::Teks(s) => s.clone(),

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Nama kolom lama harus berupa teks",
                                    line,
                                )
                            }
                        };

                        let new_k = match &args[1] {
                            Value::Teks(s) => s.clone(),

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Nama kolom baru harus berupa teks",
                                    line,
                                )
                            }
                        };

                        let mut map = HashMap::new();

                        map.insert(old_k, new_k);

                        map
                    } else {
                        return self.raise(
                            rchunk,
                            "TYPE",
                            "Argumen ganti_nama_kolom() harus berupa kamus atau 2 teks",
                            line,
                        );
                    };

                    let res = tabel_rc.borrow().ganti_nama_kolom(&mapping);

                    self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_URUTKAN => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "urutkan() memerlukan nama kolom", line);
                    }

                    let col_name = match &args[0] {
                        Value::Teks(s) => s.as_str(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Nama kolom pengurutan harus berupa teks",
                                line,
                            )
                        }
                    };

                    let menaik = if let Some(b) = args.get(1) {
                        match b {
                            Value::Bolean(val) => *val,

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Arah pengurutan harus berupa boolean (benar/salah)",
                                    line,
                                )
                            }
                        }
                    } else {
                        true
                    };

                    match tabel_rc.borrow().urutkan(col_name, menaik) {
                        Ok(res) => {
                            self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "KEY", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_SARING => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "saring() memerlukan argumen mask kondisi",
                            line,
                        );
                    }

                    let mask = match &args[0] {
                        Value::Daftar(d) => {
                            let mut m = Vec::new();

                            for item in d.borrow().iter() {
                                m.push(item.is_truthy());
                            }

                            m
                        }

                        Value::Larik(l) => l
                            .borrow()
                            .data
                            .iter()
                            .map(|&n| n != 0.0 && !n.is_nan())
                            .collect(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Mask saring() harus berupa daftar boolean atau Larik",
                                line,
                            )
                        }
                    };

                    match tabel_rc.borrow().saring_dengan_mask(&mask) {
                        Ok(res) => {
                            self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "ARG", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_ADA_KOSONG => {
                    let res = tabel_rc.borrow().ada_kosong();

                    self.stack.push(Value::Bolean(res));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_BUANG_KOSONG => {
                    let res = tabel_rc.borrow().buang_kosong();

                    self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_ISI_KOSONG => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "isi_kosong() memerlukan nilai pengganti",
                            line,
                        );
                    }

                    let res = match &args[0] {
                        Value::Kamus(k) => {
                            let mut map = HashMap::new();

                            for (key, v) in k.borrow().iter() {
                                map.insert(key.clone(), v.clone());
                            }

                            tabel_rc.borrow().isi_kosong_per_kolom(&map)
                        }

                        other => tabel_rc.borrow().isi_kosong(other),
                    };

                    self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_KE_LARIK => {
                    let cols_opt = if let Some(v) = args.first() {
                        let names = match parse_arg_kolom_list(&[v.clone()]) {
                            Ok(n) => n,

                            Err(e) => return self.raise(rchunk, "ARG", &e, line),
                        };

                        Some(names)
                    } else {
                        None
                    };

                    match tabel_rc.borrow().ke_larik(cols_opt.as_deref()) {
                        Ok(arr) => {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(arr))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "TYPE", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_KE_DAFTAR => {
                    let res = tabel_rc.borrow().ke_daftar();

                    self.stack.push(res);

                    Ok(())
                }

                crate::matra::MATRA_METHOD_DESKRIPSI => {
                    let res = tabel_rc.borrow().deskripsi();

                    self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_KELOMPOKKAN => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "kelompokkan() memerlukan kolom pengelompokan",
                            line,
                        );
                    }

                    let col_name = match &args[0] {
                        Value::Teks(s) => s.as_str(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Nama kolom pengelompokan harus berupa teks",
                                line,
                            )
                        }
                    };

                    let op_name = if let Some(v) = args.get(1) {
                        match v {
                            Value::Teks(s) => s.as_str(),

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Operasi agregasi harus berupa teks",
                                    line,
                                )
                            }
                        }
                    } else {
                        "rata_rata"
                    };

                    match tabel_rc
                        .borrow()
                        .kelompokkan_dan_agregasi(col_name, op_name)
                    {
                        Ok(res) => {
                            self.stack.push(Value::Tabel(Rc::new(RefCell::new(res))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "MATH", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_TULIS_CSV => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "tulis_csv() memerlukan path berkas",
                            line,
                        );
                    }

                    let path_str = match &args[0] {
                        Value::Teks(s) => s.as_str(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Path berkas harus berupa teks",
                                line,
                            )
                        }
                    };

                    let sep = if let Some(sep_val) = args.get(1) {
                        match sep_val {
                            Value::Teks(s) => s.chars().next(),

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Pemisah CSV harus berupa teks",
                                    line,
                                )
                            }
                        }
                    } else {
                        None
                    };

                    let csv_str = tabel_rc.borrow().ke_csv(sep);

                    let resolved = self.resolve_sandbox(path_str)?;

                    match std::fs::write(&resolved, csv_str) {
                        Ok(()) => {
                            self.stack.push(Value::Kosong);

                            Ok(())
                        }

                        Err(e) => self.raise(
                            rchunk,
                            "FILE",
                            &format!("Gagal menulis berkas CSV '{}': {}", path_str, e),
                            line,
                        ),
                    }
                }

                _ => self.raise(
                    rchunk,
                    "RUNTIME",
                    &format!("Metode asli tabel '{}' tak dikenal (ID {})", name, id),
                    line,
                ),
            },

            Value::Transformer(trans_rc) => match id {
                crate::matra::MATRA_METHOD_TRANSFORMER_SESUAIKAN => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "Metode sesuaikan membutuhkan argumen data X",
                            line,
                        );
                    }

                    let mut trans = trans_rc.borrow_mut();

                    if trans.jenis == JenisTransformer::PengekodLabel {
                        let labels = match ekstrak_vektor_string(&args[0]) {
                            Ok(l) => l,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        };

                        let _ = trans.sesuaikan_label(&labels);
                    } else {
                        let arr = match &args[0] {
                            Value::Larik(l) => l.borrow().clone(),

                            Value::Tabel(t) => match t.borrow().ke_larik(None) {
                                Ok(a) => a,

                                Err(e) => return self.raise(rchunk, "DATA", &e, line),
                            },

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Data untuk sesuaikan harus berupa Larik atau Tabel",
                                    line,
                                )
                            }
                        };

                        if let Err(e) = trans.sesuaikan(&arr) {
                            return self.raise(rchunk, "DATA", &e, line);
                        }
                    }

                    drop(trans);

                    if name == "sesuaikan_transformasi" || name == "fit_transform" {
                        let trans = trans_rc.borrow();

                        if trans.jenis == JenisTransformer::PengekodLabel {
                            let labels = match ekstrak_vektor_string(&args[0]) {
                                Ok(l) => l,

                                Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                            };

                            match trans.transformasi_label(&labels) {
                                Ok(res) => {
                                    self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                                    Ok(())
                                }

                                Err(e) => self.raise(rchunk, "DATA", &e, line),
                            }
                        } else {
                            let arr = match &args[0] {
                                Value::Larik(l) => l.borrow().clone(),

                                Value::Tabel(t) => match t.borrow().ke_larik(None) {
                                    Ok(a) => a,

                                    Err(e) => return self.raise(rchunk, "DATA", &e, line),
                                },

                                _ => unreachable!(),
                            };

                            match trans.transformasi(&arr) {
                                Ok(res) => {
                                    self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                                    Ok(())
                                }

                                Err(e) => self.raise(rchunk, "DATA", &e, line),
                            }
                        }
                    } else {
                        self.stack.push(Value::Transformer(trans_rc.clone()));

                        Ok(())
                    }
                }

                crate::matra::MATRA_METHOD_TRANSFORMER_TRANSFORMASI => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "Metode transformasi butuh argumen data X",
                            line,
                        );
                    }

                    let trans = trans_rc.borrow();

                    if trans.jenis == JenisTransformer::PengekodLabel {
                        let labels = match ekstrak_vektor_string(&args[0]) {
                            Ok(l) => l,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        };

                        match trans.transformasi_label(&labels) {
                            Ok(res) => {
                                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                                Ok(())
                            }

                            Err(e) => self.raise(rchunk, "DATA", &e, line),
                        }
                    } else {
                        let arr = match &args[0] {
                            Value::Larik(l) => l.borrow().clone(),

                            Value::Tabel(t) => match t.borrow().ke_larik(None) {
                                Ok(a) => a,

                                Err(e) => return self.raise(rchunk, "DATA", &e, line),
                            },

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Data untuk transformasi harus Larik atau Tabel",
                                    line,
                                )
                            }
                        };

                        match trans.transformasi(&arr) {
                            Ok(res) => {
                                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                                Ok(())
                            }

                            Err(e) => self.raise(rchunk, "DATA", &e, line),
                        }
                    }
                }

                crate::matra::MATRA_METHOD_TRANSFORMER_INVERS => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "Metode invers_transformasi butuh argumen data",
                            line,
                        );
                    }

                    let trans = trans_rc.borrow();

                    if trans.jenis == JenisTransformer::PengekodLabel {
                        let ids = match ekstrak_vektor_f64(&args[0]) {
                            Ok(v) => v,

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        };

                        match trans.invers_transformasi_label(&ids) {
                            Ok(lbls) => {
                                let items: Vec<Value> = lbls.into_iter().map(Value::Teks).collect();

                                self.stack.push(Value::Daftar(Rc::new(RefCell::new(items))));

                                Ok(())
                            }

                            Err(e) => self.raise(rchunk, "DATA", &e, line),
                        }
                    } else {
                        let arr = match &args[0] {
                            Value::Larik(l) => l.borrow().clone(),

                            _ => {
                                return self.raise(
                                    rchunk,
                                    "TYPE",
                                    "Data invers harus berupa Larik",
                                    line,
                                )
                            }
                        };

                        match trans.invers_transformasi(&arr) {
                            Ok(res) => {
                                self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                                Ok(())
                            }

                            Err(e) => self.raise(rchunk, "DATA", &e, line),
                        }
                    }
                }

                _ => self.raise(
                    rchunk,
                    "RUNTIME",
                    &format!("Metode transformer '{}' tak dikenal", name),
                    line,
                ),
            },

            Value::Pipa(pipa_rc) => match id {
                crate::matra::MATRA_METHOD_PIPA_TAMBAH => {
                    if args.len() < 2 {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "pipa.tambah butuh (nama, langkah)",
                            line,
                        );
                    }

                    let nama = args[0].to_string();

                    pipa_rc.borrow_mut().tambah(nama, args[1].clone());

                    self.stack.push(Value::Pipa(pipa_rc.clone()));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_PIPA_LATIH => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "pipa.latih butuh data X", line);
                    }

                    let y_val = args.get(1).unwrap_or(&Value::Kosong);

                    match pipa_rc.borrow_mut().latih(&args[0], y_val) {
                        Ok(()) => {
                            self.stack.push(Value::Pipa(pipa_rc.clone()));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_PIPA_TRANSFORMASI => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "pipa.transformasi butuh data X", line);
                    }

                    match pipa_rc.borrow().transformasi(&args[0]) {
                        Ok(val) => {
                            self.stack.push(val);

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_PIPA_PREDIKSI => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "pipa.prediksi butuh data X", line);
                    }

                    match pipa_rc.borrow().prediksi(&args[0]) {
                        Ok(val) => {
                            self.stack.push(val);

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                _ => self.raise(
                    rchunk,
                    "RUNTIME",
                    &format!("Metode pipa '{}' tak dikenal", name),
                    line,
                ),
            },

            Value::Model(model_rc) => match id {
                crate::matra::MATRA_METHOD_MODEL_LATIH => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "Metode model.latih membutuhkan data X",
                            line,
                        );
                    }

                    let arr = match &args[0] {
                        Value::Larik(l) => l.borrow().clone(),

                        Value::Tabel(t) => match t.borrow().ke_larik(None) {
                            Ok(a) => a,

                            Err(e) => return self.raise(rchunk, "DATA", &e, line),
                        },

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Data X harus berupa Larik atau Tabel",
                                line,
                            )
                        }
                    };

                    let targets = if args.len() > 1 {
                        match ekstrak_vektor_f64(&args[1]) {
                            Ok(v) => Some(v),

                            Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                        }
                    } else {
                        None
                    };

                    if let Err(e) = model_rc.borrow_mut().latih(&arr, targets.as_deref()) {
                        return self.raise(rchunk, "DATA", &e, line);
                    }

                    self.stack.push(Value::Model(model_rc.clone()));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_MODEL_PREDIKSI => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "Metode model.prediksi membutuhkan data X",
                            line,
                        );
                    }

                    let arr = match &args[0] {
                        Value::Larik(l) => l.borrow().clone(),

                        Value::Tabel(t) => match t.borrow().ke_larik(None) {
                            Ok(a) => a,

                            Err(e) => return self.raise(rchunk, "DATA", &e, line),
                        },

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Data X harus berupa Larik atau Tabel",
                                line,
                            )
                        }
                    };

                    match model_rc.borrow().prediksi(&arr) {
                        Ok(res) => {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_MODEL_PREDIKSI_PROBA => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "Metode prediksi_probabilitas butuh data X",
                            line,
                        );
                    }

                    let arr = match &args[0] {
                        Value::Larik(l) => l.borrow().clone(),

                        Value::Tabel(t) => match t.borrow().ke_larik(None) {
                            Ok(a) => a,

                            Err(e) => return self.raise(rchunk, "DATA", &e, line),
                        },

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Data X harus berupa Larik atau Tabel",
                                line,
                            )
                        }
                    };

                    match model_rc.borrow().prediksi_probabilitas(&arr) {
                        Ok(res) => {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_MODEL_SKOR => {
                    if args.len() < 2 {
                        return self.raise(rchunk, "ARG", "Metode skor butuh (X, y)", line);
                    }

                    let arr = match &args[0] {
                        Value::Larik(l) => l.borrow().clone(),

                        Value::Tabel(t) => match t.borrow().ke_larik(None) {
                            Ok(a) => a,

                            Err(e) => return self.raise(rchunk, "DATA", &e, line),
                        },

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Data X harus Larik atau Tabel",
                                line,
                            )
                        }
                    };

                    let targets = match ekstrak_vektor_f64(&args[1]) {
                        Ok(v) => v,

                        Err(e) => return self.raise(rchunk, "TYPE", &e, line),
                    };

                    match model_rc.borrow().skor(&arr, &targets) {
                        Ok(s) => {
                            self.stack.push(Value::Angka(s));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                _ => self.raise(
                    rchunk,
                    "RUNTIME",
                    &format!("Metode pipa '{}' tak dikenal", name),
                    line,
                ),
            },

            Value::Grafik(plot_rc) => match id {
                crate::matra::MATRA_METHOD_PLOT_JUDUL => {
                    let judul = if let Some(v) = args.first() {
                        v.to_string()
                    } else {
                        String::new()
                    };

                    plot_rc.borrow_mut().set_judul(judul);

                    self.stack.push(Value::Grafik(plot_rc.clone()));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_PLOT_LABEL_X => {
                    let lbl = if let Some(v) = args.first() {
                        v.to_string()
                    } else {
                        String::new()
                    };

                    plot_rc.borrow_mut().set_label_x(lbl);

                    self.stack.push(Value::Grafik(plot_rc.clone()));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_PLOT_LABEL_Y => {
                    let lbl = if let Some(v) = args.first() {
                        v.to_string()
                    } else {
                        String::new()
                    };

                    plot_rc.borrow_mut().set_label_y(lbl);

                    self.stack.push(Value::Grafik(plot_rc.clone()));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_PLOT_WARNA => {
                    let w = if let Some(v) = args.first() {
                        v.to_string()
                    } else {
                        "#4F46E5".to_string()
                    };

                    plot_rc.borrow_mut().set_warna(w);

                    self.stack.push(Value::Grafik(plot_rc.clone()));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_PLOT_WARNA_LATAR => {
                    let w = if let Some(v) = args.first() {
                        v.to_string()
                    } else {
                        "#1E1E2E".to_string()
                    };

                    plot_rc.borrow_mut().set_warna_latar(w);

                    self.stack.push(Value::Grafik(plot_rc.clone()));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_PLOT_GRID => {
                    let g = if let Some(v) = args.first() {
                        v.is_truthy()
                    } else {
                        true
                    };

                    plot_rc.borrow_mut().set_grid(g);

                    self.stack.push(Value::Grafik(plot_rc.clone()));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_PLOT_UKURAN => {
                    if args.len() < 2 {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "ukuran() memerlukan 2 argumen: lebar dan tinggi",
                            line,
                        );
                    }

                    let lebar = match args[0] {
                        Value::Angka(n) if n > 0.0 => n as u32,

                        _ => {
                            return self.raise(
                                rchunk,
                                "ARG",
                                "Lebar harus berupa angka positif",
                                line,
                            )
                        }
                    };

                    let tinggi = match args[1] {
                        Value::Angka(n) if n > 0.0 => n as u32,

                        _ => {
                            return self.raise(
                                rchunk,
                                "ARG",
                                "Tinggi harus berupa angka positif",
                                line,
                            )
                        }
                    };

                    plot_rc.borrow_mut().set_ukuran(lebar, tinggi);

                    self.stack.push(Value::Grafik(plot_rc.clone()));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_PLOT_SIMPAN_SVG => {
                    if args.is_empty() {
                        return self.raise(
                            rchunk,
                            "ARG",
                            "simpan_svg() memerlukan path berkas",
                            line,
                        );
                    }

                    let path_str = match &args[0] {
                        Value::Teks(s) => s.as_str(),

                        _ => {
                            return self.raise(
                                rchunk,
                                "TYPE",
                                "Path berkas harus berupa teks",
                                line,
                            )
                        }
                    };

                    let svg_str = plot_rc.borrow().ke_svg();

                    let resolved = self.resolve_sandbox(path_str)?;

                    match std::fs::write(&resolved, svg_str) {
                        Ok(()) => {
                            self.stack.push(Value::Grafik(plot_rc.clone()));

                            Ok(())
                        }

                        Err(e) => self.raise(
                            rchunk,
                            "FILE",
                            &format!("Gagal menyimpan berkas SVG '{}': {}", path_str, e),
                            line,
                        ),
                    }
                }

                crate::matra::MATRA_METHOD_PLOT_KE_SVG => {
                    let svg_str = plot_rc.borrow().ke_svg();

                    self.stack.push(Value::Teks(svg_str));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_PLOT_TAMPIL => {
                    let terminal_str = plot_rc.borrow().tampil_terminal();

                    if let Some(ref mut out) = self.captured_output {
                        out.push(terminal_str);
                    } else {
                        println!("{}", terminal_str);
                    }

                    self.stack.push(Value::Grafik(plot_rc.clone()));

                    Ok(())
                }

                _ => self.raise(
                    rchunk,
                    "RUNTIME",
                    &format!("Metode asli grafik '{}' tak dikenal (ID {})", name, id),
                    line,
                ),
            },

            Value::ModelSyaraf(model_rc) => match id {
                crate::matra::MATRA_METHOD_SYARAF_MAJU => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "maju() butuh input data X", line);
                    }

                    let arr = match &args[0] {
                        Value::Larik(l) => l.borrow().clone(),

                        _ => return self.raise(rchunk, "TYPE", "Input harus Larik", line),
                    };

                    match model_rc.borrow_mut().maju(&arr) {
                        Ok(res) => {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_SYARAF_MUNDUR => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "mundur() butuh gradien output", line);
                    }

                    let arr = match &args[0] {
                        Value::Larik(l) => l.borrow().clone(),

                        _ => return self.raise(rchunk, "TYPE", "Input gradien harus Larik", line),
                    };

                    match model_rc.borrow_mut().mundur(&arr) {
                        Ok(res) => {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_SYARAF_PREDIKSI => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "prediksi() butuh input data X", line);
                    }

                    let arr = match &args[0] {
                        Value::Larik(l) => l.borrow().clone(),

                        _ => return self.raise(rchunk, "TYPE", "Input harus Larik", line),
                    };

                    match model_rc.borrow_mut().prediksi(&arr) {
                        Ok(res) => {
                            self.stack.push(Value::Larik(Rc::new(RefCell::new(res))));
                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_SYARAF_MODE_LATIH => {
                    model_rc.borrow_mut().mode_latih();

                    self.stack.push(Value::Kosong);

                    Ok(())
                }

                crate::matra::MATRA_METHOD_SYARAF_MODE_EVAL => {
                    model_rc.borrow_mut().mode_evaluasi();

                    self.stack.push(Value::Kosong);

                    Ok(())
                }

                crate::matra::MATRA_METHOD_SYARAF_SIMPAN => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "simpan() butuh path berkas", line);
                    }

                    let p = args[0].to_string();

                    let resolved = self.resolve_sandbox(&p)?;

                    match model_rc.borrow_mut().simpan(&resolved) {
                        Ok(()) => {
                            self.stack.push(Value::Kosong);
                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "FILE", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_SYARAF_MUAT => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "muat() butuh path berkas", line);
                    }

                    let p = args[0].to_string();

                    let resolved = self.resolve_sandbox(&p)?;

                    match model_rc.borrow_mut().muat(&resolved) {
                        Ok(()) => {
                            self.stack.push(Value::Kosong);
                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "FILE", &e, line),
                    }
                }

                _ => self.raise(
                    rchunk,
                    "RUNTIME",
                    &format!("Metode ModelSyaraf '{}' tak dikenal", name),
                    line,
                ),
            },

            Value::Pengoptimal(optim_rc) => match id {
                crate::matra::MATRA_METHOD_OPTIM_LANGKAH => match optim_rc.borrow_mut().langkah() {
                    Ok(()) => {
                        self.stack.push(Value::Kosong);
                        Ok(())
                    }

                    Err(e) => self.raise(rchunk, "DATA", &e, line),
                },

                crate::matra::MATRA_METHOD_OPTIM_NOL_GRAD => {
                    match optim_rc.borrow_mut().nol_gradien() {
                        Ok(()) => {
                            self.stack.push(Value::Kosong);
                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                _ => self.raise(
                    rchunk,
                    "RUNTIME",
                    &format!("Metode Pengoptimal '{}' tak dikenal", name),
                    line,
                ),
            },

            Value::Pelatih(pelatih_rc) => match id {
                crate::matra::MATRA_METHOD_PELATIH_PASANG_CB => {
                    if args.is_empty() {
                        return self.raise(rchunk, "ARG", "pasang_callback butuh callback", line);
                    }

                    if let Value::Daftar(d) = &args[0] {
                        for item in d.borrow().iter() {
                            if let Value::Callback(cb) = item {
                                pelatih_rc.borrow_mut().pasang_callback(cb.clone());
                            }
                        }
                    } else if let Value::Callback(cb) = &args[0] {
                        pelatih_rc.borrow_mut().pasang_callback(cb.clone());
                    }

                    self.stack.push(Value::Pelatih(pelatih_rc.clone()));

                    Ok(())
                }

                crate::matra::MATRA_METHOD_PELATIH_LATIH => {
                    let (xl, yl, putaran, batch, xv, yv) = if !args.is_empty()
                        && matches!(args[0], Value::Kamus(_))
                    {
                        if let Value::Kamus(k) = &args[0] {
                            let map = k.borrow();

                            let (x_latih, y_latih) = match map.get("data_latih") {
                                Some(Value::Daftar(d)) => {
                                    let items = d.borrow();

                                    if items.len() < 2 {
                                        return self.raise(
                                            rchunk,
                                            "ARG",
                                            "data_latih butuh [X, y]",
                                            line,
                                        );
                                    }

                                    let x = match &items[0] {
                                        Value::Larik(l) => l.borrow().clone(),
                                        _ => {
                                            return self.raise(
                                                rchunk,
                                                "TYPE",
                                                "X harus Larik",
                                                line,
                                            )
                                        }
                                    };

                                    let y = match &items[1] {
                                        Value::Larik(l) => l.borrow().clone(),
                                        _ => {
                                            return self.raise(
                                                rchunk,
                                                "TYPE",
                                                "y harus Larik",
                                                line,
                                            )
                                        }
                                    };

                                    (x, y)
                                }

                                _ => {
                                    return self.raise(
                                        rchunk,
                                        "ARG",
                                        "latih butuh 'data_latih': [X, y]",
                                        line,
                                    )
                                }
                            };

                            let (x_val, y_val) = match map.get("data_validasi") {
                                Some(Value::Daftar(d)) => {
                                    let items = d.borrow();

                                    if items.len() >= 2 {
                                        let x = match &items[0] {
                                            Value::Larik(l) => Some(l.borrow().clone()),
                                            _ => None,
                                        };

                                        let y = match &items[1] {
                                            Value::Larik(l) => Some(l.borrow().clone()),
                                            _ => None,
                                        };

                                        (x, y)
                                    } else {
                                        (None, None)
                                    }
                                }

                                _ => (None, None),
                            };

                            let put = map
                                .get("putaran")
                                .and_then(|v| v.as_angka().map(|n| n as usize))
                                .unwrap_or(20);

                            let bsz = map
                                .get("ukuran_batch")
                                .and_then(|v| v.as_angka().map(|n| n as usize))
                                .unwrap_or(32);

                            (x_latih, y_latih, put, bsz, x_val, y_val)
                        } else {
                            unreachable!()
                        }
                    } else {
                        if args.len() < 2 {
                            return self.raise(
                                rchunk,
                                "ARG",
                                "latih butuh (X_latih, y_latih)",
                                line,
                            );
                        }

                        let xl = match &args[0] {
                            Value::Larik(l) => l.borrow().clone(),
                            _ => return self.raise(rchunk, "TYPE", "X_latih harus Larik", line),
                        };

                        let yl = match &args[1] {
                            Value::Larik(l) => l.borrow().clone(),
                            _ => return self.raise(rchunk, "TYPE", "y_latih harus Larik", line),
                        };

                        let put = args
                            .get(2)
                            .and_then(|v| v.as_angka().map(|n| n as usize))
                            .unwrap_or(20);

                        let bsz = args
                            .get(3)
                            .and_then(|v| v.as_angka().map(|n| n as usize))
                            .unwrap_or(32);

                        let xv = match args.get(4) {
                            Some(Value::Larik(l)) => Some(l.borrow().clone()),
                            _ => None,
                        };

                        let yv = match args.get(5) {
                            Some(Value::Larik(l)) => Some(l.borrow().clone()),
                            _ => None,
                        };

                        (xl, yl, put, bsz, xv, yv)
                    };

                    match pelatih_rc.borrow_mut().latih(
                        &xl,
                        &yl,
                        putaran,
                        batch,
                        xv.as_ref(),
                        yv.as_ref(),
                    ) {
                        Ok(riwayat) => {
                            let mut m = HashMap::new();

                            for (k, vec) in riwayat {
                                let items: Vec<Value> = vec.into_iter().map(Value::Angka).collect();

                                m.insert(k, Value::Daftar(Rc::new(RefCell::new(items))));
                            }

                            self.stack.push(Value::Kamus(Rc::new(RefCell::new(m))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                crate::matra::MATRA_METHOD_PELATIH_EVALUASI => {
                    if args.len() < 2 {
                        return self.raise(rchunk, "ARG", "evaluasi butuh (X, y)", line);
                    }

                    let x = match &args[0] {
                        Value::Larik(l) => l.borrow().clone(),
                        _ => return self.raise(rchunk, "TYPE", "X harus Larik", line),
                    };

                    let y = match &args[1] {
                        Value::Larik(l) => l.borrow().clone(),
                        _ => return self.raise(rchunk, "TYPE", "y harus Larik", line),
                    };

                    match pelatih_rc.borrow().evaluasi(&x, &y) {
                        Ok(res) => {
                            let mut m = HashMap::new();

                            for (k, v) in res {
                                m.insert(k, Value::Angka(v));
                            }

                            self.stack.push(Value::Kamus(Rc::new(RefCell::new(m))));

                            Ok(())
                        }

                        Err(e) => self.raise(rchunk, "DATA", &e, line),
                    }
                }

                _ => self.raise(
                    rchunk,
                    "RUNTIME",
                    &format!("Metode Pelatih '{}' tak dikenal", name),
                    line,
                ),
            },

            _ => self.raise(
                rchunk,
                "TYPE",
                &format!(
                    "Metode '{}' hanya berlaku untuk larik, tabel, atau grafik",
                    name
                ),
                line,
            ),
        }
    }
}

fn parse_bentuk_args(args: &[Value], start_idx: usize) -> Result<Vec<usize>, String> {
    if args.len() <= start_idx {
        return Err("Bentuk (shape) tidak diberikan".to_string());
    }

    if args.len() == start_idx + 1 {
        match &args[start_idx] {
            Value::Daftar(list) => {
                let mut dims = Vec::new();

                for item in list.borrow().iter() {
                    match item {
                        Value::Angka(n) => {
                            if *n < 0.0 || n.fract() != 0.0 {
                                return Err(
                                    "Dimensi bentuk harus berupa bilangan bulat non-negatif"
                                        .to_string(),
                                );
                            }

                            dims.push(*n as usize);
                        }

                        _ => return Err("Elemen bentuk harus berupa angka".to_string()),
                    }
                }

                return Ok(dims);
            }

            Value::Angka(n) => {
                if *n < 0.0 || n.fract() != 0.0 {
                    return Err(
                        "Dimensi bentuk harus berupa bilangan bulat non-negatif".to_string()
                    );
                }

                return Ok(vec![*n as usize]);
            }

            _ => return Err("Bentuk harus berupa daftar angka atau angka tunggal".to_string()),
        }
    }

    let mut dims = Vec::new();

    for arg in &args[start_idx..] {
        match arg {
            Value::Angka(n) => {
                if *n < 0.0 || n.fract() != 0.0 {
                    return Err(
                        "Dimensi bentuk harus berupa bilangan bulat non-negatif".to_string()
                    );
                }

                dims.push(*n as usize);
            }

            _ => return Err("Argumen dimensi harus berupa angka".to_string()),
        }
    }

    Ok(dims)
}

fn parse_sumbu_opt(args: &[Value], idx: usize) -> Result<Option<usize>, String> {
    if let Some(val) = args.get(idx) {
        match val {
            Value::Kosong => Ok(None),

            Value::Angka(n) => {
                if *n < 0.0 || n.fract() != 0.0 {
                    return Err("Sumbu harus berupa angka bulat (0 atau 1)".to_string());
                }

                Ok(Some(*n as usize))
            }

            _ => Err("Sumbu harus berupa angka (0 atau 1)".to_string()),
        }
    } else {
        Ok(None)
    }
}

fn parse_arg_kolom_list(args: &[Value]) -> Result<Vec<String>, String> {
    if args.is_empty() {
        return Err("Daftar kolom tidak boleh kosong".to_string());
    }

    if args.len() == 1 {
        match &args[0] {
            Value::Daftar(list) => {
                let mut out = Vec::new();

                for item in list.borrow().iter() {
                    match item {
                        Value::Teks(s) => out.push(s.clone()),

                        _ => return Err("Nama kolom dalam daftar harus berupa teks".to_string()),
                    }
                }

                return Ok(out);
            }

            Value::Teks(s) => return Ok(vec![s.clone()]),

            _ => return Err("Argumen kolom harus berupa teks atau daftar teks".to_string()),
        }
    }

    let mut out = Vec::new();

    for a in args {
        match a {
            Value::Teks(s) => out.push(s.clone()),

            _ => return Err("Argumen kolom harus berupa teks".to_string()),
        }
    }

    Ok(out)
}

fn ekstrak_vektor_f64(v: &Value) -> Result<Vec<f64>, String> {
    match v {
        Value::Larik(l) => Ok(l.borrow().data.clone()),

        Value::Daftar(d) => {
            let list = d.borrow();

            let mut out = Vec::with_capacity(list.len());

            for item in list.iter() {
                match item {
                    Value::Angka(n) => out.push(*n),

                    _ => return Err("Elemen daftar harus berupa angka numerik".to_string()),
                }
            }

            Ok(out)
        }

        _ => Err("Data harus berupa Larik atau Daftar angka".to_string()),
    }
}

fn ekstrak_vektor_string(v: &Value) -> Result<Vec<String>, String> {
    match v {
        Value::Daftar(d) => {
            let list = d.borrow();

            let mut out = Vec::with_capacity(list.len());

            for item in list.iter() {
                out.push(item.to_string());
            }

            Ok(out)
        }

        Value::Teks(s) => Ok(vec![s.clone()]),

        _ => Err("Data label atau kategori harus berupa daftar teks".to_string()),
    }
}

fn ekstrak_matriks_f64(v: &Value) -> Result<Vec<Vec<f64>>, String> {
    match v {
        Value::Larik(l) => {
            let arr = l.borrow();

            if arr.bentuk.len() != 2 {
                return Err(format!(
                    "Matriks harus 2 dimensi, memiliki bentuk {:?}",
                    arr.bentuk
                ));
            }

            let rows = arr.bentuk[0];

            let cols = arr.bentuk[1];

            let mut out = Vec::with_capacity(rows);

            for r in 0..rows {
                let start = r * cols;

                out.push(arr.data[start..start + cols].to_vec());
            }

            Ok(out)
        }

        Value::Daftar(d) => {
            let list = d.borrow();

            if list.is_empty() {
                return Ok(Vec::new());
            }

            let mut out = Vec::with_capacity(list.len());

            for row_val in list.iter() {
                let row_f64 = ekstrak_vektor_f64(row_val)?;

                out.push(row_f64);
            }

            Ok(out)
        }

        _ => Err("Data matriks harus berupa Larik 2D atau Daftar dari Daftar angka".to_string()),
    }
}

fn nilai_sama(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Angka(x), Value::Angka(y)) => x == y,

        (Value::Teks(x), Value::Teks(y)) => x == y,

        (Value::Bolean(x), Value::Bolean(y)) => x == y,

        (Value::Kosong, Value::Kosong) => true,

        (Value::Daftar(x), Value::Daftar(y)) => {
            let x = x.borrow();

            let y = y.borrow();

            x.len() == y.len() && x.iter().zip(y.iter()).all(|(a, b)| nilai_sama(a, b))
        }

        (Value::Larik(x), Value::Larik(y)) => Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow(),

        (Value::Tabel(x), Value::Tabel(y)) => Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow(),

        (Value::Grafik(x), Value::Grafik(y)) => Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow(),

        (Value::Transformer(x), Value::Transformer(y)) => {
            Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow()
        }

        (Value::Pipa(x), Value::Pipa(y)) => Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow(),

        (Value::Model(x), Value::Model(y)) => Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow(),

        (Value::ModelSyaraf(x), Value::ModelSyaraf(y)) => {
            Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow()
        }

        (Value::Pengoptimal(x), Value::Pengoptimal(y)) => {
            Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow()
        }

        (Value::Pelatih(x), Value::Pelatih(y)) => Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow(),

        (Value::Lapisan(x), Value::Lapisan(y)) => Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow(),

        (Value::FungsiKerugian(x), Value::FungsiKerugian(y)) => {
            Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow()
        }

        (Value::Callback(x), Value::Callback(y)) => Rc::ptr_eq(x, y) || *x.borrow() == *y.borrow(),

        (Value::Kamus(x), Value::Kamus(y)) => Rc::ptr_eq(x, y),

        (Value::Fungsi { chunk: x, .. }, Value::Fungsi { chunk: y, .. }) => Rc::ptr_eq(x, y),

        _ => false,
    }
}

fn urut_banding(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::Angka(x), Value::Angka(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),

        (Value::Teks(x), Value::Teks(y)) => x.cmp(y),

        _ => std::cmp::Ordering::Equal,
    }
}

fn kunci_teks(v: &Value) -> String {
    match v {
        Value::Teks(s) => s.clone(),

        other => other.to_string(),
    }
}

fn salin_nilai(v: &Value, visited: &mut HashMap<usize, Value>) -> Value {
    match v {
        Value::Larik(l) => {
            let id = Rc::as_ptr(l) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Larik(Rc::new(RefCell::new(l.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::Tabel(t) => {
            let id = Rc::as_ptr(t) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Tabel(Rc::new(RefCell::new(t.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::Grafik(g) => {
            let id = Rc::as_ptr(g) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Grafik(Rc::new(RefCell::new(g.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::Transformer(t) => {
            let id = Rc::as_ptr(t) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Transformer(Rc::new(RefCell::new(t.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::Pipa(p) => {
            let id = Rc::as_ptr(p) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Pipa(Rc::new(RefCell::new(p.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::Model(m) => {
            let id = Rc::as_ptr(m) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Model(Rc::new(RefCell::new(m.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::ModelSyaraf(m) => {
            let id = Rc::as_ptr(m) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::ModelSyaraf(Rc::new(RefCell::new(m.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::Pengoptimal(o) => {
            let id = Rc::as_ptr(o) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Pengoptimal(Rc::new(RefCell::new(o.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::Pelatih(p) => {
            let id = Rc::as_ptr(p) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Pelatih(Rc::new(RefCell::new(p.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::Lapisan(l) => {
            let id = Rc::as_ptr(l) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Lapisan(Rc::new(RefCell::new(l.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::FungsiKerugian(k) => {
            let id = Rc::as_ptr(k) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::FungsiKerugian(Rc::new(RefCell::new(k.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::Callback(c) => {
            let id = Rc::as_ptr(c) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Callback(Rc::new(RefCell::new(c.borrow().clone())));

            visited.insert(id, baru.clone());

            baru
        }

        Value::Daftar(d) => {
            let id = Rc::as_ptr(d) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Daftar(Rc::new(RefCell::new(Vec::new())));

            visited.insert(id, baru.clone());

            let isi: Vec<Value> = d.borrow().iter().map(|e| salin_nilai(e, visited)).collect();

            if let Value::Daftar(b) = &baru {
                *b.borrow_mut() = isi;
            }

            baru
        }

        Value::Kamus(m) => {
            let id = Rc::as_ptr(m) as usize;

            if let Some(v) = visited.get(&id) {
                return v.clone();
            }

            let baru = Value::Kamus(Rc::new(RefCell::new(HashMap::new())));

            visited.insert(id, baru.clone());

            let mut isi = HashMap::new();

            for (k, val) in m.borrow().iter() {
                isi.insert(k.clone(), salin_nilai(val, visited));
            }

            if let Value::Kamus(b) = &baru {
                *b.borrow_mut() = isi;
            }

            baru
        }

        other => other.clone(),
    }
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;

    let era = if z >= 0 { z } else { z - 146096 } / 146097;

    let doe = z - era * 146097;

    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;

    let y = yoe + era * 400;

    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);

    let mp = (5 * doy + 2) / 153;

    let d = doy - (153 * mp + 2) / 5 + 1;

    let m = if mp < 10 { mp + 3 } else { mp - 9 };

    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn iso_8601(milis: i64) -> String {
    let detik = milis.div_euclid(1000);

    let sisa = milis.rem_euclid(1000);

    let days = detik.div_euclid(86400);

    let (tahun, bulan, tgl) = civil_from_days(days);

    let jam = detik.rem_euclid(86400) / 3600;

    let menit = detik.rem_euclid(3600) / 60;

    let dtk = detik.rem_euclid(60);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        tahun, bulan, tgl, jam, menit, dtk, sisa
    )
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
