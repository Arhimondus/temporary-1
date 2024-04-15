use actix_web::{error, web, App, Error, HttpServer, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};
use serde_derive::{Serialize as SdSerialize, Deserialize as SdDeserialize};
use std::path::Path;
use tokio::process::{Command, Child};
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use tokio::runtime::Runtime;
use tokio::task;
use std::time::SystemTime;
use chrono;
use rlua::{Lua, Result as LuaResult};
use actix_web::http::StatusCode;
use std::fs;
use crate::test_runner::{run, Test};
use derive_more::{Display, Error};
use std::collections::HashMap;
use toml::from_str;
mod test_runner;

#[derive(Serialize, Deserialize, Debug)]
struct ProblemSolution {
	// problem_id: u32,
	language: String,
	pre_compile_script: Option<String>,
	code: String,
	ram_limit: u64,
	time_limit: f32,
	tests: Vec<Test>,
} 

#[derive(SdSerialize, SdDeserialize, Debug, Clone)]
pub struct Language {
	id: String,
	idol: String,
	file: String,
	init: Option<Vec<String>>,
	output: Option<String>,
	run: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
	name: String,
}
impl error::ResponseError for MyError {}


const MAX_SIZE: usize = 262_144; // max payload size is 256k

async fn problem(data: web::Json<ProblemSolution>) -> Result<HttpResponse, Error> {
	/*fs::create_dir_all("./")
	let root = Path::new("/")
	env::set_current_dir(&root);
	println!("Работаем в директории:", root.display());*/
	
	let problem: ProblemSolution = data.into_inner();
	
	let languages_string: String = fs::read_to_string(if cfg!(target_os = "windows") {
		"languages-windows.toml"
	} else if cfg!(target_os = "linux") {
		"./rc/languages-linux.toml"
	} else {
		panic!("Неподдерживаемая ОС")
	})?;
	let languages_hm: HashMap<String, Vec<Language>> = toml::from_str(&languages_string).unwrap();
	let languages: &[Language] = &languages_hm["languages"];
	let mut language: Language = languages.into_iter().find(|it| it.id == problem.language).unwrap().clone();
	// unwrap_or_else(|err| {	Err(MyError { name: format!("{}", err.to_string()) }) })?;
	// println!("{:?}", languages);
	println!("{:?}", problem);
	// return Ok(HttpResponse::Ok().body(format!("Success {}", problem.language)));

	println!("now is {:?}", chrono::offset::Local::now());	
	
	let pre_compile_script = problem.pre_compile_script;
	let mut code = problem.code;
	if let Some(script) = pre_compile_script {
		print!("Found precompile script - executing...");
		let lua = Lua::new();
		lua.context(|lua_ctx| {
			let globals = lua_ctx.globals();
			globals.set("input", &*code)?;
			code = lua_ctx.load(&script).eval::<String>()?;
			
			LuaResult::Ok(())
		}).expect("LuaContext Panic");
		println!("..Done");
	}
	let tests = problem.tests;
	let ram_limit = problem.ram_limit;
	let time_limit = problem.time_limit;
	
	if let Some(output) = &language.output {
		language.output = Some(output.replace("%idol", &language.idol));
	}
	
	language.file = language.file.replace("%idol", &language.idol);
	language.run = language.run.iter().map(|it| it.replace("%idol", &language.idol).to_string()).collect();
	
	println!("LR {:?}", language.run);

	if let Some(init) = &language.init {
		language.init = Some(init.iter().map(|it| it.replace("%idol", &language.idol).to_string()).collect());
	}
	
	// return Ok(HttpResponse::Ok().body(format!("Success language {:?}", language)));

	let rt = Runtime::new().unwrap();
	
	let clonedLanguage = language.clone();
	let res = rt.block_on(async move {
		println!("start compilation:");

		let mut file = File::create(&clonedLanguage.file).await.unwrap();
		file.write_all(code.as_bytes()).await.unwrap();
		drop(file);

		// Удаляем старый выходной файл, если присутствует у языка (т.е. язык компилируемый)
		if let Some(output) = &clonedLanguage.output {
			if Path::new(&output).exists() {
				std::fs::remove_file(&output).unwrap()
			}
				
			// Запуск процесса компиляции
			if let Some(init) = &clonedLanguage.init {
				let mut args = init[1].split(" ");
				let mut compilate_child = Command::new(&init[0])
					.args(args)
					.spawn()
					.expect("failed to spawn");
				let _ = compilate_child.wait().await.unwrap();
				if !Path::new(&output).exists() {
					return -1;
				}
				
				if cfg!(target_os = "linux") {
					Command::new("chmod")
						.arg(format!("-x {}", &output))
						.spawn()
						.expect("failed to spawn");
					let _ = compilate_child.wait().await.expect("Проставить права на файл невозможно, запуск так же невозможен.");
				}
				
				println!("end compilation.");
			}
		}
		return 0
	});
	
	if res == -1 {
		println!("!CompilationError!");
		return Ok(HttpResponse::Ok().body(format!("CompilationError")))
	}
	
	let mut test_index = 1;
	for test in tests {
		let input = test.input;
		let output = test.output;
		let result = run(input, output, ram_limit, time_limit, test_index, &language).await;
		if let Err(err) = result {
			return Ok(HttpResponse::Ok().body(format!("{:?}", err)));
		}
		test_index += 1;
	}
	
	println!("the end!");
	
	Ok(HttpResponse::Ok().body("Success"))
}

#[derive(Deserialize, Debug)]
struct Configuration {
	host: String,
	port: u16,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let Configuration { host: host, port: port } = envy::from_env::<Configuration>()
		.expect("Please provide HOST and PORT env vars");
		
	HttpServer::new(|| {
		App::new()
			.service(web::resource("/problem").route(web::post().to(problem)))
	})
	.bind((host, port))?
	.run()
	.await
}