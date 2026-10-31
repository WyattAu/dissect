//! dissect-evidence: tamper-evident chain for investigation evidence.
//!
//! Pins `tamper-audit` (WyattAu/auditlog) as its integrity substrate: every
//! evidence record is an AuditEntry in a SHA-256 chain, so a findings manual
//! can be checked against what the engines actually reported, after the fact.
//!
//! Chain file: JSON array of AuditEntry (default ./evidence/chain.json).
//! Genesis previous_hash is the 64-zero hash (tamper-audit's convention).
//!
//! Usage:
//!   dissect-evidence append <inv-id> <actor> <action> <resource>
//!                          [--details '<json>'] [--file <path>]
//!   dissect-evidence verify
//!   dissect-evidence list [--investigation <id>]
//!
//! Exit codes: 0 ok, 1 verification failure, 2 usage/IO error.

use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tamper_audit::{AuditChain, AuditEntry};

const GENESIS: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Serialize, Deserialize)]
struct ChainFile {
    schema: u32,
    entries: Vec<AuditEntry>,
}

impl ChainFile {
    fn load(path: &PathBuf) -> Result<Self, String> {
        if !path.exists() {
            return Ok(ChainFile {
                schema: 1,
                entries: Vec::new(),
            });
        }
        let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&raw).map_err(|e| format!("chain file corrupt: {e}"))
    }

    fn save(&self, path: &PathBuf) -> Result<(), String> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    }
}

fn sha256_file(path: &str) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{path}: {e}"))?;
    let mut h = Sha256::new();
    h.update(&bytes);
    Ok(format!("{:x}", h.finalize()))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let chain_path = PathBuf::from(
        std::env::var("DISSECT_EVIDENCE_CHAIN").unwrap_or_else(|_| "evidence/chain.json".into()),
    );

    let code = match args.first().map(String::as_str) {
        Some("append") => cmd_append(&args[1..], &chain_path),
        Some("verify") => cmd_verify(&chain_path),
        Some("list") => cmd_list(&args[1..], &chain_path),
        _ => {
            eprintln!("usage: dissect-evidence append|verify|list ...");
            2
        }
    };
    std::process::exit(code);
}

fn cmd_append(args: &[String], chain_path: &PathBuf) -> i32 {
    if args.len() < 4 {
        eprintln!("usage: append <inv-id> <actor> <action> <resource> [--details '<json>'] [--file <path>]");
        return 2;
    }
    let (inv, actor, action, resource) = (&args[0], &args[1], &args[2], &args[3]);
    let mut details = json!({ "investigation": inv });
    let mut i = 4;
    while i < args.len() {
        match args[i].as_str() {
            "--details" => {
                match serde_json::from_str::<serde_json::Value>(args.get(i + 1).map(String::as_str).unwrap_or("")) {
                    Ok(v) => details["note"] = v,
                    Err(e) => {
                        eprintln!("--details must be valid JSON: {e}");
                        return 2;
                    }
                }
                i += 2;
            }
            "--file" => {
                match sha256_file(args.get(i + 1).map(String::as_str).unwrap_or("")) {
                    Ok(h) => details["file_sha256"] = json!(h),
                    Err(e) => {
                        eprintln!("{e}");
                        return 2;
                    }
                }
                details["file_path"] = json!(args.get(i + 1).cloned().unwrap_or_default());
                i += 2;
            }
            other => {
                eprintln!("unknown flag: {other}");
                return 2;
            }
        }
    }

    let mut chain = match ChainFile::load(chain_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return 2;
        }
    };
    let previous = chain.entries.last().map(|e| e.hash.clone()).unwrap_or_else(|| GENESIS.to_string());
    let entry = AuditEntry::new(actor, action, resource, details, &previous);
    println!(
        "{}",
        json!({"ok": true, "index": chain.entries.len(), "id": entry.id.to_string(), "hash": entry.hash})
    );
    chain.entries.push(entry);
    match chain.save(chain_path) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("save failed: {e}");
            2
        }
    }
}

fn cmd_verify(chain_path: &PathBuf) -> i32 {
    let chain = match ChainFile::load(chain_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return 2;
        }
    };
    let result = AuditChain::verify(&chain.entries);
    let ok = result.is_valid();
    println!(
        "{}",
        json!({
            "ok": ok,
            "total_entries": result.total_entries,
            "valid_entries": result.valid_entries,
            "broken_at": result.broken_at,
            "error": result.error,
        })
    );
    if ok {
        0
    } else {
        1
    }
}

fn cmd_list(args: &[String], chain_path: &PathBuf) -> i32 {
    let inv = args
        .iter()
        .position(|a| a == "--investigation")
        .and_then(|p| args.get(p + 1));
    let chain = match ChainFile::load(chain_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return 2;
        }
    };
    for (idx, e) in chain.entries.iter().enumerate() {
        let entry_inv = e.details.get("investigation").and_then(|v| v.as_str()).unwrap_or("");
        if let Some(want) = inv {
            if entry_inv != want {
                continue;
            }
        }
        println!(
            "{}",
            json!({"index": idx, "actor": e.actor, "action": e.action, "resource": e.resource, "investigation": entry_inv, "hash": e.hash})
        );
    }
    0
}
