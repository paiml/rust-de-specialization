#[derive(Debug, PartialEq)]
struct NewFruit {
    name: String,
    price_cents: u64,
    quantity: u64,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    WrongFieldCount(usize),
    BadNumber(String),
}

fn parse_row(line: &str) -> Result<NewFruit, ParseError> {
    let fields: Vec<&str> = line.split(',').map(str::trim).collect();
    if fields.len() != 3 {
        return Err(ParseError::WrongFieldCount(fields.len()));
    }
    let num = |s: &str| {
        s.parse::<u64>()
            .map_err(|_| ParseError::BadNumber(s.to_string()))
    };
    Ok(NewFruit {
        name: fields[0].to_string(),
        price_cents: num(fields[1])?,
        quantity: num(fields[2])?,
    })
}

fn main() {
    let ok = parse_row("apple, 150, 12").unwrap();
    assert_eq!(
        ok,
        NewFruit {
            name: "apple".into(),
            price_cents: 150,
            quantity: 12
        }
    );
    assert_eq!(parse_row("pear,90"), Err(ParseError::WrongFieldCount(2)));
    assert_eq!(
        parse_row("fig,abc,3"),
        Err(ParseError::BadNumber("abc".into()))
    );

    let input = "apple,150,12\nbroken\nbanana,30,40\n";
    let (good, bad): (Vec<_>, Vec<_>) = input.lines().map(parse_row).partition(|r| r.is_ok());
    assert_eq!((good.len(), bad.len()), (2, 1));
    println!("csv: {} parsed, {} rejected", good.len(), bad.len());
}
