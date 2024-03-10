use hdi::prelude::*;

#[derive(Debug)]
enum NoHeaKoe {
    CapeReinga,
    Russell,
    Hokianga,
    Whāngarei,
    Dargaville,
    Auckland,
    Waiheke,
    Thames,
    Huntly,
    Ngāruawāhia,
    Morrinsville,
    Hamilton,
    Tauranga,
    Cambridge,
    Tokoroa,
    Rotorua,
    Whakatāne,
    Gisborne,
    TolagaBay,
    Wairoa,
    Taupō,
    NewPlymouth,
    Hāwera,
    Wanganui,
    Napier,
    Hastings,
    Taranaki,
    Feilding,
    PalmerstonNorth,
    Dannevirke,
    Levin,
    Masterton,
    Wellington,
    Featherston,
    UpperHutt,
    LowerHutt,
    Porirua,
}

#[derive(Debug)]
enum Maunga {
    Tutamoe,
    Tararua,
    Moehau,
    CastleRock,
    Rangitoto,
    Wellington,
    Eden,
    Hobson,
    OneTreeHill,
    Albert,
    Roskill,
    Māngere,
    BluffHill,
    TeToiokawharu,
    Kohukohunui,
    TeAroha,
    Maunganui,
    Karioi,
    Pirongia,
    Kakepuku,
    Manguatautari,
    Ngongotahā,
    TaraweraDome,
    Edgecumbe,
    Pureora,
    Tauhara,
    Tongariro,
    TeHeuheu,
    Ruapehu,
    Pouakai,
    Taranaki,
    TeMata,
    Kahuranaki,
    Hikurangi,
    Mangaweka,
    Rimutaka,
    ColonialKnob,
    Kaukau,
    TeAhumairangi,
    Victoria,
}

#[derive(Debug)]
enum Moana {
    NinetyMile,
    Doubtless,
    Rainbow,
    Whangarei,
    Kaiiwi,
    Kaipara,
    Omaha,
    Orewa,
    Long,
    Hauraki,
    Onetangi,
    Muriwai,
    Pupuke,
    Takapuna,
    Watematā,
    Cheltenham,
    Mission,
    Piha,
    Karekare,
    Manukau,
    Hunua,
    Cathedral,
    HotWater,
    Pauanui,
    Whangamata,
    Owharoa,
    Waikato,
    Kaituna,
    Waitoa,
    Ngarunui,
    BridalVeil,
    Rotoiti,
    Rotorua,
    Tarawera,
    Ohope,
    Marokopa,
    Whakatāne,
    Wairere,
    Omaru,
    Ongarue,
    Huka,
    Waikaremoana,
    Rere,
    Ruakituri,
    MountDamper,
    Whanganui,
    Waihi,
    Taupō,
    Rotopounamu,
    Emerald,
    Taranaki,
    Waipunga,
    Shine,
    Waiau,
    Tutaekuri,
    Patea,
    Moawhango,
    Turakina,
    Whenuakura,
    Tukituki,
    Manawatu,
    Hutt,
    Ruamahanga,
}

#[derive(Debug)]
enum Indigenous {
    Maori,
    Other,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct PatientRecord {
    pub content: String,
    pub resource_type: String,
    pub date_visited: Timestamp,
    pub whanau: String,
    pub ingoa: String,
    pub no_hea_koe: String,
    pub maunga: String,
    pub moana: String,
}

pub fn validate_create_patient_record(
    _action: EntryCreationAction,
    _patient_record: PatientRecord,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_patient_record(
    _action: Update,
    _patient_record: PatientRecord,
    _original_action: EntryCreationAction,
    _original_patient_record: PatientRecord,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_patient_record(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_patient_record: PatientRecord,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_patient_record_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = base_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _patient_record: crate::PatientRecord = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _patient_record: crate::PatientRecord = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_patient_record_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("PatientRecordUpdates links cannot be deleted"),
        ),
    )
}

pub fn validate_create_link_all_records(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _patient_record: crate::PatientRecord = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_records(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_records_by_recorder(
    _action: CreateLink, _base_address: AnyLinkableHash, target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _patient_record: crate::PatientRecord = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_records_by_recorder(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_recorder_to_records(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _patient_record: crate::PatientRecord = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_recorder_to_records(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_records_to_recorder(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_records_to_recorder(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(base_address)
        .map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))
        .unwrap();
    let record = must_get_valid_record(action_hash)?;
    let _patient_record: crate::PatientRecord = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
