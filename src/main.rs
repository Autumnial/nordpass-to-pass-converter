use std::{
    fs::read_to_string,
    io::{BufRead, BufReader, Read, Write},
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

struct Password {
    name: String,
    username: String,
    password: String,
    url: String,
}

fn read_file(filename: &str) -> Vec<String> {
    let mut res = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        res.push(line.to_string());
    }

    res
}

fn parse_file(lines: Vec<String>) -> Vec<Password> {
    let mut pws = Vec::new();

    for l in lines {
        let vals: Vec<&str> = l.split(',').collect();

        pws.push(Password {
            name: vals[0].to_string(),
            username: vals[3].to_string(),
            password: vals[4].to_string(),
            url: vals[1].to_string(),
        });
    }

    pws
}

fn main() {
    let lines = read_file("input.csv");
    let passwords = parse_file(lines);

    for pw in passwords {
        let pass_name = pw.name;

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

        sleep(Duration::new(0, 500_000_000));

        let url = "Url: ".to_string() + &pw.url + "\n";
        let username = "Username: ".to_string() + &pw.username + "\n";
        let password = "Password: ".to_string() + &pw.password + "\n";

        cmd_std_in.write(url.as_bytes()).unwrap();
        cmd_std_in.write(username.as_bytes()).unwrap();
        cmd_std_in.write(password.as_bytes()).unwrap();
    }
}
