<script lang="ts">

  import { dConnected } from "../store";
  import { Link } from "svelte-routing";
  
  let connected;
  
  dConnected.subscribe((value) => {
    connected = value;
  })
  
  console.log(`[Register | Connected] : ${connected}`);

</script>

<profile-prompt>
  <div class="flex flex-col w-full border-opacity-50">
    <div class="grid h-20 card bg-base-100 rounded-box place-items-center">

      <div role="alert" class="alert alert-success place-items-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        <span>Agent Registered!</span>
      </div>

    </div>
    <div class="divider"></div>
    <div class="grid h-20 card bg-base-100 rounded-box place-items-center">
      <Link class="btn btn-ghost text-xl bg-accent" on:click={() => { dConnected.set(true); }} to="/">Go Dashboard</Link>
    </div>
  </div>
</profile-prompt>
