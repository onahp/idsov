
<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Link, ActionHash, EntryHash, AppAgentClient, Record, AgentPubKey, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Comment } from './types';
import CommentDetail from './CommentDetail.svelte';

export let patientRecordHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let links: Array<Link> | undefined;

let loading = true;
let error: any = undefined;

$: links, loading, error;

onMount(async () => {
  if (patientRecordHash === undefined) {
    throw new Error(`The patientRecordHash input is required for the CommentsForPatientRecord element`);
  }

  try {
    links = await client.callZome({
      cap_secret: null,
      role_name: 'idsov',
      zome_name: 'patient_records',
      fn_name: 'get_comments_for_patient_record',
      payload: patientRecordHash
    });
  } catch (e) {
    error = e;
  }
  loading = false;
});

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching comments: ${error.data.data}.</span>
{:else if links.length === 0}
<span>No comments found for this patient record.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each links as link}
    <div style="margin-bottom: 8px;">
      <CommentDetail commentHash={link.target}></CommentDetail>
    </div>
  {/each}
</div>
{/if}
