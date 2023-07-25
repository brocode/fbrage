<script lang="ts">
  import { delay } from "$lib/delay";
  import ErrorMessage from "./ErrorMessage.svelte";

  export let text: string;
  export let label: string;

  let copied = false;
  let error: string | null = null;

  async function copyToClipboard(text: string) {
    copied = false;
    try {
      await navigator.clipboard.writeText(text);
      copied = true;
      await delay(1000);
      copied = false;
    } catch (err) {
      console.log("Failed to copy text: ", err);
    }
  }
</script>

<button type="button" on:click={() => copyToClipboard(text)}>
  {#if copied}
    Copied
  {:else}
    {label}
  {/if}
</button>

{#if error != null}
  <ErrorMessage {error} />
{/if}
