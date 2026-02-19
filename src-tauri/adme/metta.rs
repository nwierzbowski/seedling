use hyperon::metta::{runner::Metta, text::SExprParser};
use hyperon_atom::Atom;

pub fn spawn_metta_thread() -> anyhow::Result<(
    crossbeam::channel::Sender<String>,
    crossbeam::channel::Receiver<Result<Vec<String>, String>>,
    std::thread::JoinHandle<anyhow::Result<()>>,
)> {
    let (cmd_send, cmd_recv) = crossbeam::channel::unbounded::<String>();
    let (resp_send, resp_recv) = crossbeam::channel::unbounded::<Result<Vec<String>, String>>();

    let handle: std::thread::JoinHandle<anyhow::Result<()>> = std::thread::spawn(move || {
        let metta = Metta::new(None);
        let space = metta.space();

        // 1. Load and EXECUTE the core logic directly in the metta instance
        let core_logic = include_str!("../../core_logic.metta");
        let mut parser = SExprParser::new(core_logic);

        loop {
            match parser.parse(&metta.tokenizer().borrow()) {
                Ok(Some(atom)) => space.borrow_mut().add(atom),
                Ok(None) => break, // No more atoms to parse
                _ => {
                    return Err(anyhow::anyhow!("Failed to parse core logic"));
                }
            }
        }

        // Process incoming commands from the channel
        while let Ok(code) = cmd_recv.recv() {
            let query = SExprParser::new(&code);
            let result = metta.run(query);

            let responses = match result {
                Ok(results) => {
                    results
                        .into_iter()
                        .flatten()
                        .map(|atom| match atom {
                            Atom::Expression(expr) => {
                                if let Some(first) = expr.children().get(0) {
                                    if first.to_string() == "Error" {
                                        println!("Caught error trying to add child to a non existent parent");
                                        return Err(format!("Metta error: {}", expr));
                                    }
                                }
                                Err("Unsupported return: expected grounded value".to_string())
                            }
                            Atom::Grounded(grounded) => {
                                let value = grounded.to_string();
                                Ok(value.trim_matches('"').to_string())
                            }
                            _ => Err("Unsupported return: unexpected atom type".to_string()),
                        })
                        .collect::<Result<Vec<String>, String>>()
                }
                Err(e) => Err(e.to_string()),
            };

            println!("Metta received command: {}", code);
            // Optionally log results for debugging: println!("Metta result: {:?}", responses);

            // Send the result back through the response channel
            if let Err(e) = resp_send.send(responses) {
                eprintln!("Failed to send response: {}", e);
                break;
            }
        }
        Ok(())
    });

    Ok((cmd_send, resp_recv, handle))
}
