<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { Comment } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalCommentHash!: ActionHash;

export let currentRecord!: Record;
let currentComment: Comment = decode((currentRecord.entry as any).Present.entry) as Comment;

let contentComment: string | undefined = currentComment.content_comment;

let errorSnackbar: Snackbar;

$: contentComment;
$: isCommentValid = true && contentComment !== '';

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditComment element`);
  }
  if (originalCommentHash === undefined) {
    throw new Error(`The originalCommentHash input is required for the EditComment element`);
  }
});

async function updateComment() {

  const comment: Comment = {
    content_comment: contentComment!,
    patient_record_hash: currentComment.patient_record_hash,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'idsov',
      zome_name: 'patient_records',
      fn_name: 'update_comment',
      payload: {
        original_comment_hash: originalCommentHash,
        previous_comment_hash: currentRecord.signed_action.hashed.hash,
        updated_comment: comment
      }
    });

    dispatch('comment-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the comment: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit Comment</span>

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Content Comment" value={ contentComment } on:input={e => { contentComment = e.target.value;} } required></mwc-textarea>
  </div>


  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button
      raised
      label="Save"
      disabled={!isCommentValid}
      on:click={() => updateComment()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
