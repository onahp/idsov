use hdk::prelude::*;
use patient_records_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddRecordForRecorderInput {
    pub base_recorder: AgentPubKey,
    pub target_record_hash: ActionHash,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveRecordForRecorderInput {
    pub base_recorder: AgentPubKey,
    pub target_record_hash: ActionHash,
}

#[hdk_extern(visibility = "private")]
pub fn create_patient_record(patient_record: PatientRecord) -> ExternResult<Record> {
    debug!("create patient record: {:?}", patient_record);
    let patient_record_hash = create_entry(
        &EntryTypes::PatientRecord(patient_record.clone()),
    )?;
    let record = get(patient_record_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created PatientRecord"))
            ),
        )?;
    let path = Path::from("all_records");
    create_link(
        path.path_entry_hash()?,
        patient_record_hash.clone(),
        LinkTypes::AllRecords,
        (),
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_latest_patient_record(
    original_patient_record_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_patient_record_hash.clone(),
        LinkTypes::PatientRecordUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_patient_record_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest(String::from("No action hash associated with link"))
                    ),
                )?
        }
        None => original_patient_record_hash.clone(),
    };
    get(latest_patient_record_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_original_patient_record(
    original_patient_record_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_patient_record_hash, GetOptions::default())?
    else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(String::from("Malformed get details response"))
                ),
            )
        }
    }
}

#[hdk_extern]
pub fn get_all_revisions_for_patient_record(
    original_patient_record_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_patient_record(
        original_patient_record_hash.clone(),
    )? else {
        return Ok(vec![]);
    };
    let links = get_links(
        original_patient_record_hash.clone(),
        LinkTypes::PatientRecordUpdates,
        None,
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| Ok(
            GetInput::new(
                link
                    .target
                    .into_action_hash()
                    .ok_or(
                        wasm_error!(
                            WasmErrorInner::Guest(String::from("No action hash associated with link"))
                        ),
                    )?
                    .into(),
                GetOptions::default(),
            ),
        ))
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let mut records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    records.insert(0, original_record);
    Ok(records)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePatientRecordInput {
    pub original_patient_record_hash: ActionHash,
    pub previous_patient_record_hash: ActionHash,
    pub updated_patient_record: PatientRecord,
}

#[hdk_extern]
pub fn update_patient_record(input: UpdatePatientRecordInput) -> ExternResult<Record> {
    let updated_patient_record_hash = update_entry(
        input.previous_patient_record_hash.clone(),
        &input.updated_patient_record,
    )?;
    create_link(
        input.original_patient_record_hash.clone(),
        updated_patient_record_hash.clone(),
        LinkTypes::PatientRecordUpdates,
        (),
    )?;
    let record = get(updated_patient_record_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated PatientRecord"))
            ),
        )?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_patient_record(
    original_patient_record_hash: ActionHash,
) -> ExternResult<ActionHash> {
    let details = get_details(
            original_patient_record_hash.clone(),
            GetOptions::default(),
        )?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("{pascal_entry_def_name} not found"))
            ),
        )?;
    let record = match details {
        Details::Record(details) => Ok(details.record),
        _ => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(String::from("Malformed get details response"))
                ),
            )
        }
    }?;
    let path = Path::from("all_records");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllRecords, None)?;
    for link in links {
        if let Some(hash) = link.target.into_action_hash() {
            if hash.eq(&original_patient_record_hash) {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    delete_entry(original_patient_record_hash)
}

#[hdk_extern]
pub fn get_all_deletes_for_patient_record(
    original_patient_record_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_patient_record_hash, GetOptions::default())?
    else {
        return Ok(None);
    };
    match details {
        Details::Entry(_) => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into())))
        }
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}

#[hdk_extern]
pub fn get_oldest_delete_for_patient_record(
    original_patient_record_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_patient_record(
        original_patient_record_hash,
    )? else {
        return Ok(None);
    };
    deletes
        .sort_by(|delete_a, delete_b| {
            delete_a.action().timestamp().cmp(&delete_b.action().timestamp())
        });
    Ok(deletes.first().cloned())
}

#[hdk_extern]
pub fn add_records_for_recorder(
    input: AddRecordForRecorderInput,
) -> ExternResult<()> {
    debug!("add patient record to recorder: {:?}", input);
    create_link(
        input.base_recorder.clone(),
        input.target_record_hash.clone(),
        LinkTypes::RecorderToRecords,
        (),
    )?;
    create_link(
        input.target_record_hash,
        input.base_recorder,
        LinkTypes::RecordsToRecorder,
        (),
    )?;
    Ok(())
}

#[hdk_extern]
pub fn get_records_for_recorder(
    recorder: AgentPubKey,
) -> ExternResult<Vec<Record>> {

    let links = get_links(recorder, LinkTypes::RecorderToRecords, None)?;

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

#[hdk_extern]
pub fn get_recorders_for_record(
    deliberation_hash: ActionHash,
) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(
        deliberation_hash,
        LinkTypes::RecordsToRecorder,
        None,
    )?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(
            EntryHash::try_from(link.target)
                .map_err(|_| {
                    wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))
                })
                .unwrap(),
        ))
        .collect();
    Ok(agents)
}

#[hdk_extern]
pub fn remove_records_for_recorder(
    input: RemoveRecordForRecorderInput,
) -> ExternResult<()> {
    let links = get_links(
        input.base_recorder.clone(),
        LinkTypes::RecorderToRecords,
        None,
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&input.target_record_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        input.target_record_hash.clone(),
        LinkTypes::RecordsToRecorder,
        None,
    )?;
    for link in links {
        if AgentPubKey::from(
                EntryHash::try_from(link.target.clone())
                    .map_err(|_| {
                        wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))
                    })
                    .unwrap(),
            )
            .eq(&input.base_recorder)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
