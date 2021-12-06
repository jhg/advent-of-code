mod opt;

use std::io;

fn main() -> io::Result<()> {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in opt::Opt::from_args().lines()? {
        let (action, units) = split_command(&line)?;
        match action {
            "forward" => horizontal_position += units,
            "down" => depth += units,
            "up" => depth -= units,
            _ => unreachable!(),
        }
    }
    println!("{}", horizontal_position * depth);
    return Ok(());
}

fn split_command<'a>(command: &'a str) -> io::Result<(&'a str, usize)> {
    let offset = command.find(' ').ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Can NOT split command"))?;
    let action = &command[..offset];
    let units = command[offset..].trim().parse::<usize>().map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    return Ok((action, units));
}
