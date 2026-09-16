use crate::bytecode::{Chunk, OpCode};
use crate::compiler::Compiler;
use crate::value::Value;
use evernight_core::errors::EvernightError;
use evernight_core::{Lexer, Parser};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
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
                        (Value::Kamus(map), Value::Teks(k)) => {
                            map.borrow_mut().insert(k.clone(), value.clone());
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
                        (Value::Kamus(map), Value::Teks(k)) => {
                            map.borrow_mut().insert(k.clone(), value.clone());
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
                match std::fs::read_to_string(&path) {
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
                match std::fs::write(&path, isi) {
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
            54 => Value::Bolean(
                std::fs::metadata(s!(0))
                    .map(|m| m.is_file())
                    .unwrap_or(false),
            ),
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
        let mut path = PathBuf::from(module);
        if path.extension().is_none() {
            path.set_extension("eve");
        }
        if path.is_relative() {
            path = self.entry_dir.join(path);
        }
        let path_str = path.to_string_lossy().to_string();

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
