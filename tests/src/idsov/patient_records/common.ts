import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function samplePatientRecord(cell: CallableCell, partialPatientRecord = {}) {
    return {
        ...{
	  content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  resource_type: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  date_visited: 1674053334548000,
        },
        ...partialPatientRecord
    };
}

export async function createPatientRecord(cell: CallableCell, patientRecord = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "patient_records",
      fn_name: "create_patient_record",
      payload: patientRecord || await samplePatientRecord(cell),
    });
}



export async function sampleComment(cell: CallableCell, partialComment = {}) {
    return {
        ...{
	  content_comment: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
          patient_record_hash: (await createPatientRecord(cell)).signed_action.hashed.hash,
        },
        ...partialComment
    };
}

export async function createComment(cell: CallableCell, comment = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "patient_records",
      fn_name: "create_comment",
      payload: comment || await sampleComment(cell),
    });
}

