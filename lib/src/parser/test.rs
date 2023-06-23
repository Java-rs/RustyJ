#[test]
fn testtest() -> bool {
    let test: Example = parse_Example(
        "Val,Richter\nBene,Brandmeier\nMario,Hinkel\nMaxi,Floto\nPflipper,Wolf\nTori,Gonnheimer\nSander,Stella\n",
    )
        .expect("u suck");
    println!("{}", serde_json::to_string(&test).unwrap());
    true
}
