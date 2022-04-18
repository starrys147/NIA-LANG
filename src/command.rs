//

use regex::Regex;

use crate::buffer::Buffer;

pub struct Executer {
    commands: Vec<Cmd>,
    buffer: Buffer,
    idx: usize,
}

enum Cmd {
    Add,
    Sub,
    Getc,
    Putc,
    Inc,
    Dec,
    LoopB(usize),
    LoopE(usize),
    CmdErr(String),
}

impl Executer {
    pub fn from(nias: String) -> Result<Executer, String> {
        use Cmd::*;
        let re = Regex::new(r"(?i)(nia)\s*").unwrap();

        let commands = re
            .captures_iter(&nias[..])
            .map(|cap| match &cap[1] {
                "nia" => Add,                                             // bf +
                "niA" => Sub,                                             // bf -
                "nIa" => Getc,                                            // bf ,
                "nIA" => Putc,                                            // bf .
                "Nia" => Inc,                                             // bf >
                "NiA" => Dec,                                             // bf <
                "NIA" => LoopB(0),                                        // bf [
                "NIa" => LoopE(0),                                        // bf ]
                other => CmdErr(format!("No such operator: {}", &other)), // Error
            })
            .collect::<Vec<Cmd>>();

        let commands = pair(commands)?;

        Ok(Executer {
            commands,
            buffer: Buffer::new(),
            idx: 0,
        })
    }

    fn update(&mut self) {
        self.idx += 1;
    }

    pub fn run(&mut self) -> Result<(), String> {
        use Cmd::*;
        while let Some(code) = self.commands.get(self.idx) {
            match code {
                Add => self.buffer.add(),   // bf +
                Sub => self.buffer.sub(),   // bf -
                Getc => self.buffer.getc(), // bf ,
                Putc => self.buffer.putc(), // bf .
                Inc => self.buffer.inc(),   // bf >
                Dec => self.buffer.dec(),   // bf <
                LoopB(end) => {
                    if self.buffer.val() == 0 {
                        self.idx = end.clone();// bf [
                    }
                }
                LoopE(begin) => {
                    if self.buffer.val() > 0 {
                        self.idx = begin.clone();
                    }
                }
                CmdErr(err) => return Err(err.clone()), // Return The Error
            }
            self.update();
        }

        Ok(())
    }
}

fn pair(mut cmds: Vec<Cmd>) -> Result<Vec<Cmd>, String> {
    let mut i = 0;
    let mut unpaired: Vec<usize> = Vec::new();

    while let Some(cmd) = cmds.get(i) {
        if let Cmd::LoopB(_) = cmd {
            unpaired.push(i);

        } else if let Cmd::LoopE(_) = cmd {
            let idx = match unpaired.pop() {
                Some(val) => val,
                None => return Err(String::from("Got unpaired `NIa`s")),
            };

            cmds[idx] = Cmd::LoopB(i);
            cmds[i] = Cmd::LoopE(idx);
        }
        i += 1;
    }

    if unpaired.len() > 0 {
        return Err(String::from("Got unpaired `NIA`s"));
    }

    Ok(cmds)
}