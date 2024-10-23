<script lang="ts">
  import { delay } from "$lib/delay";
  import ErrorMessage from "./ErrorMessage.svelte";

  interface Props {
    text: string;
    label: string;
  }

  let { text, label }: Props = $props();

  let copied = $state(false);
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

<button type="button" onclick={() => copyToClipboard(text)}>
  {#if copied}
    Copied
  {:else}
    {label}
  {/if}
</button>

{#if error != null}
  <ErrorMessage {error} />
{/if}
