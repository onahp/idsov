use hdk::prelude::*;
use patient_records_integrity::*;

#[hdk_extern]
pub fn get_all_records(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_records");
    get_links(path.path_entry_hash()?, LinkTypes::AllRecords, None)
}


#[hdk_extern]
pub fn get_all_records_by_recorder(
    recorder: AgentPubKey,
) -> ExternResult<Vec<Record>> {

    let links = get_links(recorder, LinkTypes::RecordsByRecorder, None)?;

    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::try_from(link.target)
                .map_err(|_| {
                    wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
                })
                .unwrap()
                .into(),
            GetOptions::default(),
        ))
        .collect();

    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
