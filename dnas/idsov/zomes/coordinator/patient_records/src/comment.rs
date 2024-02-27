use hdk::prelude::*;
use patient_records_integrity::*;
#[hdk_extern]
pub fn create_comment(comment: Comment) -> ExternResult<Record> {
    let comment_hash = create_entry(&EntryTypes::Comment(comment.clone()))?;
    create_link(
        comment.patient_record_hash.clone(),
        comment_hash.clone(),
        LinkTypes::PatientRecordToComments,
        (),
    )?;
    let record = get(comment_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Comment"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_latest_comment(
    original_comment_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_comment_hash.clone(),
        LinkTypes::CommentUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_comment_hash = match latest_link {
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
        None => original_comment_hash.clone(),
    };
    get(latest_comment_hash, GetOptions::default())
}
#[hdk_extern]
pub fn get_original_comment(
    original_comment_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_comment_hash, GetOptions::default())? else {
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
pub fn get_all_revisions_for_comment(
    original_comment_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_comment(original_comment_hash.clone())?
    else {
        return Ok(vec![]);
    };
    let links = get_links(
        original_comment_hash.clone(),
        LinkTypes::CommentUpdates,
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
pub struct UpdateCommentInput {
    pub original_comment_hash: ActionHash,
    pub previous_comment_hash: ActionHash,
    pub updated_comment: Comment,
}
#[hdk_extern]
pub fn update_comment(input: UpdateCommentInput) -> ExternResult<Record> {
    let updated_comment_hash = update_entry(
        input.previous_comment_hash.clone(),
        &input.updated_comment,
    )?;
    create_link(
        input.original_comment_hash.clone(),
        updated_comment_hash.clone(),
        LinkTypes::CommentUpdates,
        (),
    )?;
    let record = get(updated_comment_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Comment"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_comment(original_comment_hash: ActionHash) -> ExternResult<ActionHash> {
    let details = get_details(original_comment_hash.clone(), GetOptions::default())?
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
    let entry = record
        .entry()
        .as_option()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Comment record has no entry"))
            ),
        )?;
    let comment = Comment::try_from(entry)?;
    let links = get_links(
        comment.patient_record_hash.clone(),
        LinkTypes::PatientRecordToComments,
        None,
    )?;
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if action_hash.eq(&original_comment_hash) {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    delete_entry(original_comment_hash)
}
#[hdk_extern]
pub fn get_all_deletes_for_comment(
    original_comment_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_comment_hash, GetOptions::default())? else {
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
pub fn get_oldest_delete_for_comment(
    original_comment_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_comment(original_comment_hash)? else {
        return Ok(None);
    };
    deletes
        .sort_by(|delete_a, delete_b| {
            delete_a.action().timestamp().cmp(&delete_b.action().timestamp())
        });
    Ok(deletes.first().cloned())
}
#[hdk_extern]
pub fn get_comments_for_patient_record(
    patient_record_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(patient_record_hash, LinkTypes::PatientRecordToComments, None)
}
#[hdk_extern]
pub fn get_deleted_comments_for_patient_record(
    patient_record_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        patient_record_hash,
        LinkTypes::PatientRecordToComments,
        None,
    )?;
    Ok(
        details
            .into_inner()
            .into_iter()
            .filter(|(_link, deletes)| deletes.len() > 0)
            .collect(),
    )
}
