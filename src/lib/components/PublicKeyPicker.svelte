<script lang="ts">
  import { sortedKeys } from "$lib/public_key_store";
  import { get } from "svelte/store";
  interface Props {
    selection?: string[];
  }

  let { selection = $bindable([]) }: Props = $props();

  function selectAll() {
    selection = [...get(sortedKeys)];
  }
</script>

<fieldset>
  {#each $sortedKeys as recipientName}
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

<button onclick={selectAll} type="button">Select all</button>
