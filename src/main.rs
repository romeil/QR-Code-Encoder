use clap::{command, Arg};
use qrcode::QrCode;
use image::Luma;

fn main() {
    let match_result = command!()
        .about("This application acts as a QR-Code encoder")
        .arg(
            Arg::new("data")
                .short('d')
                .long("data")
                .required(true)
                .help("The data to be encoded")   
        )
        .arg(
            Arg::new("name")
                .default_value("qrcode")
                .short('n')
                .long("name")
                .help("The name of the QR code image")
                
        )
    .get_matches();

    let data = match_result.get_one::<String>("data").unwrap().as_str();
    let name = match_result.get_one::<String>("name").unwrap().as_str();

    let code = QrCode::new(data).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save(format!(r"**Desired path**\{name}.png")).unwrap();
}
