use hdk::prelude::*;
use patient_records_integrity::*;

#[hdk_extern]
pub fn get_all_records(_: ()) -> ExternResult<Vec<Link>> {
    let path = Path::from("all_records");
    get_links(path.path_entry_hash()?, LinkTypes::AllRecords, None)
}
