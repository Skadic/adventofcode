use tracing::info;

use crate::{parse_input, HIGH_COUNT, INPUT, LOW_COUNT};

#[tracing::instrument(name = "part1")]
pub fn process() -> miette::Result<()> {
    let mut map = parse_input(INPUT)?;
    map.insert(
        "button",
        crate::Module {
            name: "button",
            tp: crate::ModuleType::Broadcast,
            inputs: vec![],
            outputs: vec!["broadcaster"],
        },
    );
    map.get_mut("broadcaster").unwrap().add_input("button");
    let button = &map["button"];

    for _ in 0..1000 {
        button.propagate(crate::Pulse::Low, &map);
    }

    info!(result = unsafe { LOW_COUNT * HIGH_COUNT });
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
