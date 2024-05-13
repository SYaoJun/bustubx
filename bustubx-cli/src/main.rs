use bustubx::{pretty_format_tuples, BustubxResult, Database};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use tracing::info;
fn main() -> Result<()> {
    println!(":) Welcome to the bustub-rust, please input sql.");
    // 创建一个db对象
    let mut db = Database::new_temp().unwrap();
    info!("database created");
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("bustub-rust=# ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                if line == "exit" || line == "\\q" {
                    println!("bye!");
                    break;
                }
                // db.run()是函数入口
                let result = db.run(&line);
                match result {
                    Ok(tuples) => {
                        if !tuples.is_empty() {
                            println!("{}", pretty_format_tuples(&tuples))
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
