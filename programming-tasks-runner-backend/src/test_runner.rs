use sysinfo::{ProcessExt, System, SystemExt, Signal};
use tokio::process::{Command, Child};
use tokio::time;
use tokio::time::timeout;

use std::convert::TryFrom;
use std::time::{Duration};
use std::process::{Stdio};
use tokio::runtime::Runtime;

use thiserror::Error;
use serde::{Serialize, Deserialize};
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use unwrap_or::unwrap_ok_or;
use std::collections::HashMap;
use crate::Language;

#[derive(Error, Debug, Clone)]
pub enum TestError {
	// #[error("CPU limit reached")]
	// CpuLimit,
	#[error("RAM limit reached")]
	RamLimit,
	#[error("Timeout limit reached")]
	TimeoutLimit,
	#[error("Error received from `stderr`:\r\n{error:?}")]
	Stderr { error: String },
	//#[error("Error while compiling")]
	//CompileError,
	#[error("Wrong result - test failed")]
	TestFailed(u32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResult {
	pub ram_usage: i32,
	pub time: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
	// pub id: i32,
	// pub index: i32,
	pub input: Option<String>,
	pub output: String,
}


trait SliceExt {
    fn trim(&self) -> &Self;
}

impl SliceExt for [u8] {
    fn trim(&self) -> &[u8] {
        fn is_whitespace(c: &u8) -> bool {
            *c == b'\t' || *c == b' ' || *c == b'\n' || *c == b'\r'
        }

        fn is_not_whitespace(c: &u8) -> bool {
            !is_whitespace(c)
        }

        if let Some(first) = self.iter().position(is_not_whitespace) {
            if let Some(last) = self.iter().rposition(is_not_whitespace) {
                &self[first..last + 1]
            } else {
                unreachable!();
            }
        } else {
            &[]
        }
    }
}

fn strip_trailing_newline(input: &str) -> String {
	println!("str len: {}", input.len());
    input.trim_end_matches('\n').replace("\r", "")
}

async fn wait_for_exit(child: Child, valid_output: &String, test_index: u32) -> Result<TestResult, TestError> {
	let wwo = child.wait_with_output().await.expect("failed");
	
	loop {
		if wwo.status.success() {
			// println!("success {:?}", wwo);
			if strip_trailing_newline(&std::str::from_utf8(&wwo.stdout).unwrap()) == strip_trailing_newline(&valid_output) {
				// println!("if branch stdout {:?}", wwo.stdout);
				println!("!Success!");
				return Ok(TestResult { ram_usage: 24, time: 0.5 });
			} else {
				// println!("else branch stdout {:?}", wwo.stdout);
				println!("!Fail!");
				println!("Требовалось: {}", valid_output);
				println!("Оказалось: {}", std::str::from_utf8(&wwo.stdout).unwrap());
				return Err(TestError::TestFailed(test_index));
			}
		}
		else {
			// println!("failure {:?}", wwo);
			return Err(TestError::Stderr { error: String::from_utf8(wwo.stderr).expect("Found invalid UTF-8") });
		}
	}
}

pub async fn run(test_input: Option<String>, valid_output: String, ram_limit: u64, time_limit: f32, test_index: u32, language: &Language) -> Result<TestResult, TestError> {
	let rt = Runtime::new().unwrap();
	
	rt.block_on(async move {
		let mut child = if language.run.len() == 2 as usize {
			println!("running {} {}", language.run[0], language.run[1]);
			let mut args = language.run[1].split(" ");
			Command::new(&language.run[0])
				.args(args)
				.stdin(Stdio::piped())
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.spawn()
				.expect("failed to spawn")
		} else {
			println!("running {}", language.run[0]);
			Command::new(&language.run[0])
				.stdin(Stdio::piped())
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.spawn()
				.expect("failed to spawn")
		};

		let process_id = usize::try_from(child.id().unwrap()).unwrap();	
		println!("child proccess id is {}", process_id);
		
		if let Some(it) = test_input {
			println!("stdin start write:");
			let mut stdin = child.stdin.take().unwrap();
			let bytes = it.into_bytes();
			stdin.write_all(&bytes).await.unwrap();
			drop(stdin);
			println!("stdin end write.");
		}
		
		tokio::select! {
			it = check_ram(process_id, ram_limit) => Err(it),
			it = timeout(
				Duration::from_secs_f32(time_limit),
				wait_for_exit(child, &valid_output, test_index),
			) => match it {
				Ok(it) => it,
				Err(_) => {
					let (command, args) = if cfg!(target_os = "windows") {
						("takskill", vec!["/pid".to_string(), format!("{}", process_id), "/f".to_string()])
					} else if cfg!(target_os = "linux") {
						("kill", vec!["-9".to_string(), format!("{}", process_id)])
					} else {
						panic!("Неподдерживаемая ОС")
					};
					Command::new(command)
						.args(&args)
						.spawn()
						.expect("failed to spawn");

					Err(TestError::TimeoutLimit)
				},
			},
		}
	})
}

async fn get_ram(sys: &System, process_id: usize) -> Option<u64> {
	match sys.get_process(process_id as sysinfo::Pid) {
		Some(process) => {
			println!("ram_usage {} {:?}", process.name(), process.memory());
			Some(process.memory())
		}
		None => None
	}
}

async fn check_ram(process_id: usize, ram_limit: u64) -> TestError {
	let mut sys = System::new();
	let mut interval = time::interval(time::Duration::from_secs_f32(0.1));
	
	loop {
		sys.refresh_process(process_id as sysinfo::Pid);
		let ram = get_ram(&sys, process_id).await;
		
		if ram.is_some() {
			let unwrapperd = ram.unwrap();
			if unwrapperd > ram_limit {
				sys.get_process(process_id as sysinfo::Pid).unwrap().kill(Signal::Term);
				return TestError::RamLimit;
			}
		}
		
		interval.tick().await;
	}
}