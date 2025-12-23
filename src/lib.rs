use anyhow::{Context, Result};
use std::{fs, io::Write};

use polkavm::ProgramBlob;
use polkavm_disassembler::{Disassembler, DisassemblyFormat};

fn main() -> Result<()> {
    let in_path = std::env::args()
        .nth(1)
        .context("usage: polkavm_disasm <contract.polkavm> [out.asm]")?;

    let out_path = std::env::args().nth(2).unwrap_or_else(|| "out.asm".into());

    let bytes = fs::read(&in_path).with_context(|| format!("read {in_path}"))?;
    let blob = ProgramBlob::parse(bytes).context("parse polkavm ProgramBlob")?; // :contentReference[oaicite:2]{index=2}

    let mut dis = Disassembler::new(&blob, DisassemblyFormat::Guest)?;
    dis.show_raw_bytes(true);

    let mut out = fs::File::create(&out_path).with_context(|| format!("create {out_path}"))?;
    dis.disassemble_into(&mut out)?;
    out.flush()?;

    eprintln!("wrote {out_path}");
    Ok(())
}
