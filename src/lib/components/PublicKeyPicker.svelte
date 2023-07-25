<script lang="ts">
  import type { PublicKeyStore } from "$lib/public_key_store";

  export let publicKeyStore: PublicKeyStore;
  export let selection: string[] = [];

  function selectAll() {
    selection = Object.keys(publicKeyStore);
  }
</script>

<fieldset>
  {#each Object.keys(publicKeyStore) as recipientName}
    <label for={`recipient-${recipientName}`}>
      <input
        type="checkbox"
        bind:group={selection}
        id={`recipient-${recipientName}`}
        name={recipientName}
        value={recipientName}
      />
      {recipientName}
    </label>
  {/each}
</fieldset>

<button on:click={selectAll} type="button">Select all</button>
