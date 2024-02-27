<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Comment } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let patientRecordHash!: ActionHash;


let contentComment: string = '';

let errorSnackbar: Snackbar;

$: contentComment, patientRecordHash;
$: isCommentValid = true && contentComment !== '';

onMount(() => {
  if (patientRecordHash === undefined) {
    throw new Error(`The patientRecordHash input is required for the CreateComment element`);
  }
});

async function createComment() {  
  const commentEntry: Comment = { 
    content_comment: contentComment!,
    patient_record_hash: patientRecordHash!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'idsov',
      zome_name: 'patient_records',
      fn_name: 'create_comment',
      payload: commentEntry,
    });
    dispatch('comment-created', { commentHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the comment: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Comment</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Content Comment" value={ contentComment } on:input={e => { contentComment = e.target.value;} } required></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Create Comment"
    disabled={!isCommentValid}
    on:click={() => createComment()}
  ></mwc-button>
</div>
