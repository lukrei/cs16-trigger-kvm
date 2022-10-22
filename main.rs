use std::thread::sleep;
use std::time::Duration;

mod offsets;

const PROCESS_NAME: &str = "hl.exe";

fn main() {
    // Create context (attack to KVM)
    // If it fails panic with the error message
    let (mut ctx, c_ctx) = vmread::create_context(0).expect("Failed to create context");

    // Find the process from the process list
    let mut process = ctx
        .refresh_processes()
        .process_list
        .iter_mut()
        .find(|p| p.name.to_lowercase() == PROCESS_NAME.to_lowercase())
        .expect(&format!("Could not find process {}", PROCESS_NAME));

    // Get the needed module base addresses

    // Refresh the module list
    process.refresh_modules(c_ctx);

    // Get client.dll
    let mut client_base = process
        .module_list
        .iter()
        .find(|module| module.name.to_lowercase() == String::from("client.dll"))
        .expect("Could not find client.dll")
        .info
        .baseAddress as u32;

    // Get engine.dll
    let mut engine_base = process
        .module_list
        .iter()
        .find(|module| module.name.to_lowercase() == String::from("hw.dll"))
        .expect("Could not find hw.dll")
        .info
        .baseAddress as u32;
let Geld: u32 = process.read(&c_ctx, (client_base + offsets::dwLeben) as u64);
println!("{} Leben: ", Geld, );

    loop {
        sleep(Duration::from_millis(1));
process.write(&c_ctx, (client_base + offsets::dwAttack) as u64, &4);
        // Get the local player base address
        let local_team: u32 = process.read(&c_ctx, (client_base + offsets::dwTeam) as u64);
        let crosshair_id: u32 = process.read(&c_ctx, (client_base + offsets::dwCrossId) as u64);
//println!("{} local_team: vor check ob id = 0 ist", local_team, );
//println!("{} crosshair_id: vor check ob id = 0 ist", crosshair_id, );
        // Continue if there is no entity on crosshair
        if crosshair_id == 0 {
            continue;
        }
//println!("{} Crosshair_ID: nach check ob crosshair id = 0 ist ", crosshair_id, );
        // Get the entity associated with the crosshair_id
	let crosshair_entity: u32 = process.read(&c_ctx, (client_base + offsets::dwCrossId) as u64);
        let crosshair_entity_team: u32 = process.read(&c_ctx, (client_base + offsets::dwCrossIdTeam) as u64);
//println!("{} Crosshair Entity: ", crosshair_entity, );
//println!("{} Crosshair Entity Team: ", crosshair_entity_team, );
        // Continue if the entity doesn't exist or it's on the same team
//        if crosshair_entity_team == local_team {
//            continue;
//println!("{} Crosshair Entity Team: ", crosshair_entity_team, );
//println!("{} Local Team: ", local_team, );
//        }

        // If we get here, the crosshair is on an enemy

        // Shoot by setting forceAttack to 6 (+attack & -attack)
        process.write(&c_ctx, (client_base + offsets::dwAttack) as u64, &5);
    }
}

