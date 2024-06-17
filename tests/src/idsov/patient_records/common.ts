import { CallableCell } from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
  fakeDnaHash,
} from "@holochain/client";


export async function samplePatientRecord(cell: CallableCell, partialPatientRecord = {}) {
    return {
        ...{
	  content: "This is a patient record created by Ioane Mataki",
	  resource_type: "The resource type is a visitation",
	  date_visited: 1674053334548000,
	  whanau: "Mataki",
	  ingoa: "Ioane",
	  no_hea_koe: "Region Area",
	  maunga: "Mountain Place",
	  moana: "Body of Water",
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
	  content_comment: "This is some random comment for the patient record",
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
