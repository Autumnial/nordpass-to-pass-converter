use std::{
    fs::read_to_string,
    io::{BufRead, BufReader, Read, Write},
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};


fn read_file(filename: &str) -> Vec<String> {
    let mut res = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        res.push(line.to_string());
    }

    res
}



fn main() {
    for i in 0..=10 {
        let pass_name = "test/test".to_string() + i.to_string().as_str();

        let mut child = Command::new("pass")
            .arg("insert")
            .arg("-m")
            .arg(pass_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let cmd_std_out = child.stdout.as_mut().take().unwrap();
        let cmd_std_in = child.stdin.as_mut().take().unwrap();

        sleep(Duration::new(1, 0));

        cmd_std_in.write("pass\n".as_bytes()).unwrap();
        cmd_std_in.write("word".as_bytes()).unwrap();
        cmd_std_in.write("^d".as_bytes()).unwrap();
    }
}
