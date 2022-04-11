use std::time::Duration;
use std::{fs, thread};
use std::io::Write;
use std::path::Path;

fn outfile() -> fs::File {
    if !(Path::new("./output.txt").exists()) {
        return fs::File::create("./output.txt").expect("[ X ] -> Could not create `output.txt`");
    } else {
       return fs::OpenOptions::new()
            .append(true)
            .open("./output.txt")
            .expect("[ X ] -> Could not append to `output.txt`");
    }
}

fn create(session: &ureq::Agent) -> Result<Vec<String>, ureq::Error> {
    let email = nanoid::nanoid!(10, &nanoid::alphabet::SAFE) + "@gmail.com";
    let password = nanoid::nanoid!(16, &nanoid::alphabet::SAFE);
    let post_data = format!("birth_day=1&birth_month=01&birth_year=1970&collect_personal_info=undefined&creation_flow=&creation_point=https://www.spotify.com/uk/&displayname=github.com/9sv&email={0}&gender=neutral&iagree=1&key=a1e486e2729f46d6bb368d6b2bcda326&password={1}&password_repeat={1}&platform=www&referrer=&send-email=1&thirdpartyemail=0&fb=0", email, password);

    let resp = session.post("https://spclient.wg.spotify.com/signup/public/v1/account")
        .set("Accept", "*/*")
        .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.141 Safari/537.36")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .set("Referer", "https,//www.spotify.com/")
        .send_string(&post_data)
        .expect("[ X ] -> RATE LIMITED");

    let json: ureq::serde_json::Value = resp.into_json()?;
    let __out = vec![email, password, (json["login_token"].to_string().replace('"', ""))];
    return Ok(__out);
}

fn main() {
    let mut outfile: fs::File = outfile();
    let session = ureq::AgentBuilder::new().build();
    let mut count = 0;
    let mut alerted = false;

    loop {
        let out = match create(&session) {
            Ok(r) => r.join(":"),
            Err(_) => continue,
        };
        match writeln!(outfile, "{}", format!("{}", out)) {
            Ok(r) => r,
            Err(_) => continue,
        };
        count += 1;
        println!("[ * ] -> CREATED {} SPOTIFY ACCOUNTS", count);
        if count >= 175 && !alerted {
            println!("[ ! ] -> 175 ACCOUNTS CREATED. RATE LIMITING LIKELY");
            thread::sleep(Duration::from_secs(5));
            alerted = true;
        }
    }
}